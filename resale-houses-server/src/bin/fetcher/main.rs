use area::usecase::AreaUsecase;
use reqwest::{
    header::{COOKIE, USER_AGENT},
    Client,
};
use resale_houses_server::fetcher::Fetcher;
use sqlx::MySqlPool;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let cookie = env::var("COOKIE_VALUE")?;
    let user_agent = env::var("USER_AGENT_VALUE")?;
    let database_url = env::var("DATABASE_URL")?;
    let pool = MySqlPool::connect(&database_url).await?;
    let area_usecase = AreaUsecase::new(pool.clone());
    let base_url = "https://gz.ke.com/";

    let mut fetcher = Fetcher::new();
    fetcher
        .set_area_usecase(&area_usecase)
        .set_base_url(base_url);
    fetcher.fetch().await.unwrap();

    // let mut target = OpenOptions::new()
    //     .create(true)
    //     .write(true)
    //     .open("/home/yuki/Projects/resale-houses/.data/dummy/area.html")?;
    // let url = "https://gz.ke.com/xiaoqu/haizhu/";
    // let client = Client::new();
    // let response = client
    //     .get(url)
    //     .header(COOKIE, &cookie)
    //     .header(USER_AGENT, &user_agent)
    //     .send()
    //     .await?;
    // target.write(&response.bytes().await?)?;
    Ok(())
}
