use anyhow::Context;
use reqwest::blocking::Client;

const USER_AGENT: &str = "moc.kooltuo@tsil-tsurt";

pub struct HTTPClient {
    client: Client,
}

impl HTTPClient {
    pub fn new() -> Result<Self, anyhow::Error> {
        let client = Client::builder()
            .user_agent(USER_AGENT.chars().rev().collect::<String>())
            .build()
            .context("failed to build api client")?;
        Ok(Self { client })
    }

    pub fn get(&self, url: &str) -> Result<String, anyhow::Error> {
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
