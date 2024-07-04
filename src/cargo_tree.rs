use std::collections::BTreeSet;
use std::process::Command;

use anyhow::{Context, Error};
use itertools::Itertools;

pub fn crate_names(depth: Option<u8>) -> Result<BTreeSet<String>, Error> {
    let output = Command::new("cargo")
        .args(&args(depth))
        .output()
        .context("failed to call cargo tree")?;

    Ok(to_crate_names(validate_output(output.stdout)?))
}

fn args(depth: Option<u8>) -> Vec<String> {
    let mut args = "tree --format {lib} --prefix none --no-dedupe"
        .split(" ")
        .map(String::from)
        .collect::<Vec<String>>();

    if let Some(depth) = depth {
        args.push("--depth".to_string());
        args.push(depth.to_string());
    }

    args
}

fn validate_output(stdout: Vec<u8>) -> Result<String, Error> {
    String::from_utf8(stdout).context("cargo tree output contained invalid utf8")
}

fn to_crate_names(output: String) -> BTreeSet<String> {
    output
        .trim()
        .split("\n")
        .unique()
        .sorted()
        .map(String::from)
        .collect()
}
