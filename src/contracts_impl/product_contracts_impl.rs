use crate::contracts::product_contracts::ProductStorage;
use crate::contracts_impl::{create_schema, open_pool, row_uuid};
use crate::entities::Product;
use async_sqlite::Pool;
use rusqlite::{OptionalExtension, params};
use time::UtcDateTime;
use uuid::Uuid;

/// Колонки таблицы `product` в одном порядке для всех SELECT.
const SELECT_COLUMNS: &str = "item_number, name, unit, price, created_at, updated_at, deleted_at";

/// Хранилище товаров в SQLite.
pub struct ProductSqliteStorage {
    connection: Pool,
}

/// Превращает строку результата запроса в сущность.
fn map_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Product> {
    Ok(Product {
        item_number: row_uuid(row, 0)?,
        name: row.get(1)?,
        unit: row.get(2)?,
        price: row.get(3)?,
        created_at: row.get(4)?,
        updated_at: row.get(5)?,
        deleted_at: row.get(6)?,
    })
}

impl ProductSqliteStorage {
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

impl ProductStorage for ProductSqliteStorage {
    async fn add(&self, product: &Product) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
            INSERT INTO product (item_number, name, unit, price, created_at, updated_at, deleted_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        ";

        // Замыкание исполняется на рабочем потоке пула, поэтому данные передаются внутрь по значению.
        let row = (
            product.item_number.to_string(),
            product.name.clone(),
            product.unit,
            product.price,
            product.created_at,
            product.updated_at,
            product.deleted_at,
        );
        let item_number: String = row.0.clone();

        let affected: usize = self
            .connection
            .conn(move |connection| {
                connection.execute(
                    QUERY,
                    params![row.0, row.1, row.2, row.3, row.4, row.5, row.6],
                )
            })
            .await?;

        if affected != 1 {
            return Err(format!("Товар {item_number} не добавлен").into());
        }
        Ok(())
    }

    async fn get(&self, uuid: Uuid) -> Result<Option<Product>, Box<dyn std::error::Error>> {
        let query: String = format!(
            "SELECT {SELECT_COLUMNS} FROM product WHERE item_number = ?1 AND deleted_at IS NULL"
        );
        let item_number: String = uuid.to_string();

        let product: Option<Product> = self
            .connection
            .conn(move |connection| {
                connection
                    .query_row(&query, params![item_number], map_row)
                    .optional()
            })
            .await?;

        Ok(product)
    }

    async fn list(&self) -> Result<Vec<Product>, Box<dyn std::error::Error>> {
        let query: String = format!(
            "SELECT {SELECT_COLUMNS} FROM product
             WHERE deleted_at IS NULL
             ORDER BY created_at"
        );

        let products: Vec<Product> = self
            .connection
            .conn(move |connection| {
                let mut statement = connection.prepare(&query)?;
                let rows = statement
                    .query_map(params![], map_row)?
                    .collect::<rusqlite::Result<Vec<Product>>>()?;
                Ok(rows)
            })
            .await?;

        Ok(products)
    }

    async fn update(&self, product: &Product) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
            UPDATE product
            SET name = ?1,
                unit = ?2,
                price = ?3,
                updated_at = ?4
            WHERE item_number = ?5 AND deleted_at IS NULL
        ";

        let row = (
            product.name.clone(),
            product.unit,
            product.price,
            UtcDateTime::now(),
            product.item_number.to_string(),
        );
        let item_number: String = row.4.clone();

        let affected: usize = self
            .connection
            .conn(move |connection| {
                connection.execute(QUERY, params![row.0, row.1, row.2, row.3, row.4])
            })
            .await?;

        if affected == 0 {
            return Err(format!("Товар {item_number} не найден").into());
        }
        Ok(())
    }

    async fn remove(&self, uuid: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "DELETE FROM product WHERE item_number = ?1";

        let item_number: String = uuid.to_string();
        let query_item_number: String = item_number.clone();

        let affected: usize = self
            .connection
            .conn(move |connection| connection.execute(QUERY, params![query_item_number]))
            .await?;

        if affected == 0 {
            return Err(format!("Товар {item_number} не удалён: запись не найдена").into());
        }
        Ok(())
    }
}
