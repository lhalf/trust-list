use anyhow::Context;
use reqwest::blocking::Client;

pub const USER_AGENT: &str = "moc.kooltuo@tsil-tsurt";

#[cfg_attr(test, autospy::autospy)]
pub trait GetRequest {
    fn get(&self, url: &str) -> Result<String, anyhow::Error>;
}

impl GetRequest for Client {
    fn get(&self, url: &str) -> Result<String, anyhow::Error> {
        let request = self
            .get(url)
            .build()
            .with_context(|| format!("failed to build request to: {}", url))?;

        let response = self
            .execute(request)
            .with_context(|| format!("failed to send request to: {}", url))?
            .error_for_status()
            .with_context(|| format!("invalid response from: {}", url))?;

        response
            .text_with_charset("utf-8")
            .with_context(|| format!("response from {} contained invalid characters", url))
    }
}

pub fn build() -> anyhow::Result<Client> {
    Client::builder()
        .user_agent(USER_AGENT.chars().rev().collect::<String>())
        .build()
        .context("failed to build api client")
}
