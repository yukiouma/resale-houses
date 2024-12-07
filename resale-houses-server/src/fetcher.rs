mod error;
mod fetcher;
mod repo;

pub use fetcher::Fetcher;
pub use repo::Repo;
use std::{error::Error, path::Path};

async fn fetch() -> Result<(), Box<dyn Error>> {
    let areas = list_area().await?;
    let area_preview_dir = Path::new(r"");
    let area_dir = Path::new(r"");
    let community_preview_dir = Path::new(r"");
    let community_dir = Path::new(r"");
    let houses_dir = Path::new(r"");
    let codes = areas.iter().map(|e| e.code.as_str()).collect::<Vec<&str>>();
    fetch_area_preview(&codes, area_preview_dir).await?;
    extract_area_total_page(area_preview_dir).await?;
    fetch_area_pages(area_dir).await?;
    extract_communities(area_dir).await?;
    fetch_community_preview(community_preview_dir).await?;
    extract_community_total_page(community_preview_dir).await?;
    fetch_community_pages(community_dir).await?;
    extract_houses(community_dir).await?;
    fetch_houses(houses_dir).await?;
    extract_house_details(houses_dir).await?;
    Ok(())
}

async fn list_area() -> Result<Vec<Area>, Box<dyn Error>> {
    todo!()
}

async fn fetch_area_preview(code: &[&str], dest: &Path) -> Result<(), Box<dyn Error>> {
    todo!()
}

async fn extract_area_total_page(dir: &Path) -> Result<(), Box<dyn Error>> {
    todo!()
}

async fn fetch_area_pages(dir: &Path) -> Result<(), Box<dyn Error>> {
    todo!()
}

async fn extract_communities(dir: &Path) -> Result<(), Box<dyn Error>> {
    todo!()
}

async fn fetch_community_preview(dir: &Path) -> Result<(), Box<dyn Error>> {
    todo!()
}

async fn extract_community_total_page(dir: &Path) -> Result<(), Box<dyn Error>> {
    todo!()
}

async fn fetch_community_pages(dir: &Path) -> Result<(), Box<dyn Error>> {
    todo!()
}

async fn extract_houses(dir: &Path) -> Result<(), Box<dyn Error>> {
    todo!()
}

async fn fetch_houses(dir: &Path) -> Result<(), Box<dyn Error>> {
    todo!()
}

async fn extract_house_details(dir: &Path) -> Result<(), Box<dyn Error>> {
    todo!()
}

struct Area {
    id: usize,
    code: String,
    name: String,
}
