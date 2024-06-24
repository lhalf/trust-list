use anyhow::Error;
use clap::Parser;

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

    md_table::write(output_file, vec![client.get("anyhow".to_string())?])?;

    Ok(())
}
