use crate::entities::Product::Product;
use uuid::Uuid;
use std::future::Future;
pub trait ProductContract {
    fn add(&self, product: Product) ->
        impl Future<Output = Result<(), 
        Box<dyn std::error::Error>>>;
    
    fn remove(&self, product: Product) -> 
        impl Future<Output = Result<(), 
        Box<dyn std::error::Error>>>;
    
    fn get(&self, uuid: Uuid) -> 
        impl Future<Output = Result<(), 
        Box<dyn std::error::Error>>>;
    fn update(&self, product: &Product) ->
        impl Future<Output = Result<(), 
        Box<dyn std::error::Error>>>;
}    
