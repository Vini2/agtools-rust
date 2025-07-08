use crate::args::Gfa2fastaCommand;
use log::{debug, info};

pub fn run(cmd: &Gfa2fastaCommand) {
    // Here is where you would add your real logic!
    info!("Converting the following GFA file: {}", cmd.graph);
    info!("Output will be placed in: {}", cmd.output);
    debug!("gfa2fasta logic would run here!");
}