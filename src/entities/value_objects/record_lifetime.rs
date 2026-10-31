// src/entities/value_objects/record_lifetime.rs
use time::UtcDateTime;

#[derive(Debug, Clone)]
pub struct RecordLifetime {
    pub(crate) created_at: UtcDateTime,
    pub(crate) updated_at: Option<UtcDateTime>,
    pub(crate) finished_at: Option<UtcDateTime>,
    pub(crate) deleted_at: Option<UtcDateTime>,
}

#[derive(Debug, Clone)]
pub enum RecordLifetimeError {
    UpdatedBeforeCreated,
    FinishedBeforeCreated,
    DeletedBeforeCreated,
}

impl RecordLifetime {
    pub fn create(
        created_at: UtcDateTime,
        updated_at: Option<UtcDateTime>,
        finished_at: Option<UtcDateTime>,
        deleted_at: Option<UtcDateTime>,
    ) -> Result<Self, RecordLifetimeError> {
        if let Some(updated_at) = updated_at {
            if updated_at < created_at {
                return Err(RecordLifetimeError::UpdatedBeforeCreated);
            }
        }
        if let Some(finished_at) = finished_at {
            if finished_at < created_at {
                return Err(RecordLifetimeError::FinishedBeforeCreated);
            }
        }
        if let Some(deleted_at) = deleted_at {
            if deleted_at < created_at {
                return Err(RecordLifetimeError::DeletedBeforeCreated);
            }
        }

        Ok(Self {
            created_at,
            updated_at,
            finished_at,
            deleted_at,
        })
    }
}
