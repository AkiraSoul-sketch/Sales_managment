use std::path::PathBuf;

use crate::contracts::sales_contract::SalesStorage;
use crate::entities::Sales;
use async_sqlite::{Client, Pool, PoolBuilder};
use rusqlite::types::Value;
use rusqlite::{OptionalExtension, params_from_iter};
use time::format_description::FormatItem;
use time::macros::format_description;

/// Колонки дат в SQLite — текст вида "YYYY-MM-DD HH:MM:SS".
/// В таком формате их читает `row.get::<_, time::UtcDateTime>()`.
const DATETIME_ENCODING: &[FormatItem<'_>] =
    format_description!(version = 2, "[year]-[month]-[day] [hour]:[minute]:[second]");

pub struct SalesSqliteStorage {
    connection: Client,
}

impl SalesSqliteStorage {
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

impl SalesStorage for SalesSqliteStorage {
    async fn add(&self, sales: &Sales) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
        INSERT INTO sales (id, unit_id, department_id, sail_date, sold_items_count) VALUES (?1, ?2, ?3, ?4, ?5)";
        let mut parameters: Vec<Value> = Vec::with_capacity(5);
        parameters.push(sales.id.into());
        parameters.push(sales.unit_id.into());
        parameters.push(sales.department_id.into());
        parameters.push(sales.sail_date.format(&DATETIME_ENCODING)?.into());
        parameters.push(sales.sold_items_count.into());

        self.connection
            .conn(move |connection| {
                let affected = connection.execute(QUERY, params_from_iter(parameters))?;
                Ok(affected)
            })
            .await?;
        Ok(())
    }

    async fn get(&self, uuid: uuid::Uuid) -> Result<Option<Sales>, Box<dyn std::error::Error>> {
        const QUERY: &str = "
              SELECT id, unit_id, department_id, sail_date, sold_items_count
              FROM sales
              WHERE id = ?1
        ";
        let parameters: Vec<Value> = vec![uuid.into()];
        let sales: Option<Sales> = self
            .connection
            .conn(move |connection| {
                connection
                    .query_row(QUERY, params_from_iter(parameters), |row| {
                        Ok(Sales {
                            id: row.get(0)?,
                            unit_id: row.get(1)?,
                            department_id: row.get(2)?,
                            sail_date: row.get(3)?,
                            sold_items_count: row.get(4)?,
                        })
                    })
                    .optional()
            })
            .await?;
        Ok(sales)
    }

    async fn remove(&self, uuid: uuid::Uuid) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "DELETE FROM sales WHERE id = ?1";
        let parameters: Vec<Value> = vec![uuid.into()];
        self.connection
            .conn(move |connection| {
                let affected = connection.execute(QUERY, params_from_iter(parameters))?;
                Ok(affected)
            })
            .await?;
        Ok(())
    }

    async fn update(&self, sales: &Sales) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
 UPDATE sales
 SET unit_id = ?1, department_id = ?2, sail_date = ?3, sold_items_count = ?4
 WHERE id = ?5
 ";
        let mut parameters: Vec<Value> = Vec::with_capacity(5);
        parameters.push(sales.unit_id.into());
        parameters.push(sales.department_id.into());
        parameters.push(sales.sail_date.format(&DATETIME_ENCODING)?.into());
        parameters.push(sales.sold_items_count.into());
        parameters.push(sales.id.into());
        self.connection
            .conn(move |connection| {
                let affected = connection.execute(QUERY, params_from_iter(parameters))?;
                Ok(affected)
            })
            .await?;
        Ok(())
    }
}
