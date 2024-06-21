use anyhow::Error;
use clap::Parser;

mod cargo_tree;
mod dependencies;
mod md_table;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Output file (.md)
    #[arg(short, long, default_value_t = String::from("dependencies"))]
    output_file: String,

    /// GitHub API Token
    #[arg(short, long, default_value_t = String::from(""))]
    api_token: String,
}

fn main() {
    let args = Args::parse();

    if let Err(error) = generate_table(args.output_file) {
        panic!("failed to generate trust list: {}", error)
    }
}

fn generate_table(output_file: String) -> Result<(), Error> {
    md_table::write(output_file, dependencies::get()?)
}
