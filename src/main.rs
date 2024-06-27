use anyhow::Error;
use clap::Parser;
use pbr::ProgressBar;

use output_file::OutputFile;

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
}

fn main() {
    let args = Args::parse();

    if let Err(error) = generate_trust_list(args.user_agent, args.output_file) {
        panic!("failed to generate trust list: {:?}", error)
    }
}

fn generate_trust_list(user_agent: String, output_file: String) -> Result<(), Error> {
    let client = crates_io::CratesIOClient::with_user_agent(user_agent)?;

    let crate_names = cargo_tree::crate_names(1)?;

    let mut progress = ProgressBar::new(crate_names.len() as u64);
    progress.format("╢▌▌░╟");

    let mut crates = vec![];

    for crate_name in crate_names {
        progress.message(&format!("{} ", crate_name));
        crates.push(client.get(crate_name)?);
        progress.inc();
        std::thread::sleep(crates_io::API_INTERVAL);
    }

    OutputFile::at_path(&output_file).write_md_table(crates)?;

    progress.finish_print(&format!("{}.md", output_file));

    Ok(())
}
