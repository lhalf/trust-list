use anyhow::Error;
use std::collections::BTreeSet;

use crate::crates_io::{Crate, get_crate_info};
use crate::file_io::FileIO;
use crate::github::get_contributor_count;
use crate::http_client::GetRequest;

pub fn generate_list(
    mut crate_names: BTreeSet<String>,
    output_file: impl FileIO,
    http_client: impl GetRequest,
) -> Result<(), Error> {
    if !output_file.exists() {
        output_file.create()?;
        output_file.append(Crate::table_heading().as_bytes())?;
        output_file.append(Crate::table_divider().as_bytes())?;
    }

    crate_names = crate_names
        .difference(
            &output_file
                .read_to_string()?
                .split('\n')
                .skip(2)
                .flat_map(|line| line.split('|').skip(1).take(1).collect::<Vec<&str>>())
                .map(|crate_name| crate_name.trim().to_string())
                .collect::<BTreeSet<String>>(),
        )
        .cloned()
        .collect();

    for crate_name in crate_names {
        match get_crate_info(&http_client, &crate_name) {
            Ok(mut crate_info) => {
                crate_info.contributors =
                    get_contributor_count(&http_client, &crate_info.repository)?;

                output_file.append(crate_info.table_entry().as_bytes())?;

                println!("{}", crate_name);
            }
            Err(error) => {
                println!("failed to get info for {crate_name}: {error}");
            }
        }
    }

    Ok(())
}
