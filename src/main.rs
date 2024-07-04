use std::collections::BTreeSet;
use std::io::Stdout;

use anyhow::Error;
use clap::Parser;
use pbr::ProgressBar;

use crate::crates_io::{Crate, CratesIOClient};
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

    /// Crates.io user agent (name surname (example@email.com))
    #[arg(short, long)]
    user_agent: String,

    /// Recreate table
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
    let crates_to_get = cargo_tree::crate_names(args.depth)?;
    let client = CratesIOClient::with_user_agent(args.user_agent)?;

    if args.recreate || !output_file.exists() {
        write_new_table(output_file, client, crates_to_get)?;
    } else {
        append_to_table(output_file, client, crates_to_get)?;
    }

    Ok(())
}

fn write_new_table(
    output_file: OutputFile,
    client: CratesIOClient,
    crates_to_get: BTreeSet<String>,
) -> Result<(), Error> {
    let mut progress = ProgressBar::new(crates_to_get.len() as u64);
    progress.format("╢▌▌░╟");

    let crates = get_crate_info(client, crates_to_get, &mut progress)?;

    output_file.recreate()?;
    output_file.write_headings()?;
    output_file.write_md_table(crates)?;

    progress.finish_print(&output_file.path);

    Ok(())
}

fn append_to_table(
    output_file: OutputFile,
    client: CratesIOClient,
    crates_to_get: BTreeSet<String>,
) -> Result<(), Error> {
    let unchecked_crates = output_file.get_unchecked_crates(&crates_to_get)?;

    if unchecked_crates.is_empty() {
        println!("got all crates already");
        return Ok(());
    }

    let mut progress = ProgressBar::new(unchecked_crates.len() as u64);
    progress.format("╢▌▌░╟");

    let crates = get_crate_info(client, unchecked_crates, &mut progress)?;

    output_file.write_md_table(crates)?;

    progress.finish_print(&output_file.path);

    Ok(())
}

fn get_crate_info(
    client: CratesIOClient,
    crates_to_get: BTreeSet<String>,
    progress_bar: &mut ProgressBar<Stdout>,
) -> Result<Vec<Crate>, Error> {
    let mut crates = vec![];

    for crate_name in crates_to_get {
        progress_bar.message(&format!("{} ", crate_name));
        // currently ignore crates we can't find
        if let Ok(crate_info) = client.get(crate_name) {
            crates.push(crate_info)
        }
        progress_bar.inc();
        std::thread::sleep(crates_io::API_INTERVAL);
    }

    Ok(crates)
}
