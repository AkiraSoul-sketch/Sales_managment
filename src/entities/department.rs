use time::UtcDateTime;
use uuid::Uuid;

/// Подразделение торговой сети.
#[derive(Debug, Clone)]
pub struct Department {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) head_of_department: String,
    pub(crate) number: i32,
    pub(crate) volume: i32,

    pub(crate) created_at: UtcDateTime,
    pub(crate) updated_at: Option<UtcDateTime>,
    pub(crate) deleted_at: Option<UtcDateTime>,
}

impl Department {
    /// Полный конструктор: все поля задаются явно.
    /// Используется при чтении готовой записи из базы данных.
    #[expect(
        clippy::too_many_arguments,
        reason = "конструктор повторяет состав таблицы department"
    )]
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
            name,
            head_of_department,
            number,
            volume,
            created_at,
            updated_at,
            deleted_at,
        }
    }

    /// Создаёт новое подразделение: идентификатор и время создания
    /// генерируются автоматически, отметок изменения и удаления нет.
    pub fn new(name: String, head_of_department: String, number: i32, volume: i32) -> Self {
        Self::create(
            Uuid::new_v4(),
            name,
            head_of_department,
            number,
            volume,
            UtcDateTime::now(),
            None,
            None,
        )
    }
}
