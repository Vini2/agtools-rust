use crate::args::StatsCommand;
use log::{debug, info};

pub fn run(cmd: &StatsCommand) {
    // Here is where you would add your real logic!
    info!("Obtaining statistics from the following GFA file: {}", cmd.graph);
    info!("Output will be placed in: {}", cmd.output);
    debug!("Stats logic would run here!");
}