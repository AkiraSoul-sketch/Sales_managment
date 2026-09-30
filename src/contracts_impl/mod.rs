use async_sqlite::Pool;

pub struct ContractsSqliteStorage {
    connection: Pool,
}

impl ContractsSqliteStorage {
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