use area::area::Area;
use reqwest::Client;

use crate::api::area::{ListAreaReply, UpdateAreaPageRequest};

pub struct Repo {
    client: Client,
    base_url: String,
}

impl Repo {
    pub fn new(base_url: &str) -> Self {
        Repo {
            client: Client::new(),
            base_url: base_url.into(),
        }
    }
    pub async fn list_area(&self) -> anyhow::Result<Vec<Area>> {
        let bytes = self
            .client
            .get(format!("{}/area", self.base_url))
            .send()
            .await?;
        let reply = serde_json::from_slice::<ListAreaReply>(&bytes.bytes().await?)?;
        Ok(reply.data)
    }
    pub async fn update_area_page(&self, id: i64, page: i64) -> anyhow::Result<()> {
        let body = UpdateAreaPageRequest { page };
        self.client
            .put(format!("{}/area/{}", self.base_url, id))
            .json(&body)
            .send()
            .await?;
        Ok(())
    }
}
