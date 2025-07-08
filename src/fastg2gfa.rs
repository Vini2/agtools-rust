use crate::args::Fastg2gfaCommand;
use log::{debug, info};

pub fn run(cmd: &Fastg2gfaCommand) {
    // Here is where you would add your real logic!
    info!("Converting the following FASTG file: {}", cmd.graph);
    info!("k-mer size to use: {}", cmd.k_size);
    info!("Output will be placed in: {}", cmd.output);
    debug!("fastg2gfa logic would run here!");
}