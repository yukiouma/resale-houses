use area::usecase::AreaUsecase;
use sqlx::{MySql, MySqlPool, Pool};

#[tokio::test]
async fn area() -> anyhow::Result<()> {
    let url = "mysql://root:000000@localhost:3306/resale-houses?parseTime=True";
    let pool = create_pool(url).await;
    let uc = AreaUsecase::new(pool);
    let areas: Vec<area::area::Area> = uc.list_area().await?;
    assert_eq!(areas.len(), 1);
    let mut area = areas.first().unwrap().clone();
    area.page = 4;
    uc.update_area_pages(&area).await?;
    Ok(())
}

pub async fn create_pool(url: &str) -> Pool<MySql> {
    MySqlPool::connect(url)
        .await
        .expect("failed to connect database")
}
