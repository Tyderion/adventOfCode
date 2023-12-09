#![feature(async_await, await_macro)]
use future::future;

use clap::{Parser, Subcommand};
use start_day::Config;

mod start_day;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Start new Day
    StartDay {
        /// Url to the input file (implies year & day to start)
        /// for example https://adventofcode.com/2023/day/8/input
        #[arg(short, long, value_parser = start_day::parse_day_config)]
        input: Config,
    },
}

pub fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::StartDay { input }) => start_day::start_day(input),
        None => future::empty(),
    }
}
