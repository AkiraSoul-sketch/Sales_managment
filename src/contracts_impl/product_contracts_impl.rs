use std::path::PathBuf;

use crate::contracts::product_contracts::ProductStorage;
use crate::entities::Product;
use async_sqlite::{Client, Pool, PoolBuilder};
use rusqlite::types::Value;
use rusqlite::{OptionalExtension, params_from_iter};
use time::format_description::FormatItem;
use time::macros::format_description;

/// Колонки дат в SQLite — текст вида "YYYY-MM-DD HH:MM:SS".
/// В таком формате их читает `row.get::<_, time::UtcDateTime>()`.
const DATETIME_ENCODING: &[FormatItem<'_>] =
    format_description!(version = 2, "[year]-[month]-[day] [hour]:[minute]:[second]");

pub struct ProductSqliteStorage {
    connection: Client,
}

impl ProductSqliteStorage {
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

impl ProductStorage for ProductSqliteStorage {
    async fn add(&self, product: &Product) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
        INSERT INTO product (item_number, name, unit, price, created_at) VALUES (?1, ?2, ?3, ?4, ?5)";
        let mut parameters: Vec<Value> = Vec::with_capacity(5);
        parameters.push(product.item_number.into());
        parameters.push(product.name.clone().into());
        parameters.push(product.unit.into());
        parameters.push(product.price.into());
        parameters.push(product.created_at.format(&DATETIME_ENCODING)?.into());

        self.connection
            .conn(move |connection| {
                let affected = connection.execute(QUERY, params_from_iter(parameters))?;
                Ok(affected)
            })
            .await?;
        Ok(())
    }

    async fn get(&self, uuid: uuid::Uuid) -> Result<Option<Product>, Box<dyn std::error::Error>> {
        const QUERY: &str = "
              SELECT item_number, name, unit, price, created_at
              FROM product
              WHERE item_number = ?1
        ";
        let parameters: Vec<Value> = vec![uuid.into()];
        let product: Option<Product> = self
            .connection
            .conn(move |connection| {
                connection
                    .query_row(QUERY, params_from_iter(parameters), |row| {
                        Ok(Product {
                            item_number: row.get(0)?,
                            name: row.get(1)?,
                            unit: row.get(2)?,
                            price: row.get(3)?,
                            created_at: row.get(4)?,
                            updated_at: None,
                            deleted_at: None,
                        })
                    })
                    .optional()
            })
            .await?;
        Ok(product)
    }

    async fn remove(&self, uuid: uuid::Uuid) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "DELETE FROM product WHERE item_number = ?1";
        let parameters: Vec<Value> = vec![uuid.into()];
        self.connection
            .conn(move |connection| {
                let affected = connection.execute(QUERY, params_from_iter(parameters))?;
                Ok(affected)
            })
            .await?;
        Ok(())
    }

    async fn update(&self, product: &Product) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
 UPDATE product
 SET name = ?1, unit = ?2, price = ?3
 WHERE item_number = ?4
 ";
        let mut parameters: Vec<Value> = Vec::with_capacity(4);
        parameters.push(product.name.clone().into());
        parameters.push(product.unit.into());
        parameters.push(product.price.into());
        parameters.push(product.item_number.into());
        self.connection
            .conn(move |connection| {
                let affected = connection.execute(QUERY, params_from_iter(parameters))?;
                Ok(affected)
            })
            .await?;
        Ok(())
    }
}
