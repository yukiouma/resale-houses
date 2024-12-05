use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Area {
    pub id: Option<i64>,
    pub name: String,
    pub code: String,
    pub page: i64,
}
