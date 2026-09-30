use time::UtcDateTime;
use uuid::Uuid;

pub struct Department {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) Head_of_Department: String,
    pub(crate) number: i32,
    pub(crate) volume: i32,

    pub(crate) created_at: UtcDateTime,
    pub(crate) updated_at: Option<UtcDateTime>,
    pub(crate) deleted_at: Option<UtcDateTime>,

}

impl Department {
    pub fn create(
        id: Uuid,
        name: String,
        head_of_department: String,
        number: i32,
        volume: i32,
        created_at: UtcDateTime,
        updated_at: Option<UtcDateTime>,
        deleted_at: Option<UtcDateTime>,
    ) -> Self {
        Self {
            id,
            head_of_department,
            number,
            volume,
            created_at,
            updated_at,
            deleted_at,
        }
    }

}

pub fn create_new() -> Self {
    let id: Uuid = Uuid::new_v4();
    let created_at: UtcDateTime = UtcDateTime::now();
    Self::create(id, name, head_of_department, number, volume, created_at)
}
