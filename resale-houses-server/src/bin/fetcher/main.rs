use reqwest::{
    header::{COOKIE, USER_AGENT},
    Client,
};
use std::{env, error::Error, fs::OpenOptions, io::Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let cookie = env::var("COOKIE_VALUE")?;
    let user_agent = env::var("USER_AGENT_VALUE")?;
    let mut target = OpenOptions::new()
        .create(true)
        .write(true)
        .open("/home/yuki/Projects/resale-houses/.data/dummy/area.html")?;
    let url = "https://gz.ke.com/xiaoqu/haizhu/";
    let client = Client::new();
    let response = client
        .get(url)
        .header(COOKIE, &cookie)
        .header(USER_AGENT, &user_agent)
        .send()
        .await?;
    target.write(&response.bytes().await?)?;
    Ok(())
}
