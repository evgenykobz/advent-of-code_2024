use clap::{Parser, Subcommand};

mod historian_hysteria;

#[derive(Parser)]
#[command(name = "Advent Of Code 2024")]
#[command(about = "Daily puzzle solutions")]
struct Command {
    /// Which puzzle to solve
    #[command(subcommand)]
    puzzle: Puzzle,
}
#[derive(Subcommand)]
enum Puzzle {
    /// Day 1: Historian Hysteria
    HistorianHysteria {
        #[arg(default_value = "input.txt")]
        path: String,
    },
}
fn main() {
    match Command::parse().puzzle {
        Puzzle::HistorianHysteria { path } => historian_hysteria::solve(path),
    }
}
