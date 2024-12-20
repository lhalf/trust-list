use anyhow::{Context, Error};

pub struct GitHubClient {
    client: reqwest::blocking::Client,
}

impl GitHubClient {
    pub fn new() -> Result<Self, Error> {
        let client = reqwest::blocking::Client::builder()
            .build()
            .context("failed to build api client")?;
        Ok(Self { client })
    }

    pub fn get_contributor_count(&self, repo_url: String) -> Result<u16, Error> {
        let contributors_url = format!("{}/contributors", repo_url);

        match serde_json::from_str::<serde_json::Value>(&self.get(&contributors_url)?)
            .with_context(|| format!("failed to deserialize response from: {}", contributors_url))?
            .as_array()
        {
            Some(contributors) => Ok(contributors.len() as u16),
            None => Ok(0),
        }
    }

    fn get(&self, url: &str) -> Result<String, Error> {
        let request = self
            .client
            .get(url)
            .build()
            .with_context(|| format!("failed to build request to: {}", url))?;

        let response = self
            .client
            .execute(request)
            .with_context(|| format!("failed to send request to: {}", url))?
            .error_for_status()
            .with_context(|| format!("invalid response from: {}", url))?;

        response
            .text_with_charset("utf-8")
            .with_context(|| format!("response from {} contained invalid characters", url))
    }
}
