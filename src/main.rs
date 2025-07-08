// File: src/main.rs
use crate::args::{Cli, Commands};
use clap::Parser;
mod args;
mod stats;
mod rename;
mod merge;
mod filter;
mod component;
mod fastg2gfa;
mod gfa2fastg;
mod gfa2dot;
mod gfa2fasta;
mod gfa2adj;

fn main() {
    let cli = Cli::parse();
    
    match &cli.commands {
        Some(Commands::Stats(cmd)) => stats::run(cmd),
        Some(Commands::Rename(cmd)) => rename::run(cmd),
        Some(Commands::Merge(cmd)) => merge::run(cmd),
        Some(Commands::Filter(cmd)) => filter::run(cmd),
        Some(Commands::Component(cmd)) => component::run(cmd),
        Some(Commands::Fastg2gfa(cmd)) => fastg2gfa::run(cmd),
        Some(Commands::Gfa2fastg(cmd)) => gfa2fastg::run(cmd),
        Some(Commands::Gfa2dot(cmd)) => gfa2dot::run(cmd),
        Some(Commands::Gfa2fasta(cmd)) => gfa2fasta::run(cmd),
        Some(Commands::Gfa2adj(cmd)) => gfa2adj::run(cmd),
        _ => {
            println!("No command given. Please provide a command to run.");
            println!("Use --help to see available commands and options.");
            std::process::exit(1);
        }
    }
}
