use clap::{Parser, Subcommand};

mod ceres_search;
mod historian_hysteria;
mod mull_it_over;
mod red_nosed_reports;

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
    /// Day 2: Red-Nosed Reports
    RedNosedReports {
        #[arg(default_value = "input.txt")]
        path: String,
    },
    /// Day 3: Mull It Over
    MullItOver {
        #[arg(default_value = "input.txt")]
        path: String,
    },
    /// Day 4: Ceres Search
    CeresSearch {
        #[arg(default_value = "input.txt")]
        path: String,
    },
}
fn main() {
    match Command::parse().puzzle {
        Puzzle::HistorianHysteria { path } => historian_hysteria::solve(path),
        Puzzle::RedNosedReports { path } => red_nosed_reports::solve(path),
        Puzzle::MullItOver { path } => mull_it_over::solve(path),
        Puzzle::CeresSearch { path } => ceres_search::solve(path),
    }
}
