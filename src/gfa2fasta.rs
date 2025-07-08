use crate::args::Gfa2fastaCommand;

pub fn run(cmd: &Gfa2fastaCommand) {
    // Here is where you would add your real logic!
    println!("Converting the following GFA file: {:?}", cmd.graph);
    println!("Output will be placed in: {}", cmd.output);
    println!("gfa2fasta logic would run here!");
}