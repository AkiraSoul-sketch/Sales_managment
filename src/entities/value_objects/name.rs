// src/entities/value_objects/name.rs
#[derive(Debug, Clone, PartialEq)]
pub struct Name(String);

#[derive(Debug, Clone, PartialEq)]
pub enum NameError {
    NameIsEmpty,
    NameTooLong,
    NameTooShort,
}

impl Name {
    pub fn new(value: impl Into<String>) -> Result<Self, NameError> {
        let value = value.into().trim().to_string();

        if value.is_empty() {
            return Err(NameError::NameIsEmpty);
        }
        if value.len() < 2 {
            return Err(NameError::NameTooShort);
        }
        if value.len() > 255 {
            return Err(NameError::NameTooLong);
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
