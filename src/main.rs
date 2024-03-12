use clap::Parser;
use wake_on_lan::Cli;

fn main() {
    let cli = Cli::parse();
    cli.run();
}
