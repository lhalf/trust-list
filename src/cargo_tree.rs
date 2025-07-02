use std::collections::BTreeSet;
use std::process::Command;

use anyhow::Context;
use itertools::Itertools;

pub fn crate_names(
    depth: Option<u8>,
    include_dev_dependencies: bool,
    include_build_dependencies: bool,
) -> anyhow::Result<BTreeSet<String>> {
    let output = Command::new("cargo")
        .args(args(
            depth,
            include_dev_dependencies,
            include_build_dependencies,
        ))
        .output()
        .context("failed to call cargo tree")?;
    Ok(to_crate_names(validate_output(output.stdout)?))
}

fn args(
    depth: Option<u8>,
    include_dev_dependencies: bool,
    include_build_dependencies: bool,
) -> Vec<String> {
    let mut args = vec![
        "tree".to_string(),
        "--format".to_string(),
        "{lib}".to_string(),
        "--prefix".to_string(),
        "none".to_string(),
        "--no-dedupe".to_string(),
    ];

    let mut edges = Vec::new();
    if !include_dev_dependencies {
        edges.push("no-dev");
    }
    if !include_build_dependencies {
        edges.push("no-build");
    }
    if !edges.is_empty() {
        args.push("--edges".to_string());
        args.push(edges.join(","));
    }

    if let Some(depth) = depth {
        args.push("--depth".to_string());
        args.push(depth.to_string());
    }

    args
}

fn validate_output(stdout: Vec<u8>) -> anyhow::Result<String> {
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
            "field_names".to_string(),
            "itertools".to_string(),
            "reqwest".to_string(),
            "serde".to_string(),
            "serde_json".to_string(),
        ]);
        assert_eq!(expected_crates, crate_names(Some(1), false, false).unwrap());
    }

    #[test]
    fn cargo_tree_depth_1_with_dev() {
        // could get this from cargo.toml?
        let expected_crates = BTreeSet::from([
            "anyhow".to_string(),
            "autospy".to_string(),
            "chrono".to_string(),
            "clap".to_string(),
            "field_names".to_string(),
            "itertools".to_string(),
            "reqwest".to_string(),
            "serde".to_string(),
            "serde_json".to_string(),
        ]);
        assert_eq!(expected_crates, crate_names(Some(1), true, false).unwrap());
    }
}
