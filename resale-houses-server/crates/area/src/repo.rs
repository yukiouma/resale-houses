use crate::area::Area;
use sqlx::{MySql, Pool};

pub async fn list_area(pool: &Pool<MySql>) -> anyhow::Result<Vec<Area>> {
    let areas = sqlx::query_as::<_, Area>("SELECT `id`, `name`, `code`, `page` FROM `area`")
        .fetch_all(pool)
        .await?;
    Ok(areas)
}

pub async fn update_area_pages(pool: &Pool<MySql>, area: &Area) -> anyhow::Result<()> {
    if let Some(id) = area.id {
        sqlx::query("UPDATE `area` SET `page` = ? WHERE `id` = ?")
            .bind(area.page)
            .bind(id)
            .execute(pool)
            .await?;
    }
    Ok(())
}
