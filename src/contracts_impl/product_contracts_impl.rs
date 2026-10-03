use async_sqlite::Pool;
use std::path::PathBuf;
use async_sqlite::{PoolBuilder};
use traits::ProductStorage::ProductStorage;
pub struct ProductSqliteStorage {
    connection: Pool,
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
    async fn add(&self, product: crate::entities::Product) ->
    Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
    async fn get(&self, uuid: Uuid::Uuid) -> 
    Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
    async fn remove(&self, product: crate::entities::Product) ->
    Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
    async fn update(&self, product: crate::entities::Product) ->
    Result<(), Box<dyn std::error::Error>> {
        todo!()
    } 
}