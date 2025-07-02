use anyhow::Error;
use std::collections::BTreeSet;

use crate::cargo_tree;
use crate::crates_io::{Crate, get_crate_info};
use crate::file_io::FileIO;
use crate::github::get_contributor_count;
use crate::http_client::GetRequest;

pub fn generate_list(
    recreate: bool,
    depth: Option<u8>,
    output_file: impl FileIO,
    http_client: impl GetRequest,
) -> Result<(), Error> {
    let mut crates_to_get = cargo_tree::crate_names(depth)?;

    if recreate || !output_file.exists() {
        output_file.create()?;

        output_file.append(Crate::table_heading().as_bytes())?;
        output_file.append(Crate::table_divider().as_bytes())?;
    } else {
        if !output_file.exists() {
            return Err(anyhow::anyhow!("output file does not exist"));
        }

        crates_to_get = crates_to_get
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
            .collect()
    }

    if crates_to_get.is_empty() {
        return Ok(());
    }

    for crate_name in crates_to_get {
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
