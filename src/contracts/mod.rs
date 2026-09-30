pub trait ContractsStorage {
    fn add(&self, contract: Contract) -> 
        impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    fn remove(&self, contract: Contract) -> 
        impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    fn get(&self, uuid: Uuid) -> 
        impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    fn update(&self, contract: &Contract) -> 
        impl Future<Output = Result<(), Box<dyn std::error::Error>>>;
}

impl ContractsStorage for ContractsSqliteStorage {

    async fn add(&self, contract: crate::entities::Contract) -> 
        Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn get(&self, uuid: uuid::Uuid) -> 
        Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn remove(&self, contract: crate::entities::Contract) -> 
        Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn update(&self, contract: &crate::entities::Contract) -> 
        Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

}