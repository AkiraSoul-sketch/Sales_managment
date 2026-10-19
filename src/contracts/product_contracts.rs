use crate::entities::Product;
use std::future::Future;
use uuid::Uuid;

/// Контракт хранилища товаров.
pub trait ProductStorage {
    /// Добавляет новый товар.
    fn add(
        &self,
        product: &Product,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    /// Возвращает товар по артикулу либо `None`,
    /// если запись отсутствует или помечена как удалённая.
    fn get(
        &self,
        uuid: Uuid,
    ) -> impl Future<Output = Result<Option<Product>, Box<dyn std::error::Error>>>;

    /// Обновляет товар по артикулу.
    fn update(
        &self,
        product: &Product,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    /// Удаляет товар по артикулу.
    fn remove(&self, uuid: Uuid) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;
}
