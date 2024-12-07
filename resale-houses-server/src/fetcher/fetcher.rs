use std::{fs, path::Path, time::Duration};

use area::area::Area;
use reqwest::{
    header::{COOKIE, USER_AGENT},
    Client, IntoUrl,
};

use super::{error::Error, Repo};

#[derive(Default)]
pub struct Fetcher<'a> {
    base_url: &'a str,
    base_dir: Option<&'a Path>,
    repo: Option<&'a Repo>,
    cookie: &'a str,
    user_agent: &'a str,
    client: Client,
}

impl<'a> Fetcher<'a> {
    pub fn new() -> Self {
        Fetcher::default()
    }
    pub fn set_repo(&mut self, repo: &'a Repo) -> &mut Self {
        self.repo = Some(&repo);
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
    pub fn set_cookie(&mut self, cookie: &'a str) -> &mut Self {
        self.cookie = cookie;
        self
    }
    pub fn set_user_agent(&mut self, user_agent: &'a str) -> &mut Self {
        self.user_agent = user_agent;
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
        let uc = self.repo.ok_or(Error::ErrRepoNotSet)?;
        let area = uc.list_area().await.map_err(|_| Error::ErrListAreaFailed)?;
        Ok(area)
    }
    async fn fetch_area_preview(&self, codes: &[&str]) -> anyhow::Result<(), Error> {
        let destination = self
            .base_dir
            .ok_or(Error::ErrInvalidBaseDir)?
            .join("area_preview");
        if !destination.exists() {
            fs::create_dir_all(&destination).map_err(|_| Error::ErrCreateDirFailed)?;
        }
        for code in codes {
            let url = format!("{}/xiaoqu/{}/", self.base_url, code);
            let page = self
                .fetch_page(url)
                .await
                .map_err(|_| Error::ErrFetchPageFailed)?;
            fs::write(&destination.join(format!("{}.html", code)), &page)
                .map_err(|_| Error::ErrSavePageFailed)?;
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
        Ok(())
    }
    async fn fetch_page<U: IntoUrl>(&self, url: U) -> anyhow::Result<Vec<u8>> {
        println!("fetching: {:?}", url.as_str());
        let response = self
            .client
            .get(url)
            .header(COOKIE, self.cookie)
            .header(USER_AGENT, self.user_agent)
            .send()
            .await?;
        Ok(response.bytes().await?.to_vec())
    }
}
