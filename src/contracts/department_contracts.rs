use crate::entities::Department::Department;
use uuid::Uuid;
use std::future::Future;

pub trait DepartmentStorage {
    fn add(&self, department: Department) ->
        impl Future<Output = Result<(), 
        Box<dyn std::error::Error>>>;
    
    fn remove(&self, department: Department) -> 
        impl Future<Output = Result<(), 
        Box<dyn std::error::Error>>>;
    
    fn get(&self, uuid: Uuid) -> 
        impl Future<Output = Result<(), 
        Box<dyn std::error::Error>>>;
    fn update(&self, department: &Department) ->
        impl Future<Output = Result<(), 
        Box<dyn std::error::Error>>>;
}
