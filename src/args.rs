use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct HackerNewsArgs {
    /// Page number of the top stories
    #[arg(short, long, default_value_t = 1)]
    page: u8,
}
