use std::process::Command;

use anyhow::{Context, Error};
use itertools::Itertools;

pub fn crate_names() -> Result<Vec<String>, Error> {
    let output = Command::new("cargo")
        .args([
            "tree",
            "--format",
            "{lib}",
            "--prefix",
            "none",
            "--no-dedupe",
        ])
        .output()
        .context("failed to call cargo tree")?;

    Ok(to_crate_list(validate_output(output.stdout)?))
}

fn validate_output(stdout: Vec<u8>) -> Result<String, Error> {
    String::from_utf8(stdout).context("cargo tree output contained invalid utf8")
}

fn to_crate_list(output: String) -> Vec<String> {
    output
        .trim()
        .split("\n")
        .unique()
        .sorted()
        .map(String::from)
        .collect()
}
