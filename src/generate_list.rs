use anyhow::Error;

use crate::crates_io::get_crate_info;
use crate::github::get_contributor_count;
use crate::http_client::HTTPClient;
use crate::output_file::OutputFile;
use crate::{Args, cargo_tree};

pub fn generate_list(args: Args) -> Result<(), Error> {
    let output_file = OutputFile::at_path(&format!("{}.md", args.output_file));
    let mut crates_to_get = cargo_tree::crate_names(args.depth)?;
    let client = HTTPClient::new()?;

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

    for crate_name in crates_to_get {
        match get_crate_info(&client, &crate_name) {
            Ok(mut crate_info) => {
                crate_info.contributors = get_contributor_count(&client, &crate_info.repository)?;
                output_file.write_row(crate_info)?;
                println!("{}", crate_name);
            }
            Err(_) => {
                println!("failed to get crate info for: {}", crate_name);
            }
        }
    }

    Ok(())
}
