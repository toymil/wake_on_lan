use clap::Parser;
use wake_on_lan::Config;

fn main() {
    let config = Config::parse();
    #[allow(clippy::unwrap_used)]
    config.wake().unwrap();
}
