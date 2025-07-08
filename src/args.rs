use std::path::Component;

use clap::{Parser, Subcommand, Args};

const ABOUT: &str = "agtools: tools for manipulating assembly graphs

a toolkit for manipulating assembly graphs, 
with a focus on metagenomic applications";

#[derive(Parser)]
#[command(version, about, long_about = ABOUT, subcommand_help_heading = "Available commands")]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Compute statistics about the graph
    Stats(StatsCommand),
    /// Rename segments in a GFA file
    Rename(RenameCommand),
    /// Merge two or more GFA files
    Merge(MergeCommand),
    /// Filter segments from GFA file
    Filter(FilterCommand),
    /// Extract a component containing a given segment
    Component(ComponentCommand),
    /// Convert FASTG file to GFA format
    Fastg2gfa(Fastg2gfaCommand),
    /// Convert GFA file to FASTG format
    Gfa2fastg(Gfa2fastgCommand),
    /// Convert GFA file to DOT format (Graphviz)
    Gfa2dot(Gfa2dotCommand),
    /// Get segments in FASTA format
    Gfa2fasta(Gfa2fastaCommand),
    /// Get adjacency matrix of the assembly graph
    Gfa2adj(Gfa2adjCommand),
}

#[derive(Args)]
pub struct StatsCommand {
    /// Path to the assembly graph file
    #[arg(short, long)]
    pub graph: String,

    /// Path to the output folder
    #[arg(short, long)]
    pub output: String,
}

#[derive(Args)]
pub struct RenameCommand {
    /// Path to the assembly graph file
    #[arg(short, long)]
    pub graph: String,

    /// Prefix for the segment names
    /// This will be prepended to each segment name
    #[arg(short, long, verbatim_doc_comment, default_value = "")]
    pub prefix: String,

    /// Path to the output folder
    #[arg(short, long)]
    pub output: String,
}

#[derive(Args)]
pub struct MergeCommand {
    /// Paths to the assembly graph files
    #[arg(short, long, num_args = 1..,)]
    pub graph: Vec<String>,

    /// Path to the output folder
    #[arg(short, long)]
    pub output: String,
}

#[derive(Args)]
pub struct FilterCommand {
    /// Path to the assembly graph file
    #[arg(short, long)]
    pub graph: String,

    /// Minimum length of segments to keep
    #[arg(short, long, value_parser = clap::value_parser!(u64).range(1..), verbatim_doc_comment, default_value_t = 1000)]
    pub min_length: u64,

    /// Path to the output folder
    #[arg(short, long)]
    pub output: String,
}

#[derive(Args)]
pub struct ComponentCommand {
    /// Path to the assembly graph file
    #[arg(short, long)]
    pub graph: String,

    /// Segment ID
    #[arg(short, long)]
    pub segment: String,

    /// Path to the output folder
    #[arg(short, long)]
    pub output: String,
}

#[derive(Args)]
pub struct Fastg2gfaCommand {
    /// Path to the assembly graph file
    #[arg(short, long)]
    pub graph: String,

    /// k-mer size used for the assembly
    #[arg(short, long, value_parser = clap::value_parser!(u64).range(1..), verbatim_doc_comment, default_value_t = 141)]
    pub k_size: u64,

    /// Path to the output folder
    #[arg(short, long)]
    pub output: String,
}

#[derive(Args)]
pub struct Gfa2fastgCommand {
    /// Paths to the assembly graph file
    #[arg(short, long)]
    pub graph: String,

    /// Path to the output folder
    #[arg(short, long)]
    pub output: String,
}

#[derive(Args)]
pub struct Gfa2dotCommand {
    /// Paths to the assembly graph file
    #[arg(short, long)]
    pub graph: String,

    /// Path to the output folder
    #[arg(short, long)]
    pub output: String,
}

#[derive(Args)]
pub struct Gfa2fastaCommand {
    /// Paths to the assembly graph file
    #[arg(short, long)]
    pub graph: String,

    /// Path to the output folder
    #[arg(short, long)]
    pub output: String,
}

#[derive(Args)]
pub struct Gfa2adjCommand {
    /// Paths to the assembly graph file
    #[arg(short, long)]
    pub graph: String,

    /// Path to the output folder
    #[arg(short, long)]
    pub output: String,
}