use std::collections::HashSet;
use std::process::Command;

use anyhow::{Context, Error};
use itertools::Itertools;

pub fn crate_names(depth: u8) -> Result<HashSet<String>, Error> {
    let output = Command::new("cargo")
        .args([
            "tree",
            "--depth",
            &depth.to_string(),
            "--format",
            "{lib}",
            "--prefix",
            "none",
            "--no-dedupe",
        ])
        .output()
        .context("failed to call cargo tree")?;

    Ok(to_crate_names(validate_output(output.stdout)?))
}

fn validate_output(stdout: Vec<u8>) -> Result<String, Error> {
    String::from_utf8(stdout).context("cargo tree output contained invalid utf8")
}

fn to_crate_names(output: String) -> HashSet<String> {
    output
        .trim()
        .split("\n")
        .unique()
        .sorted()
        .map(String::from)
        .collect()
}
