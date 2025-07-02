use crate::crates_io::{Crate, get_crate_info};
use crate::file_io::FileIO;
use crate::github::get_contributor_count;
use crate::http_client::GetRequest;
use anyhow::Error;
use std::collections::BTreeSet;

pub fn generate_list(
    crate_names: BTreeSet<String>,
    output_file: impl FileIO,
    http_client: impl GetRequest,
) -> Result<(), Error> {
    if !output_file.exists() {
        output_file.create()?;
        output_file.append(Crate::table_heading().as_bytes())?;
        output_file.append(Crate::table_divider().as_bytes())?;
    }

    let existing_names = parse_existing_crate_names(&output_file.read_to_string()?);
    let missing_names = crate_names.difference(&existing_names);

    for crate_name in missing_names {
        match get_crate_info(&http_client, crate_name) {
            Ok(mut crate_info) => {
                crate_info.contributors =
                    get_contributor_count(&http_client, &crate_info.repository)?;
                output_file.append(crate_info.table_entry().as_bytes())?;
                println!("{crate_name}");
            }
            Err(error) => {
                println!("failed to get info for {crate_name}: {error}");
            }
        }
    }
    Ok(())
}

fn parse_existing_crate_names(contents: &str) -> BTreeSet<String> {
    contents
        .lines()
        .skip(2)
        .filter_map(|line| line.split('|').nth(1))
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}
