use crate::args::Fastg2gfaCommand;

pub fn run(cmd: &Fastg2gfaCommand) {
    // Here is where you would add your real logic!
    println!("Converting the following FASTG file: {}", cmd.graph);
    println!("k-mer size to use: {}", cmd.k_size);
    println!("Output will be placed in: {}", cmd.output);
    println!("fastg2gfa logic would run here!");
}