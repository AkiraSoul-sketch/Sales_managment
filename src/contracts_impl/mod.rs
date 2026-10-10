//! Реализации контрактов доступа к данным поверх SQLite (`async-sqlite`).
//!
//! Здесь же находятся общие для всех хранилищ вспомогательные функции:
//! открытие пула соединений, создание схемы и безопасное чтение `Uuid` и чисел.

mod department_contracts_impl;
mod product_contracts_impl;
mod sales_contracts_impl;

pub use department_contracts_impl::DepartmentSqliteStorage;
pub use product_contracts_impl::ProductSqliteStorage;
pub use sales_contracts_impl::SalesSqliteStorage;

use async_sqlite::{JournalMode, Pool, PoolBuilder};
use rusqlite::Row;
use rusqlite::types::{FromSqlError, Type, Value};
use std::error::Error;
use std::path::PathBuf;
use uuid::Uuid;

/// Количество соединений в пуле. Записи в SQLite в любом случае сериализуются,
/// поэтому большой пул не ускоряет запись, зато ускоряет параллельное чтение.
const NUM_CONNS: usize = 4;

/// Открывает пул соединений к файлу базы данных `database_url`.
pub async fn open_pool(database_url: &str) -> Result<Pool, Box<dyn Error>> {
    let path_buf: PathBuf = PathBuf::from(database_url);

    // Каталог под файл БД никто не создаёт, а SQLite умеет создавать только сам файл.
    if let Some(parent) = path_buf.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)?;
    }

    let builder: PoolBuilder = PoolBuilder::new()
        .path(path_buf)
        .journal_mode(JournalMode::Wal)
        .num_conns(NUM_CONNS);
    let connection: Pool = builder.open().await?;

    // Контроль внешних ключей в SQLite выключен по умолчанию и включается на каждом соединении.
    let pragmas = connection
        .conn_for_each(|connection| connection.execute_batch("PRAGMA foreign_keys = ON;"))
        .await;
    for result in pragmas {
        result?;
    }

    Ok(connection)
}

/// Создаёт таблицы, если их ещё нет. DDL повторяет скрипты из `migrations/`.
pub async fn create_schema(connection: &Pool) -> Result<(), Box<dyn Error>> {
    const QUERY: &str = "
        CREATE TABLE IF NOT EXISTS department (
            id                 TEXT PRIMARY KEY NOT NULL,
            name               VARCHAR NOT NULL,
            Head_of_Department VARCHAR NOT NULL,
            number             INTEGER NOT NULL,
            volume             INTEGER NOT NULL,
            created_at         TEXT NOT NULL,
            updated_at         TEXT,
            deleted_at         TEXT
        );

        CREATE TABLE IF NOT EXISTS product (
            item_number TEXT PRIMARY KEY NOT NULL,
            name        TEXT NOT NULL,
            unit        INTEGER NOT NULL,
            price       REAL NOT NULL,
            created_at  TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at  TEXT,
            deleted_at  TEXT
        );

        CREATE TABLE IF NOT EXISTS sales (
            id               TEXT PRIMARY KEY NOT NULL,
            unit_id          TEXT NOT NULL,
            department_id    TEXT NOT NULL,
            sail_date        TEXT NOT NULL DEFAULT (datetime('now')),
            sold_items_count INTEGER NOT NULL,

            FOREIGN KEY (unit_id) REFERENCES product(item_number),
            FOREIGN KEY (department_id) REFERENCES department(id)
        );
    ";

    connection
        .conn_mut(|connection| connection.execute_batch(QUERY))
        .await?;
    Ok(())
}

/// Читает `Uuid` из колонки, в которой идентификатор лежит строкой.
///
/// Идентификаторы всегда пишем как TEXT, но колонка может быть объявлена и как BLOB
/// (SQLite приведение типов не строгое), поэтому принимаем оба представления.
pub(crate) fn row_uuid(row: &Row<'_>, index: usize) -> rusqlite::Result<Uuid> {
    let parsed: Result<Uuid, Box<dyn Error + Send + Sync>> = match row.get::<_, Value>(index)? {
        Value::Text(text) => Uuid::parse_str(&text).map_err(Into::into),
        Value::Blob(bytes) => <[u8; 16]>::try_from(bytes.as_slice())
            .map(Uuid::from_bytes)
            .map_err(Into::into),
        _ => Err("ожидались TEXT или BLOB".into()),
    };

    parsed.map_err(|err| rusqlite::Error::FromSqlConversionFailure(index, Type::Text, err))
}

/// Читает `i32` из колонки, которая может хранить и INTEGER, и REAL.
///
/// Типы в SQLite аффинитивны, а не строги: в колонке REAL окажутся вещественные
/// значения даже при вставке целого, поэтому приводим оба варианта.
pub(crate) fn row_i32(row: &Row<'_>, index: usize) -> rusqlite::Result<i32> {
    match row.get::<_, Value>(index)? {
        Value::Integer(value) => Ok(value as i32),
        Value::Real(value) => Ok(value as i32),
        other => Err(rusqlite::Error::FromSqlConversionFailure(
            index,
            Type::Integer,
            Box::new(FromSqlError::Other(
                format!("ожидалось целое значение, получено {other:?}").into(),
            )),
        )),
    }
}
