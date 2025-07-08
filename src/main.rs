// use crate::args::Cli;
use crate::args::{Cli, Commands};
use clap::Parser;
mod args;
mod merge;
mod rename;

fn main() {
    let cli = Cli::parse();
    
    match &cli.commands {
        Some(Commands::Merge(cmd)) => merge::run(cmd),
        Some(Commands::Rename(cmd)) => rename::run(cmd),
        // ... other command arms ...
        _ => {
            println!("No command given or command not implemented yet.");
        }
    }
}
