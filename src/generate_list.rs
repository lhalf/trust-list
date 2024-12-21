use std::collections::BTreeSet;

use anyhow::Error;
use pbr::ProgressBar;

use crate::{Args, cargo_tree};
use crate::crates_io::get_crate_info;
use crate::http_client::HTTPClient;
use crate::output_file::OutputFile;

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

    append_list(output_file, client, crates_to_get)?;

    Ok(())
}

fn append_list(
    output_file: OutputFile,
    client: HTTPClient,
    crates_to_get: BTreeSet<String>,
) -> Result<(), Error> {
    let mut progress = progress_bar(crates_to_get.len() as u64);

    for crate_name in crates_to_get {
        progress.message(&format!("{:width$}", crate_name, width = 30));

        // currently ignore crates we can't find
        match get_crate_info(&client, crate_name) {
            Ok(crate_info) => output_file.write_row(crate_info)?,
            Err(error) => progress.message(&format!("{} ", error)),
        }

        progress.inc();
    }

    progress.finish_print(&output_file.path);

    Ok(())
}

fn progress_bar(length: u64) -> ProgressBar<std::io::Stdout> {
    let mut progress = ProgressBar::new(length);
    progress.format("╢▌▌░╟");
    progress.show_speed = false;
    progress.show_percent = false;
    progress
}
