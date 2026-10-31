// src/entities/value_objects/price.rs
#[derive(Debug, Clone, PartialEq)]
pub struct Price(f64);

#[derive(Debug, Clone, PartialEq)]
pub enum PriceError {
    PriceIsNegativeOrZero,
}

impl Price {
    pub fn new(value: f64) -> Result<Self, PriceError> {
        if value <= 0.0 {
            return Err(PriceError::PriceIsNegativeOrZero);
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}
