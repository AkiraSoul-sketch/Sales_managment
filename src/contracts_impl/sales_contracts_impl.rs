use crate::contracts::sales_contract::SalesStorage;
use crate::contracts_impl::{create_schema, open_pool, row_uuid};
use crate::entities::Sales;
use async_sqlite::Pool;
use rusqlite::types::Value;
use rusqlite::{OptionalExtension, params, params_from_iter};
use time::UtcDateTime;
use uuid::Uuid;

/// Колонки таблицы `sales` в одном порядке для всех SELECT.
const SELECT_COLUMNS: &str = "id, unit_id, department_id, sail_date, sold_items_count";

/// Хранилище продаж в SQLite.
pub struct SalesSqliteStorage {
    connection: Pool,
}

/// Превращает строку результата запроса в сущность.
fn map_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Sales> {
    Ok(Sales {
        id: row_uuid(row, 0)?,
        unit_id: row_uuid(row, 1)?,
        department_id: row_uuid(row, 2)?,
        sail_date: row.get(3)?,
        sold_items_count: row.get(4)?,
    })
}

impl SalesSqliteStorage {
    /// Открывает собственное соединение к базе и создаёт таблицы при необходимости.
    pub async fn new(database_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let connection: Pool = open_pool(database_url).await?;
        create_schema(&connection).await?;
        Ok(Self::from_pool(connection))
    }

    /// Хранилище поверх уже готового пула: одно соединение можно разделить между хранилищами.
    pub const fn from_pool(connection: Pool) -> Self {
        Self { connection }
    }

    /// Закрывает пул соединений.
    pub async fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.connection.close().await?;
        Ok(())
    }
}

impl SalesStorage for SalesSqliteStorage {
    async fn add(&self, sales: &Sales) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
            INSERT INTO sales (id, unit_id, department_id, sail_date, sold_items_count)
            VALUES (?1, ?2, ?3, ?4, ?5)
        ";

        // Замыкание исполняется на рабочем потоке пула, поэтому данные передаются внутрь по значению.
        let row = (
            sales.id.to_string(),
            sales.unit_id.to_string(),
            sales.department_id.to_string(),
            sales.sail_date,
            sales.sold_items_count,
        );
        let id: String = row.0.clone();

        let affected: usize = self
            .connection
            .conn(move |connection| {
                connection.execute(QUERY, params![row.0, row.1, row.2, row.3, row.4])
            })
            .await?;

        if affected != 1 {
            return Err(format!("Продажа {id} не добавлена").into());
        }
        Ok(())
    }

    async fn add_many(&self, sales: &[Sales]) -> Result<(), Box<dyn std::error::Error>> {
        if sales.is_empty() {
            return Ok(());
        }

        const QUERY: &str = "
            INSERT INTO sales (id, unit_id, department_id, sail_date, sold_items_count)
            VALUES (?1, ?2, ?3, ?4, ?5)
        ";

        let rows: Vec<(String, String, String, UtcDateTime, i32)> = sales
            .iter()
            .map(|sale| {
                (
                    sale.id.to_string(),
                    sale.unit_id.to_string(),
                    sale.department_id.to_string(),
                    sale.sail_date,
                    sale.sold_items_count,
                )
            })
            .collect();
        let expected: usize = rows.len();

        // Одна транзакция на всю пачку: при ошибке любой записи откатывается вся партия.
        let inserted: usize = self
            .connection
            .conn_mut(move |connection| {
                let transaction = connection.transaction()?;
                let mut inserted = 0_usize;
                for row in rows {
                    inserted +=
                        transaction.execute(QUERY, params![row.0, row.1, row.2, row.3, row.4])?;
                }
                transaction.commit()?;
                Ok(inserted)
            })
            .await?;

        if inserted != expected {
            return Err(format!("Добавлено {inserted} продаж из {expected}").into());
        }
        Ok(())
    }

    async fn get(&self, uuid: Uuid) -> Result<Option<Sales>, Box<dyn std::error::Error>> {
        let query: String = format!("SELECT {SELECT_COLUMNS} FROM sales WHERE id = ?1");
        let id: String = uuid.to_string();

        let sales: Option<Sales> = self
            .connection
            .conn(move |connection| {
                connection
                    .query_row(&query, params![id], map_row)
                    .optional()
            })
            .await?;

        Ok(sales)
    }

    async fn list(&self) -> Result<Vec<Sales>, Box<dyn std::error::Error>> {
        let query: String = format!(
            "SELECT {SELECT_COLUMNS} FROM sales
             ORDER BY sail_date"
        );

        self.select_many(query, Vec::new()).await
    }

    async fn list_by_department(
        &self,
        department_id: Uuid,
    ) -> Result<Vec<Sales>, Box<dyn std::error::Error>> {
        let query: String = format!(
            "SELECT {SELECT_COLUMNS} FROM sales
             WHERE department_id = ?1
             ORDER BY sail_date"
        );

        self.select_many(query, vec![Value::Text(department_id.to_string())])
            .await
    }

    async fn update(&self, sales: &Sales) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
            UPDATE sales
            SET unit_id = ?1,
                department_id = ?2,
                sail_date = ?3,
                sold_items_count = ?4
            WHERE id = ?5
        ";

        let row = (
            sales.unit_id.to_string(),
            sales.department_id.to_string(),
            sales.sail_date,
            sales.sold_items_count,
            sales.id.to_string(),
        );
        let id: String = row.4.clone();

        let affected: usize = self
            .connection
            .conn(move |connection| {
                connection.execute(QUERY, params![row.0, row.1, row.2, row.3, row.4])
            })
            .await?;

        if affected == 0 {
            return Err(format!("Продажа {id} не найдена").into());
        }
        Ok(())
    }

    async fn remove(&self, uuid: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "DELETE FROM sales WHERE id = ?1";

        let id: String = uuid.to_string();
        let query_id: String = id.clone();

        let affected: usize = self
            .connection
            .conn(move |connection| connection.execute(QUERY, params![query_id]))
            .await?;

        if affected == 0 {
            return Err(format!("Продажа {id} не удалена: запись не найдена").into());
        }
        Ok(())
    }
}

impl SalesSqliteStorage {
    /// Общий помощник для SELECT, возвращающих несколько строк.
    async fn select_many(
        &self,
        query: String,
        parameters: Vec<Value>,
    ) -> Result<Vec<Sales>, Box<dyn std::error::Error>> {
        let sales: Vec<Sales> = self
            .connection
            .conn(move |connection| {
                let mut statement = connection.prepare(&query)?;
                let rows = statement
                    .query_map(params_from_iter(parameters), map_row)?
                    .collect::<rusqlite::Result<Vec<Sales>>>()?;
                Ok(rows)
            })
            .await?;

        Ok(sales)
    }
}
