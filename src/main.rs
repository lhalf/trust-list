use crate::file_io::OutputFile;
use crate::generate_list::generate_list;
use clap::Parser;
use std::path::PathBuf;

mod cargo_tree;
mod crates_io;
mod file_io;
mod generate_list;
mod github;
mod http_client;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The output filename, appended with .md
    #[arg(short, long, default_value_t = String::from("trust-list"))]
    output_file: String,

    /// Recreate table [default: appends new dependencies]
    #[arg(short, long)]
    recreate: bool,

    /// The depth of dependencies to collect information on [default: all sub dependencies]
    #[arg(short = 'D', long)]
    depth: Option<u8>,

    /// Include dev dependencies [default: excluded]
    #[arg(short, long)]
    dev: bool,

    /// Include build dependencies [default: excluded]
    #[arg(short, long)]
    build: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let output_file = OutputFile::new(
        PathBuf::from(format!("{}.md", args.output_file)),
        args.recreate,
    )?;

    let http_client = http_client::build()?;

    let crates_names = cargo_tree::crate_names(args.depth, args.dev, args.build)?;

    if let Err(error) = generate_list(crates_names, output_file, http_client) {
        panic!("failed to generate trust list: {:?}", error)
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Args;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Args::command().debug_assert();
    }
}
