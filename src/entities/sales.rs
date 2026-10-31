// src/entities/sales.rs
use crate::entities::value_objects::{
    EntityId, EntityIdError, Quantity, QuantityError, RecordLifetime, RecordLifetimeError,
};
use time::UtcDateTime;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Sales {
    pub(crate) id: EntityId,
    pub(crate) unit_id: EntityId,
    pub(crate) department_id: EntityId,
    pub(crate) sold_items_count: Quantity,
    pub(crate) record_lifetime: RecordLifetime, // sail_date будет здесь как created_at
}

#[derive(Debug)]
pub enum SalesError {
    IdError(EntityIdError),
    UnitIdError(EntityIdError),
    DepartmentIdError(EntityIdError),
    CountError(QuantityError),
    RecordLifetimeError(RecordLifetimeError),
}

impl From<EntityIdError> for SalesError {
    fn from(e: EntityIdError) -> Self {
        Self::IdError(e)
    }
}
impl From<QuantityError> for SalesError {
    fn from(e: QuantityError) -> Self {
        Self::CountError(e)
    }
}
impl From<RecordLifetimeError> for SalesError {
    fn from(e: RecordLifetimeError) -> Self {
        Self::RecordLifetimeError(e)
    }
}

impl Sales {
    pub fn create(
        id: EntityId,
        unit_id: EntityId,
        department_id: EntityId,
        sold_items_count: Quantity,
        record_lifetime: RecordLifetime,
    ) -> Self {
        Self {
            id,
            unit_id,
            department_id,
            sold_items_count,
            record_lifetime,
        }
    }

    pub fn new(
        unit_id: Uuid,
        department_id: Uuid,
        sold_items_count: i32,
    ) -> Result<Self, SalesError> {
        let id = EntityId::new();
        let u_id = EntityId::create(unit_id)?;
        let d_id = EntityId::create(department_id)?;
        let count = Quantity::new(sold_items_count)?;

        // sail_date используем как created_at
        let lifetime = RecordLifetime::create(UtcDateTime::now(), None, None, None)?;

        Ok(Self::create(id, u_id, d_id, count, lifetime))
    }
}
