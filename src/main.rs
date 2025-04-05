use clap::Parser;
use cli::CLI;

mod cli;
mod eui;
mod ipv6;

fn main() {
    let cli = CLI::parse();
    cli.run();
}
