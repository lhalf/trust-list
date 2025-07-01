use anyhow::{Context, Error};
use std::collections::BTreeSet;
use std::io::Write;
use std::path::PathBuf;

use crate::cargo_tree;
use crate::crates_io::{Crate, get_crate_info};
use crate::github::get_contributor_count;
use crate::http_client::HTTPClient;

pub fn generate_list(filename: String, recreate: bool, depth: Option<u8>) -> Result<(), Error> {
    let output_filepath = PathBuf::from(format!("{filename}.md"));

    let mut crates_to_get = cargo_tree::crate_names(depth)?;

    let client = HTTPClient::new()?;

    if recreate || !output_filepath.exists() {
        std::fs::File::create(&output_filepath).context("could not create file")?;

        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(&output_filepath)
            .context("file does not exist")?;

        file.write_all(Crate::table_heading().as_bytes())
            .context("unable to write to output file")?;

        file.write_all(Crate::table_divider().as_bytes())
            .context("unable to write to output file")?;
    } else {
        if !output_filepath.exists() {
            return Err(anyhow::anyhow!("output file does not exist"));
        }

        crates_to_get = crates_to_get
            .difference(
                &std::fs::read_to_string(&output_filepath)
                    .context("failed to open output file")?
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
        println!("{:?}", output_filepath);
        return Ok(());
    }

    for crate_name in crates_to_get {
        match get_crate_info(&client, &crate_name) {
            Ok(mut crate_info) => {
                crate_info.contributors = get_contributor_count(&client, &crate_info.repository)?;

                let mut file = std::fs::OpenOptions::new()
                    .append(true)
                    .open(&output_filepath)
                    .context("file does not exist")?;

                file.write_all(crate_info.table_entry().as_bytes())
                    .with_context(|| {
                        format!("failed to write info for {} to file", crate_info.name)
                    })?;

                println!("{}", crate_name);
            }
            Err(error) => {
                println!("failed to get info for {crate_name}: {error}");
            }
        }
    }

    Ok(())
}
