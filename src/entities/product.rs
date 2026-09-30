use time::UtcDateTime;
use uuid::Uuid;

pub struct Product {
    pub(crate) item_number: Uuid,
    pub(crate) name: String,
    pub(crate) unit: i32,
    pub(crate) price: f32,

    pub(crate) created_at: UtcDateTime,
    pub(crate) updated_at: Option<UtcDateTime>,
    pub(crate) deleted_at: Option<UtcDateTime>,
}

impl Product {
    pub fn create(
        item_number: Uuid,
        name: String,
        unit: i32,
        price: f32,
        created_at: UtcDateTime,
        updated_at: Option<UtcDateTime>,
        deleted_at: Option<UtcDateTime>,
    ) -> Self {
        Self {
            item_number,
            name,
            unit,
            price,
            created_at,
            updated_at,
            deleted_at,
        }
    }
}

pub fn create_new() -> Self {
    let item_number: Uuid = Uuid::new_v4();
    let created_at: UtcDateTime = UtcDateTime::now();
    Self::create(item_number, name, unit, price, created_at)
}