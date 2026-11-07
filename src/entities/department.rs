// src/entities/department.rs
use crate::entities::value_objects::{
    EntityId, EntityIdError, Name, NameError, PhoneNumber, PhoneNumberError, Quantity,
    QuantityError, RecordLifetime, RecordLifetimeError,
};
use time::UtcDateTime;

#[derive(Debug, Clone)]
pub struct Department {
    pub(crate) id: EntityId,
    pub(crate) name: Name,
    pub(crate) head_of_department: Name,
    pub(crate) number: PhoneNumber,
    pub(crate) volume: Quantity,
    pub(crate) record_lifetime: RecordLifetime,
}

#[derive(Debug)]
pub enum DepartmentError {
    IdError(EntityIdError),
    NameError(NameError),
    HeadNameError(NameError),
    PhoneNumberError(PhoneNumberError),
    VolumeError(QuantityError),
    RecordLifetimeError(RecordLifetimeError),
}

// Реализация From для удобного использования оператора ?
impl From<EntityIdError> for DepartmentError {
    fn from(err: EntityIdError) -> Self {
        Self::IdError(err)
    }
}
impl From<NameError> for DepartmentError {
    fn from(err: NameError) -> Self {
        Self::NameError(err)
    }
}
impl From<PhoneNumberError> for DepartmentError {
    fn from(err: PhoneNumberError) -> Self {
        Self::PhoneNumberError(err)
    }
}
impl From<QuantityError> for DepartmentError {
    fn from(err: QuantityError) -> Self {
        Self::VolumeError(err)
    }
}
impl From<RecordLifetimeError> for DepartmentError {
    fn from(err: RecordLifetimeError) -> Self {
        Self::RecordLifetimeError(err)
    }
}

impl Department {
    /// Принимает уже проверенные объекты-значения
    pub fn create(
        id: EntityId,
        name: Name,
        head_of_department: Name,
        number: PhoneNumber,
        volume: Quantity,
        record_lifetime: RecordLifetime,
    ) -> Self {
        Self {
            id,
            name,
            head_of_department,
            number,
            volume,
            record_lifetime,
        }
    }

    /// Удобный конструктор: принимает сырые данные, валидирует и создает
    pub fn new(
        name: String,
        head_of_department: String,
        number: String,
        volume: i32,
    ) -> Result<Self, DepartmentError> {
        let id = EntityId::new();
        let name = Name::new(name)?;
        let head_name = Name::new(head_of_department)?;
        let phone = PhoneNumber::new(number)?;
        let vol = Quantity::new(volume)?;

        let lifetime = RecordLifetime::create(UtcDateTime::now(), None, None, None)?;

        Ok(Self::create(id, name, head_name, phone, vol, lifetime))
    }
}
