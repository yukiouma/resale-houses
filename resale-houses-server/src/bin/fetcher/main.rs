use dotenv;
use resale_houses_server::fetcher::{Fetcher, Repo};
use std::{env, path::Path};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let server_base_url = env::var("SERVER_BASE_URL")?;
    let cookie = env::var("COOKIE_VALUE")?;
    let user_agent = env::var("USER_AGENT_VALUE")?;
    let repo = Repo::new(&server_base_url);
    let base_url = env::var("BASE_URL")?;
    let base_dir = env::var("BASE_DIR")?;

    let mut fetcher = Fetcher::new();
    fetcher
        .set_repo(&repo)
        .set_base_url(&base_url)
        .set_base_dir(Path::new(&base_dir))
        .set_cookie(&cookie)
        .set_user_agent(&user_agent);
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
