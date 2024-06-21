use anyhow::{Context, Error};
use serde::{Deserialize, Serialize};

use crate::cargo_tree;

pub type Dependencies = Vec<Dependency>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Dependency {
    pub name: String,
    pub link: String,
}

pub fn get() -> Result<Dependencies, Error> {
    match cargo_tree::output() {
        Ok(output) => Ok(to_dependencies(output)?),
        Err(error) => Err(anyhow::anyhow!("failed to call cargo tree: {}", error)),
    }
}

fn to_dependencies(output: String) -> Result<Dependencies, Error> {
    let dependencies: Dependencies = output
        .trim()
        .split("\n")
        .map(|json| {
            serde_json::from_str(json)
                .with_context(|| format!("could not deserialize a dependency: {}", json))
        })
        .collect::<Result<Dependencies, Error>>()?;

    Ok(dependencies)
}

#[cfg(test)]
mod tests {
    use super::{to_dependencies, Dependency};

    #[test]
    fn one_dependency() {
        let output = r#"{"name": "test", "link": "example.com"}"#.to_string();
        let expected_dependencies = vec![Dependency {
            name: "test".to_string(),
            link: "example.com".to_string(),
        }];
        assert_eq!(expected_dependencies, to_dependencies(output).unwrap())
    }
}
