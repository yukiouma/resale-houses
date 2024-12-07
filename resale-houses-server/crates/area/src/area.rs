use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, Default)]
pub struct Area {
    pub id: Option<i64>,
    pub name: String,
    pub code: String,
    pub page: i64,
}
