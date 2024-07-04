use anyhow::{Context, Error};
use chrono::{DateTime, Utc};
use serde::Deserialize;

const API_URL: &str = "https://crates.io/api/v1/crates/";
pub const API_INTERVAL: std::time::Duration = std::time::Duration::from_millis(1000);

pub struct CratesIOClient {
    client: reqwest::blocking::Client,
    url: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct CrateInfo {
    #[serde(rename = "crate")]
    _crate: Crate,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Crate {
    pub created_at: DateTime<Utc>,
    pub downloads: u64,
    pub name: String,
    pub updated_at: DateTime<Utc>,
    pub repository: String,
    pub versions: Vec<u64>,
}

impl CratesIOClient {
    pub fn with_user_agent(user_agent: String) -> Result<Self, Error> {
        let client = reqwest::blocking::Client::builder()
            .user_agent(user_agent)
            .build()
            .context("failed to build api client")?;
        Ok(Self {
            client,
            url: API_URL.to_string(),
        })
    }

    pub fn get(&self, crate_name: String) -> Result<Crate, Error> {
        let url = format!("{}{}", self.url, crate_name);

        let request = self
            .client
            .get(&url)
            .build()
            .with_context(|| format!("failed to build request for crate: {}", crate_name))?;

        let response = self
            .client
            .execute(request)
            .with_context(|| format!("failed to send request to: {}", url))?
            .error_for_status()
            .with_context(|| format!("invalid response from: {}", url))?;

        let mut crate_info: CrateInfo = serde_json::from_str(
            &response
                .text_with_charset("utf-8")
                .context("response contained invalid characters")?,
        )
        .with_context(|| format!("failed to deserialize response from: {}", url))?;

        // crates.io treats - and _ the same, set crate name to cargo tree name
        // so when appending we don't get the name again
        crate_info._crate.name = crate_name;

        Ok(crate_info._crate)
    }
}
