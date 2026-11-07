use regex::Regex;
use std::sync::LazyLock;

static PHONE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\A7[0-9]{10}\z").expect("Phone regex must be valid"));

#[derive(Debug, Clone, PartialEq)] // ← ДОБАВИТЬ
pub struct PhoneNumber(String);

#[derive(Debug, Clone, PartialEq)] // ← ДОБАВИТЬ
pub enum PhoneNumberError {
    EmptyNumber,
    InvalidFormat,
}

impl PhoneNumber {
    pub fn new(value: impl Into<String>) -> Result<Self, PhoneNumberError> {
        let value: String = value.into();

        if value.trim().is_empty() {
            return Err(PhoneNumberError::EmptyNumber);
        }

        if !PHONE_REGEX.is_match(&value) {
            return Err(PhoneNumberError::InvalidFormat);
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
