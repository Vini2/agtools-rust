use crate::args::Gfa2fastgCommand;

pub fn run(cmd: &Gfa2fastgCommand) {
    // Here is where you would add your real logic!
    println!("Converting the following GFA file: {:?}", cmd.graph);
    println!("Output will be placed in: {}", cmd.output);
    println!("gfa2fastg logic would run here!");
}