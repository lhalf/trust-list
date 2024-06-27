use anyhow::{Context, Error};
use chrono::{DateTime, Utc};
use serde::Deserialize;

const API_URL: &str = "https://crates.io/api/v1/crates/";

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
        let request = self
            .client
            .get(format!("{}{}", self.url, crate_name))
            .build()
            .with_context(|| format!("failed to build request for crate: {}", crate_name))?;

        let response = self
            .client
            .execute(request)
            .context("failed to send request")?;

        let crate_info: CrateInfo = serde_json::from_str(
            &response
                .text_with_charset("utf-8")
                .context("response contained invalid characters")?,
        )
        .context("failed to deserialize response as json")?;

        Ok(crate_info._crate)
    }
}
