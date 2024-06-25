use anyhow::Error;
use clap::Parser;
use pbr::ProgressBar;

mod cargo_tree;
mod crate_info;
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

    /// Crates.io user agent
    #[arg(short, long)]
    user_agent: String,
}

fn main() {
    let args = Args::parse();

    if let Err(error) = generate_table(args.user_agent, args.output_file) {
        panic!("failed to generate trust list: {}", error)
    }
}

fn generate_table(user_agent: String, output_file: String) -> Result<(), Error> {
    let client = crate_info::CratesIOClient::with_user_agent(user_agent)?;

    let crate_names = cargo_tree::crate_names(1)?;

    let mut progress = ProgressBar::new(crate_names.len() as u64);
    progress.format("╢▌▌░╟");

    let mut crates = vec![];

    for crate_name in crate_names {
        progress.message(&format!("{} ", crate_name));
        crates.push(client.get(crate_name)?);
        progress.inc();
        std::thread::sleep(std::time::Duration::from_millis(800));
    }

    md_table::write(&output_file, crates)?;

    progress.finish_print(&format!("{}.md", output_file));

    Ok(())
}
