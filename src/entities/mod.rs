//! Доменные сущности базы данных торговли.
//!
//! Имена модулей — в нижнем регистре (соответствуют именам файлов),
//! а типы реэкспортируются отсюда, чтобы дальше по коду использовался
//! короткий путь `crate::entities::Department`.

mod department;
mod product;
mod sales;
pub mod value_objects;

pub use department::Department;
pub use product::Product;
pub use sales::Sales;
pub use value_objects::*;
