use crate::args::RenameCommand;

pub fn run(cmd: &RenameCommand) {
    // Here is where you would add your real logic!
    println!("Renaming segments in the following GFA files: {}", cmd.graph);
    println!("Prefix used: {}", cmd.prefix);
    println!("Output will be placed in: {}", cmd.output);
    println!("Rename logic would run here!");
}