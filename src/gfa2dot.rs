use crate::args::Gfa2dotCommand;
use log::{debug, info};

pub fn run(cmd: &Gfa2dotCommand) {
    // Here is where you would add your real logic!
    info!("Converting the following GFA file: {}", cmd.graph);
    info!("Output will be placed in: {}", cmd.output);
    debug!("gfa2dot logic would run here!");
}