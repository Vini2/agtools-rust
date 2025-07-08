# agtools: Tools for manipulating assembly graphs

`agtools` is a toolkit for manipulating assembly graphs, with a focus on metagenomic applications. It offers a command-line interface for tasks such as graph format conversion, segment filtering, and component extraction. Additionally, it provides a Python package interface that exposes assembler-specific functionality for advanced analysis and integration.

This is the version of [`agtools`](https://github.com/Vini2/agtools/) written in Rust. It's still under active development.

## Installing from source

If you don't have Rust, make sure to [install](https://www.rust-lang.org/tools/install) it.

Clone the GitHub repository and build `agtools` as follows.

```shell
git clone https://github.com/Vini2/agtools-rust.git
cd agtools-rust
cargo build release
```

You can find the newly built executable in `target/release/agtools`. You can add this to your system path.

## Available subcommands in `agtools`

Run `agtools --help` or `agtools -h` to list the help message for `agtools`.

```shell
agtools: tools for manipulating assembly graphs


Usage: agtools [COMMAND]

Available commands:
  stats      Compute statistics about the graph
  rename     Rename segments in a GFA file
  merge      Merge two or more GFA files
  filter     Filter segments from GFA file
  component  Extract a component containing a given segment
  fastg2gfa  Convert FASTG file to GFA format
  gfa2fastg  Convert GFA file to FASTG format
  gfa2dot    Convert GFA file to DOT format (Graphviz)
  gfa2fasta  Get segments in FASTA format
  gfa2adj    Get adjacency matrix of the assembly graph
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```