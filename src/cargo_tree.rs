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
        .split(' ')
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
        .split('\n')
        .skip(1) // removes own library name from output
        .unique()
        .filter(|crate_name| !crate_name.is_empty())
        .sorted()
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod test {
    use std::collections::BTreeSet;

    use super::crate_names;

    #[test]
    fn cargo_tree_depth_1() {
        // could get this from cargo.toml?
        let expected_crates = BTreeSet::from([
            "anyhow".to_string(),
            "chrono".to_string(),
            "clap".to_string(),
            "itertools".to_string(),
            "pbr".to_string(),
            "reqwest".to_string(),
            "serde".to_string(),
            "serde_json".to_string(),
        ]);
        assert_eq!(expected_crates, crate_names(Some(1)).unwrap());
    }
}
