use crate::entities::{Department, Product};
use time::UtcDateTime;
use uuid::Uuid;

/// Продажа: сколько единиц конкретного товара отдало подразделение.
#[derive(Debug, Clone)]
pub struct Sales {
    pub(crate) id: Uuid,
    pub(crate) unit_id: Uuid,
    pub(crate) department_id: Uuid,
    pub(crate) sail_date: UtcDateTime,
    pub(crate) sold_items_count: i32,
}

impl Sales {

    pub fn create(
        id: Uuid,
        unit_id: Uuid,
        department_id: Uuid,
        sail_date: UtcDateTime,
        sold_items_count: i32,
    ) -> Self {
        Self {
            id,
            unit_id,
            department_id,
            sail_date,
            sold_items_count,
        }
    }

    pub fn new(product: &Product, department: &Department, sold_items_count: i32) -> Self {
        Self::create(
            Uuid::new_v4(),
            product.item_number,
            department.id,
            UtcDateTime::now(),
            sold_items_count,
        )
    }
}
