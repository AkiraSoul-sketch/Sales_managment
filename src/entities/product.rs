// src/entities/product.rs
use crate::entities::value_objects::{
    EntityId, EntityIdError, Name, NameError, Price, PriceError, Quantity, QuantityError,
    RecordLifetime, RecordLifetimeError,
};
use time::UtcDateTime;

#[derive(Debug, Clone)]
pub struct Product {
    pub(crate) item_number: EntityId,
    pub(crate) name: Name,
    pub(crate) unit: Quantity,
    pub(crate) price: Price,
    pub(crate) record_lifetime: RecordLifetime,
}

#[derive(Debug)]
pub enum ProductError {
    IdError(EntityIdError),
    NameError(NameError),
    UnitError(QuantityError),
    PriceError(PriceError),
    RecordLifetimeError(RecordLifetimeError),
}

impl From<EntityIdError> for ProductError {
    fn from(e: EntityIdError) -> Self {
        Self::IdError(e)
    }
}
impl From<NameError> for ProductError {
    fn from(e: NameError) -> Self {
        Self::NameError(e)
    }
}
impl From<QuantityError> for ProductError {
    fn from(e: QuantityError) -> Self {
        Self::UnitError(e)
    }
}
impl From<PriceError> for ProductError {
    fn from(e: PriceError) -> Self {
        Self::PriceError(e)
    }
}
impl From<RecordLifetimeError> for ProductError {
    fn from(e: RecordLifetimeError) -> Self {
        Self::RecordLifetimeError(e)
    }
}

impl Product {
    pub fn create(
        item_number: EntityId,
        name: Name,
        unit: Quantity,
        price: Price,
        record_lifetime: RecordLifetime,
    ) -> Self {
        Self {
            item_number,
            name,
            unit,
            price,
            record_lifetime,
        }
    }

    pub fn new(name: String, unit: i32, price: f64) -> Result<Self, ProductError> {
        let item_number = EntityId::new();
        let name = Name::new(name)?;
        let unit_qty = Quantity::new(unit)?;
        let price_obj = Price::new(price)?;

        let lifetime = RecordLifetime::create(UtcDateTime::now(), None, None, None)?;

        Ok(Self::create(
            item_number,
            name,
            unit_qty,
            price_obj,
            lifetime,
        ))
    }
}
