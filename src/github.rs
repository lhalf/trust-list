use crate::http_client::GetRequest;
use anyhow::{Context, Error};

const API_BASE_URL: &str = "https://api.github.com/repos";

pub fn get_contributor_count(http_client: &impl GetRequest, repo_url: &str) -> Result<u16, Error> {
    let owner_and_name = match repo_url.strip_prefix("https://github.com/") {
        None => {
            return Err(anyhow::anyhow!(
                "could not extract owner and name from repository url"
            ));
        }
        Some(owner_and_name) => owner_and_name,
    };

    let contributors_url = format!("{}/{}/contributors", API_BASE_URL, owner_and_name);

    match serde_json::from_str::<serde_json::Value>(&http_client.get(&contributors_url)?)
        .with_context(|| format!("failed to deserialize response from: {}", contributors_url))?
        .as_array()
    {
        Some(contributors) => Ok(contributors.len() as u16),
        None => Ok(0),
    }
}
