use crate::{area::Area, repo::list_area};
use sqlx::{MySql, Pool};

pub struct AreaUsecase {
    pool: Pool<MySql>,
}

impl AreaUsecase {
    pub fn new(pool: Pool<MySql>) -> Self {
        AreaUsecase { pool }
    }
    pub async fn list_area(&self) -> anyhow::Result<Vec<Area>> {
        list_area(&self.pool).await
    }
}
