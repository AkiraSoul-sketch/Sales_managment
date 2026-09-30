use time::UtcDateTime;
use uuid::Uuid;

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
        unit_id: &Product,
        department_id: &Department,
        sail_date: UtcDateTime,
        sold_items_count: i32,
    ) -> Self {
        Sales {
            id,
            unit_id,
            department_id,
            sail_date,
            sold_items_count,
        };
    }
}
    pub fn create_new(product: &Product, department: &Department) -> Self {
        let new_sales = Sales {
            unit_id: &Product,
            department_id: &Department,
            sail_date: UtcDateTime::now(),
            sold_items_count: 0,
        };
    }
