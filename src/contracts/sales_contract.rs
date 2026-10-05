use crate::entities::Sales;
use std::future::Future;
use uuid::Uuid;

/// Контракт хранилища продаж.
pub trait SalesStorage {
    /// Регистрирует продажу.
    fn add(&self, sales: &Sales) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    /// Регистрирует несколько продаж одной транзакцией:
    /// либо записываются все, либо ни одной.
    fn add_many(
        &self,
        sales: &[Sales],
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    /// Возвращает продажу по идентификатору либо `None`.
    fn get(
        &self,
        uuid: Uuid,
    ) -> impl Future<Output = Result<Option<Sales>, Box<dyn std::error::Error>>>;

    /// Возвращает все продажи.
    fn list(&self) -> impl Future<Output = Result<Vec<Sales>, Box<dyn std::error::Error>>>;

    /// Возвращает продажи конкретного подразделения.
    fn list_by_department(
        &self,
        department_id: Uuid,
    ) -> impl Future<Output = Result<Vec<Sales>, Box<dyn std::error::Error>>>;

    /// Обновляет продажу по идентификатору.
    fn update(&self, sales: &Sales)
    -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    /// Удаляет продажу по идентификатору.
    fn remove(&self, uuid: Uuid) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;
}
