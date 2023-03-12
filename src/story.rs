use reqwest::{
    header::{ACCEPT, CONTENT_TYPE},
    Client,
};
use serde::{Deserialize, Serialize};

static HACKERNEWS_BASE_URL: &str = "https://hacker-news.firebaseio.com/v0";

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

pub struct Stories {
    pub items: Vec<Story>,
    pub pages: u32,
    pub current_page: u32,
    pub page_size: u32,
    client: Client,
}

impl Stories {
    pub fn new(current_page: u32, page_size: u32) -> Self {
        Self {
            items: vec![],
            pages: 0,
            current_page,
            page_size,
            client: reqwest::Client::new(),
        }
    }

    pub async fn fetch_story(&mut self, id: &u32) {
        let url = format!(
            "{url}/item/{id}.json?print=pretty",
            url = HACKERNEWS_BASE_URL,
            id = id,
        );
        let item_res = self
            .client
            .get(url)
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .send()
            .await
            .unwrap();

        match item_res.status() {
            reqwest::StatusCode::OK => {
                match item_res.json::<Story>().await {
                    Ok(story) => self.items.push(story.clone()),
                    Err(_) => println!("Hm, the response didn't match the shape we expected."),
                };
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                println!("Need to grab a new token");
            }
            other => {
                panic!("Uh oh! Something unexpected happened: {:?}", other);
            }
        }
    }

    pub async fn fetch_top(&mut self) {
        let url = format!(
            "{url}/topstories.json?print=pretty",
            url = HACKERNEWS_BASE_URL
        );
        let response = self
            .client
            .get(url)
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .send()
            .await
            .unwrap();

        match response.status() {
            reqwest::StatusCode::OK => {
                match response.json::<Vec<u32>>().await {
                    Ok(top_ids) => {
                        let start = (self.current_page - 1) * self.page_size;
                        let page_ids = &top_ids[start as usize..(start + self.page_size) as usize];

                        self.pages = top_ids.len() as u32 / self.page_size;

                        for item in page_ids.iter() {
                            self.fetch_story(item).await;
                        }
                    }
                    Err(_) => println!("Hm, the response didn't match the shape we expected."),
                };
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                println!("Need to grab a new token");
            }
            other => {
                panic!("Uh oh! Something unexpected happened: {:?}", other);
            }
        };
    }

    pub fn print_stories(self) {
        let from = (self.current_page - 1) * self.page_size;
        let to = from + self.page_size;

        println!(
            "Stories {} to {} - Page {}/{}",
            from, to, self.current_page, self.pages
        );
        println!("---------");

        for story in self.items {
            println!("🔥 {}", story.title);
            println!("🔗 {}", story.url);
            println!("---------")
        }
    }
}
