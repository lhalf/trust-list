use clap::Parser;

use crate::generate_list::generate_list;

mod cargo_tree;
mod crates_io;
mod generate_list;
mod github;
mod output_file;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Output file (.md)
    #[arg(short, long, default_value_t = String::from("trust-list"))]
    output_file: String,

    /// Recreate table (appends new dependencies by default)
    #[arg(short, long)]
    recreate: bool,

    /// Dependency depth
    #[arg(short, long)]
    depth: Option<u8>,
}

fn main() {
    let args = Args::parse();

    if let Err(error) = generate_list(args) {
        panic!("failed to generate trust list: {:?}", error)
    }
}
