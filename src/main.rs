use crate::file_io::OutputFile;
use crate::generate_list::generate_list;
use crate::http_client::USER_AGENT;
use anyhow::Context;
use clap::Parser;
use reqwest::blocking::Client;
use std::path::PathBuf;

mod cargo_tree;
mod crates_io;
mod file_io;
mod generate_list;
mod github;
mod http_client;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The output filename (appended with .md)
    #[arg(short, long, default_value_t = String::from("trust-list"))]
    output_file: String,

    /// Recreate table (appends new dependencies by default)
    #[arg(short, long)]
    recreate: bool,

    /// The depth of dependencies to collect information on (all sub-dependencies by default)
    #[arg(short, long)]
    depth: Option<u8>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let output_file = OutputFile::at(PathBuf::from(format!("{}.md", args.output_file)));
    let http_client = build_http_client()?;

    if let Err(error) = generate_list(args.recreate, args.depth, output_file, http_client) {
        panic!("failed to generate trust list: {:?}", error)
    }

    Ok(())
}

fn build_http_client() -> Result<Client, anyhow::Error> {
    Client::builder()
        .user_agent(USER_AGENT.chars().rev().collect::<String>())
        .build()
        .context("failed to build api client")
}
