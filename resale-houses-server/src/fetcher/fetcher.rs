use std::path::Path;

use area::{area::Area, usecase::AreaUsecase};

use super::error::Error;

#[derive(Default)]
pub struct Fetcher<'a> {
    base_url: &'a str,
    base_dir: Option<&'a Path>,
    area_usecase: Option<&'a AreaUsecase>,
}

impl<'a> Fetcher<'a> {
    pub fn new() -> Self {
        Fetcher::default()
    }
    pub fn set_area_usecase(&mut self, area_usecase: &'a AreaUsecase) -> &mut Self {
        self.area_usecase = Some(&area_usecase);
        self
    }
    pub fn set_base_url(&mut self, base_url: &'a str) -> &mut Self {
        self.base_url = base_url;
        self
    }
    pub fn set_base_dir(&mut self, base_dir: &'a Path) -> &mut Self {
        self.base_dir = Some(base_dir);
        self
    }
    pub async fn fetch(&self) -> anyhow::Result<(), Error> {
        let areas = self.list_area().await?;
        let codes = areas
            .iter()
            .map(|area| area.code.as_str())
            .collect::<Vec<&str>>();
        self.fetch_area_preview(&codes).await?;
        Ok(())
    }
    async fn list_area(&self) -> anyhow::Result<Vec<Area>, Error> {
        let uc = self.area_usecase.ok_or(Error::ErrAreaUsecaseNotSet)?;
        let area = uc.list_area().await.map_err(|_| Error::ErrListAreaFailed)?;
        Ok(area)
    }
    async fn fetch_area_preview(&self, codes: &[&str]) -> anyhow::Result<(), Error> {
        Ok(())
    }
}
