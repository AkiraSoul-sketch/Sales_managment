mod contracts;
mod contracts_impl;
mod entities;

use contracts::{DepartmentStorage, ProductStorage, SalesStorage};
use contracts_impl::{DepartmentSqliteStorage, ProductSqliteStorage, SalesSqliteStorage};
use entities::{Department, Product, Sales};
use std::error::Error;
use uuid::Uuid;

/// Каталог и файл базы данных по умолчанию.
const DEFAULT_DATABASE_URL: &str = "database/sales_manager.db";

/// Путь к базе можно переопределить переменной окружения `DATABASE_URL`.
fn database_url() -> String {
    std::env::var("DATABASE_URL").unwrap_or_else(|_| DEFAULT_DATABASE_URL.to_owned())
}

fn main() {
    if let Err(error) = smol::block_on(run(database_url())) {
        eprintln!("Ошибка: {error}");
        std::process::exit(1);
    }
}

/// Демонстрация работы хранилищ: добавление, чтение, изменение и удаление записей.
async fn run(database_url: String) -> Result<(), Box<dyn Error>> {
    let departments = DepartmentSqliteStorage::new(&database_url).await?;
    let products = ProductSqliteStorage::new(&database_url).await?;
    let sales = SalesSqliteStorage::new(&database_url).await?;

    let department = Department::new("Отдел №1".to_owned(), "Иванов И. И.".to_owned(), 1, 100);
    departments.add(&department).await?;
    println!("Подразделение: {:?}", departments.get(department.id).await?);

    let product = Product::new("Молоко".to_owned(), 1, 85.50);
    products.add(&product).await?;
    println!("Товар: {:?}", products.get(product.item_number).await?);

    let first = Sales::new(&product, &department, 10);
    sales.add(&first).await?;

    let second = Sales::new(&product, &department, 5);
    let third = Sales::new(&product, &department, 7);
    sales.add_many(&[second.clone(), third]).await?;

    println!(
        "Продаж в подразделении: {}",
        sales.list_by_department(department.id).await?.len()
    );
    println!(
        "Продажа по идентификатору: {:?}",
        sales.get(second.id).await?
    );

    let mut department_update = department.clone();
    department_update.volume = 120;
    departments.update(&department_update).await?;

    let mut product_update = product.clone();
    product_update.price = 90.00;
    products.update(&product_update).await?;

    let mut sales_update = second.clone();
    sales_update.sold_items_count = 6;
    sales.update(&sales_update).await?;

    println!("Подразделений в базе: {}", departments.list().await?.len());
    println!("Товаров в базе: {}", products.list().await?.len());
    println!("Продаж в базе: {}", sales.list().await?.len());

    // Порядок удаления важен из-за внешних ключей: сначала продажи, затем справочники.
    for sale in sales.list().await? {
        sales.remove(sale.id).await?;
    }
    products.remove(product.item_number).await?;
    departments.remove(department.id).await?;

    println!(
        "Удалённого подразделения нет: {:?}",
        departments.get(Uuid::nil()).await?
    );

    departments.close().await?;
    products.close().await?;
    sales.close().await?;
    Ok(())
}
