use area::area::Area;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ListAreaReply {
    pub data: Vec<Area>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAreaPageRequest {
    pub page: i64,
}

#[derive(Debug, Serialize)]
pub struct UpdateAreaPageReply {}
