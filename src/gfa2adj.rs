use crate::args::Gfa2adjCommand;

pub fn run(cmd: &Gfa2adjCommand) {
    // Here is where you would add your real logic!
    println!("Converting the following GFA file: {:?}", cmd.graph);
    println!("Output will be placed in: {}", cmd.output);
    println!("gfa2adj logic would run here!");
}