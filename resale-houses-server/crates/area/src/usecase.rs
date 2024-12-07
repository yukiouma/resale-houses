use crate::{
    area::Area,
    repo::{list_area, update_area_pages},
};
use sqlx::{MySql, Pool};

#[derive(Debug, Clone)]
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
    pub async fn update_area_pages(&self, area: &Area) -> anyhow::Result<()> {
        update_area_pages(&self.pool, area).await
    }
}
