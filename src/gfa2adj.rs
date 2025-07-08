use crate::args::Gfa2adjCommand;
use log::{debug, info};

pub fn run(cmd: &Gfa2adjCommand) {
    // Here is where you would add your real logic!
    info!("Converting the following GFA file: {}", cmd.graph);
    info!("Output will be placed in: {}", cmd.output);
    debug!("gfa2adj logic would run here!");
}