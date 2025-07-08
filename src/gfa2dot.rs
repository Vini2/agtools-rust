use crate::args::Gfa2dotCommand;

pub fn run(cmd: &Gfa2dotCommand) {
    // Here is where you would add your real logic!
    println!("Converting the following GFA file: {}", cmd.graph);
    println!("Output will be placed in: {}", cmd.output);
    println!("gfa2dot logic would run here!");
}