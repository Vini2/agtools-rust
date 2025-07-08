use crate::args::RenameCommand;
use log::{debug, info};

pub fn run(cmd: &RenameCommand) {
    // Here is where you would add your real logic!
    info!("Renaming segments in the following GFA files: {}", cmd.graph);
    info!("Prefix used: {}", cmd.prefix);
    info!("Output will be placed in: {}", cmd.output);
    debug!("Rename logic would run here!");
}