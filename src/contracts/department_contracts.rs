use crate::entities::Department;
use std::future::Future;
use uuid::Uuid;

/// Контракт хранилища подразделений.
///
/// Методы асинхронные: реализации работают поверх пула соединений SQLite.
pub trait DepartmentStorage {
    /// Добавляет новое подразделение.
    fn add(
        &self,
        department: &Department,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    /// Возвращает подразделение по идентификатору либо `None`,
    /// если запись отсутствует или помечена как удалённая.
    fn get(
        &self,
        uuid: Uuid,
    ) -> impl Future<Output = Result<Option<Department>, Box<dyn std::error::Error>>>;

    /// Обновляет подразделение по идентификатору.
    fn update(
        &self,
        department: &Department,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    /// Удаляет подразделение по идентификатору.
    fn remove(&self, uuid: Uuid) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;
}
