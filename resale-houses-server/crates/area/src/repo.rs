use crate::area::Area;
use sqlx::{MySql, Pool};

pub async fn list_area(pool: &Pool<MySql>) -> anyhow::Result<Vec<Area>> {
    let areas = sqlx::query_as::<_, Area>("SELECT `id`, `name`, `code`, `page` FROM `area`")
        .fetch_all(pool)
        .await?;
    Ok(areas)
}
