use crate::contracts::department_contracts::DepartmentStorage;
use crate::contracts_impl::{create_schema, open_pool, row_i32, row_uuid};
use crate::entities::Department;
use async_sqlite::Pool;
use rusqlite::{OptionalExtension, params};
use time::UtcDateTime;
use uuid::Uuid;

/// Колонки таблицы `department` в одном порядке для всех SELECT.
const SELECT_COLUMNS: &str =
    "id, name, Head_of_Department, number, volume, created_at, updated_at, deleted_at";

/// Хранилище подразделений в SQLite.
pub struct DepartmentSqliteStorage {
    connection: Pool,
}

/// Превращает строку результата запроса в сущность.
fn map_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Department> {
    Ok(Department {
        id: row_uuid(row, 0)?,
        name: row.get(1)?,
        head_of_department: row.get(2)?,
        number: row_i32(row, 3)?,
        volume: row_i32(row, 4)?,
        created_at: row.get(5)?,
        updated_at: row.get(6)?,
        deleted_at: row.get(7)?,
    })
}

impl DepartmentSqliteStorage {
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

impl DepartmentStorage for DepartmentSqliteStorage {
    async fn add(&self, department: &Department) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
            INSERT INTO department (id, name, Head_of_Department, number, volume,
                                    created_at, updated_at, deleted_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
        ";

        // Замыкание исполняется на рабочем потоке пула, поэтому данные передаются внутрь по значению.
        let row = (
            department.id.to_string(),
            department.name.clone(),
            department.head_of_department.clone(),
            department.number,
            department.volume,
            department.created_at,
            department.updated_at,
            department.deleted_at,
        );
        let id: String = row.0.clone();

        let affected: usize = self
            .connection
            .conn(move |connection| {
                connection.execute(
                    QUERY,
                    params![row.0, row.1, row.2, row.3, row.4, row.5, row.6, row.7],
                )
            })
            .await?;

        if affected != 1 {
            return Err(format!("Подразделение {id} не добавлено").into());
        }
        Ok(())
    }

    async fn get(&self, uuid: Uuid) -> Result<Option<Department>, Box<dyn std::error::Error>> {
        let query: String =
            format!("SELECT {SELECT_COLUMNS} FROM department WHERE id = ?1 AND deleted_at IS NULL");
        let id: String = uuid.to_string();

        let department: Option<Department> = self
            .connection
            .conn(move |connection| {
                connection
                    .query_row(&query, params![id], map_row)
                    .optional()
            })
            .await?;

        Ok(department)
    }

    async fn list(&self) -> Result<Vec<Department>, Box<dyn std::error::Error>> {
        let query: String = format!(
            "SELECT {SELECT_COLUMNS} FROM department
             WHERE deleted_at IS NULL
             ORDER BY created_at"
        );

        let departments: Vec<Department> = self
            .connection
            .conn(move |connection| {
                let mut statement = connection.prepare(&query)?;
                let rows = statement
                    .query_map(params![], map_row)?
                    .collect::<rusqlite::Result<Vec<Department>>>()?;
                Ok(rows)
            })
            .await?;

        Ok(departments)
    }

    async fn update(&self, department: &Department) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
            UPDATE department
            SET name = ?1,
                Head_of_Department = ?2,
                number = ?3,
                volume = ?4,
                updated_at = ?5
            WHERE id = ?6 AND deleted_at IS NULL
        ";

        let row = (
            department.name.clone(),
            department.head_of_department.clone(),
            department.number,
            department.volume,
            UtcDateTime::now(),
            department.id.to_string(),
        );
        let id: String = row.5.clone();

        let affected: usize = self
            .connection
            .conn(move |connection| {
                connection.execute(QUERY, params![row.0, row.1, row.2, row.3, row.4, row.5])
            })
            .await?;

        if affected == 0 {
            return Err(format!("Подразделение {id} не найдено").into());
        }
        Ok(())
    }

    async fn remove(&self, uuid: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "DELETE FROM department WHERE id = ?1";

        let id: String = uuid.to_string();
        let query_id: String = id.clone();

        let affected: usize = self
            .connection
            .conn(move |connection| connection.execute(QUERY, params![query_id]))
            .await?;

        if affected == 0 {
            return Err(format!("Подразделение {id} не удалено: запись не найдена").into());
        }
        Ok(())
    }
}
