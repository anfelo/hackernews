mod args;
mod story;

use args::HackerNewsArgs;
use clap::Parser;
use story::Stories;

#[tokio::main]
async fn main() {
    let args = HackerNewsArgs::parse();
    let page_size = 10;
    let mut stories = Stories::new(args.page as u32, page_size);
    stories.fetch_top().await;
    stories.print_stories();
}
