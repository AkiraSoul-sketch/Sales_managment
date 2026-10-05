//! Контракты (трейты) слоя доступа к данным.
//!
//! Модули публичны, чтобы реализации из `crate::contracts_impl`
//! и вызывающий код могли обратиться к трейтам по пути `crate::contracts::...`.

pub mod department_contracts;
pub mod product_contracts;
pub mod sales_contract;

pub use department_contracts::DepartmentStorage;
pub use product_contracts::ProductStorage;
pub use sales_contract::SalesStorage;
