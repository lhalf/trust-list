use std::process::Command;

use anyhow::{Context, Error};

// weird artefact of cargo tree
const EMPTY_LINE: &str = r#"{"name": "", "link": ""}"#;

pub fn output() -> Result<String, Error> {
    let output = Command::new("cargo")
        .args([
            "tree",
            "--format",
            r#"{{"name": "{lib}", "link": "{r}"}}"#,
            "--prefix",
            "none",
            "--no-dedupe",
        ])
        .output()
        .context("failed to call cargo tree")?;

    to_string(output.stdout)
}

fn to_string(stdout: Vec<u8>) -> Result<String, Error> {
    Ok(String::from_utf8(stdout)
        .context("cargo tree output contained invalid utf8")?
        .replace(EMPTY_LINE, ""))
}

#[cfg(test)]
mod tests {
    use super::output;

    #[test]
    fn can_call_cargo_tree() {
        assert!(output().is_ok())
    }
}
