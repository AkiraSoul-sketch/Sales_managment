// src/entities/value_objects/quantity.rs
#[derive(Debug, Clone, PartialEq)]
pub struct Quantity(i32); // Используем i32 как в твоем коде, но будем проверять

#[derive(Debug, Clone, PartialEq)]
pub enum QuantityError {
    QuantityIsZeroOrNegative,
}

impl Quantity {
    pub fn new(value: i32) -> Result<Self, QuantityError> {
        if value <= 0 {
            return Err(QuantityError::QuantityIsZeroOrNegative);
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}
