use std::path::PathBuf;

use crate::contracts::department_contracts::DepartmentStorage;
use crate::entities::Department;
use async_sqlite::{Client, Pool, PoolBuilder};
use rusqlite::types::Value;
use rusqlite::{OptionalExtension, params_from_iter};
use time::format_description::FormatItem;
use time::macros::format_description;

/// Колонки дат в SQLite — текст вида "YYYY-MM-DD HH:MM:SS".
/// В таком формате их читает `row.get::<_, time::UtcDateTime>()`.
const DATETIME_ENCODING: &[FormatItem<'_>] =
    format_description!(version = 2, "[year]-[month]-[day] [hour]:[minute]:[second]");

pub struct DepartmentSqliteStorage {
    connection: Client,
}

impl DepartmentSqliteStorage {
    pub async fn new(database_url: &str) -> Self {
        let path_buf: PathBuf = PathBuf::from(database_url);
        let builder: PoolBuilder = PoolBuilder::new()
            .path(path_buf)
            .journal_mode(async_sqlite::JournalMode::Wal)
            .num_conns(10);
        let connection: Pool = builder.open().await.expect("Database creation error");
        Self { connection }
    }
}

impl DepartmentStorage for DepartmentSqliteStorage {
    async fn add(&self, department: &Department) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
        INSERT INTO department (id, name, Head_of_Department, volume, number, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)";
        let mut parameters: Vec<Value> = Vec::with_capacity(6);
        parameters.push(department.id.into());
        parameters.push(department.name.clone().into());
        parameters.push(department.head_of_department.clone().into());
        parameters.push(department.volume.into());
        parameters.push(department.number.into());
        parameters.push(department.created_at.format(&DATETIME_ENCODING)?.into());

        self.connection
            .conn(move |connection| {
                let affected = connection.execute(QUERY, params_from_iter(parameters))?;
                Ok(affected)
            })
            .await?;
        Ok(())
    }

    async fn get(
        &self,
        uuid: uuid::Uuid,
    ) -> Result<Option<Department>, Box<dyn std::error::Error>> {
        const QUERY: &str = "
              SELECT id, name, Head_of_Department, volume, number, created_at
              FROM department
              WHERE id = ?1
        ";
        let parameters: Vec<Value> = vec![uuid.into()];
        let department: Option<Department> = self
            .connection
            .conn(move |connection| {
                connection
                    .query_row(QUERY, params_from_iter(parameters), |row| {
                        Ok(Department {
                            id: row.get(0)?,
                            name: row.get(1)?,
                            head_of_department: row.get(2)?,
                            volume: row.get(3)?,
                            number: row.get(4)?,
                            created_at: row.get(5)?,
                            updated_at: None,
                            finished_at: None,
                            deleted_at: None,
                        })
                    })
                    .optional()
            })
            .await?;
        Ok(department)
    }

    async fn remove(&self, uuid: uuid::Uuid) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "DELETE FROM department WHERE id = ?1";
        let parameters: Vec<Value> = vec![uuid.into()];
        self.connection
            .conn(move |connection| {
                let affected = connection.execute(QUERY, params_from_iter(parameters))?;
                Ok(affected)
            })
            .await?;
        Ok(())
    }

    async fn update(&self, department: &Department) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
 UPDATE department
 SET name = ?1, Head_of_Department = ?2, volume = ?3, number = ?4
 WHERE id = ?5
 ";
        let mut parameters: Vec<Value> = Vec::with_capacity(5);
        parameters.push(department.name.clone().into());
        parameters.push(department.head_of_department.clone().into());
        parameters.push(department.volume.into());
        parameters.push(department.number.into());
        parameters.push(department.id.into());
        self.connection
            .conn(move |connection| {
                let affected = connection.execute(QUERY, params_from_iter(parameters))?;
                Ok(affected)
            })
            .await?;
        Ok(())
    }
}
