use std::collections::BTreeSet;

use anyhow::Error;
use clap::Parser;
use pbr::ProgressBar;

use crate::crates_io::CratesIOClient;
use crate::output_file::OutputFile;

mod cargo_tree;
mod crates_io;
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

    if let Err(error) = generate_trust_list(args) {
        panic!("failed to generate trust list: {:?}", error)
    }
}

fn generate_trust_list(args: Args) -> Result<(), Error> {
    let output_file = OutputFile::at_path(&format!("{}.md", args.output_file));
    let mut crates_to_get = cargo_tree::crate_names(args.depth)?;
    let client = CratesIOClient::new()?;

    if args.recreate || !output_file.exists() {
        output_file.recreate()?;
        output_file.write_headings()?;
    } else {
        crates_to_get = output_file.get_unchecked_crates(&crates_to_get)?;
    }

    if crates_to_get.is_empty() {
        println!("{}", output_file.path);
        return Ok(());
    }

    append_to_table(output_file, client, crates_to_get)?;

    Ok(())
}

fn append_to_table(
    output_file: OutputFile,
    client: CratesIOClient,
    crates_to_get: BTreeSet<String>,
) -> Result<(), Error> {
    let mut progress = progress_bar(crates_to_get.len() as u64);

    for crate_name in crates_to_get {
        progress.message(&format!("{:width$}", crate_name, width = 30));
        // currently ignore crates we can't find
        if let Ok(crate_info) = client.get(crate_name) {
            output_file.write_row(crate_info)?;
            progress.inc();
            std::thread::sleep(crates_io::API_INTERVAL);
        }
    }

    progress.finish_print(&output_file.path);

    Ok(())
}

fn progress_bar(length: u64) -> ProgressBar<std::io::Stdout> {
    let mut progress = ProgressBar::new(length);
    progress.format("╢▌▌░╟");
    progress.show_speed = false;
    progress.show_percent = false;
    progress
}