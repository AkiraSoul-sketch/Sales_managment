use async_sqlite::{ ClientBuilder};

type ManagedPool = (
 smol::channel::Sender<Client>,
 smol::channel::Receiver<Client>,
);

pub struct ManagedConnectionPool {
 database_path: String,
 max_connections_count: usize,
 pool: ManagedPool,
}

ipml ManagedConnectionPool {
    pub async fn initialize(database_path: String, max_connections_count: usize) -> Self {
 let path_clone = database_path.clone();
 let (sender, receiver) =
smol::channel::bounded::<Client>(max_connections_count);
 for _ in 0..max_connections_count {
 let connection = Self::create_connection(path_clone.clone()).await;
 sender.send(connection).await.expect("Не удается наполнить пул
соединений.")
 }
 Self {
 database_path: path_clone,
 max_connections_count,
 pool: (sender, receiver),
 }
}

async fn create_connection(database_path: String) -> Client {
 let builder: ClientBuilder =
ClientBuilder::new().path(database_path.clone());
 builder.open().await.expect("Не удается открыть соединение.")
}

async fn acquire_connection(&self) ->
 Result<Client, Box<dyn std::error::Error>> {
 let connection = self.pool.1.recv().await?;
 Ok(connection)
}

async fn return_connection(&self, connection: Client) ->
 Result<(), Box<dyn std::error::Error>> {
 self.pool.0.send(connection).await?;
 Ok(())
 }

}