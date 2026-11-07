// src/entities/value_objects/entity_id.rs
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct EntityId(Uuid);

#[derive(Debug, Clone, PartialEq)]
pub enum EntityIdError {
    EmptyId,
    InvalidId,
}

impl EntityId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn create(id: Uuid) -> Result<Self, EntityIdError> {
        if id.is_nil() {
            return Err(EntityIdError::EmptyId);
        }
        // Упрощенная проверка, можно добавить как в методичке
        Ok(Self(id))
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}
