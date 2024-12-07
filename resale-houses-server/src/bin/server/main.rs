use area::usecase::AreaUsecase;
use resale_houses_server::server::Server;
use sqlx::MySqlPool;
use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let port = env::var("SERVER_PORT")?.parse::<usize>()?;
    let database_url = env::var("DATABASE_URL")?;
    let pool = MySqlPool::connect(&database_url).await?;
    let area = AreaUsecase::new(pool);
    let server = Server::new(area);
    server.run(port).await?;
    Ok(())
}
