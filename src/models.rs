use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Story {
    pub by: String,
    pub descendants: u32,
    pub id: u64,
    pub score: u32,
    pub time: u64,
    pub title: String,
    pub url: String,
}
