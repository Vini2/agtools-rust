use crate::args::FilterCommand;
use log::{debug, info};

pub fn run(cmd: &FilterCommand) {
    // Here is where you would add your real logic!
    info!("Filtering segments from the following GFA file: {}", cmd.graph);
    info!("Minimum length of segments to keep: {}", cmd.min_length);
    info!("Output will be placed in: {}", cmd.output);
    debug!("Filter logic would run here!");
}