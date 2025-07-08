use crate::args::MergeCommand;
use log::{debug, error, info, warn};

pub fn run(cmd: &MergeCommand) {
    // Here is where you would add your real logic!

    info!("Merging the following GFA files: {:?}", cmd.graph);
    info!("Output will be placed in: {}", cmd.output);
    debug!("Additional debug information can be logged here if needed.");
    warn!("Be cautious with the merge operation, ensure files do not have same segment ID.");
    info!("Merge logic would run here!");
    error!("Something went wrong");
}