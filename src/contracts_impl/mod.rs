//! Реализации контрактов доступа к данным поверх SQLite (`async-sqlite`).
//!
//! Здесь же находятся общие для всех хранилищ вспомогательные функции:
//! открытие пула соединений, создание схемы и безопасное чтение `Uuid` и чисел.

mod department_contracts_impl;
mod managed_connection_pool;
mod product_contracts_impl;
mod sales_contracts_impl;
