mod args;
mod models;

use args::HackerNewsArgs;
use clap::Parser;
use models::Story;
use reqwest::{
    self,
    header::{ACCEPT, CONTENT_TYPE},
};

// tokio let's us use "async" on our main function
#[tokio::main]
async fn main() {
    let args = HackerNewsArgs::parse();
    let page_size = 10;

    let url = "https://hacker-news.firebaseio.com/v0/topstories.json?print=pretty";
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<Vec<u32>>().await {
                Ok(parsed) => {
                    let mut stories: Vec<Story> = vec![];
                    for item in parsed[0..page_size].iter() {
                        let url = format!(
                            "https://hacker-news.firebaseio.com/v0/item/{item}.json?print=pretty",
                            item = item,
                        );

                        let item_res = client
                            .get(url)
                            .header(CONTENT_TYPE, "application/json")
                            .header(ACCEPT, "application/json")
                            .send()
                            .await
                            .unwrap();

                        match item_res.status() {
                            reqwest::StatusCode::OK => {
                                match item_res.json::<Story>().await {
                                    Ok(story) => stories.push(story.clone()),
                                    Err(_) => println!(
                                        "Hm, the response didn't match the shape we expected."
                                    ),
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
                    print_stories(stories);
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

fn print_stories(stories: Vec<Story>) {
    for story in stories {
        println!("🔥 {}", story.title);
        println!("🔗 {}", story.url);
        println!("---------")
    }
}
