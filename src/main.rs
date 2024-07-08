use clap::Parser;
use wake_on_lan::Cli;

fn main() {
    let cli = Cli::parse();
    #[allow(clippy::unwrap_used)]
    cli.run().unwrap();
}
