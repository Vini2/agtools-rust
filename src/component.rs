use crate::args::ComponentCommand;
use log::{debug, info};

pub fn run(cmd: &ComponentCommand) {
    // Here is where you would add your real logic!
    info!("Obtaining component from the following GFA file: {}", cmd.graph);
    info!("Segment to use: {}", cmd.segment);
    info!("Output will be placed in: {}", cmd.output);
    debug!("Component logic would run here!");
}