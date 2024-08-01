use anyhow::{Context, Error};
use chrono::{DateTime, Utc};
use serde::Deserialize;

const API_URL: &str = "https://crates.io/api/v1/crates";
const USER_AGENT: &str = "moc.kooltuo@tsil-tsurt";
const API_INTERVAL: std::time::Duration = std::time::Duration::from_millis(1000);

pub struct CratesIOClient {
    client: reqwest::blocking::Client,
    api_url: String,
}

#[derive(Deserialize, Debug)]
pub struct CrateInfo {
    #[serde(rename = "crate")]
    _crate: Crate,
}

#[derive(Deserialize, Debug)]
pub struct Crate {
    pub created_at: DateTime<Utc>,
    pub downloads: u64,
    pub name: String,
    pub updated_at: DateTime<Utc>,
    pub repository: String,
    pub versions: Vec<u64>,
    #[serde(skip)]
    pub reverse_dependencies: u64,
}

#[derive(Deserialize, Debug)]
struct ReverseDependencies {
    meta: Meta,
}

#[derive(Deserialize, Debug)]
struct Meta {
    total: u64,
}

impl CratesIOClient {
    pub fn new() -> Result<Self, Error> {
        let client = reqwest::blocking::Client::builder()
            .user_agent(USER_AGENT.chars().rev().collect::<String>())
            .build()
            .context("failed to build api client")?;
        Ok(Self {
            client,
            api_url: API_URL.to_string(),
        })
    }

    pub fn get_crate_info(&self, crate_name: String) -> Result<Crate, Error> {
        let url = format!("{}/{}", self.api_url, &crate_name);

        let mut crate_info: CrateInfo = serde_json::from_str(&self.get(&url)?)
            .with_context(|| format!("failed to deserialize response from: {}", url))?;

        // crates.io treats - and _ the same, set crate name to cargo tree name
        // so when appending we don't get the name again
        crate_info._crate.name = crate_name.clone();

        crate_info._crate.reverse_dependencies = self
            .get_reverse_dependencies(&crate_name)
            .with_context(|| format!("failed to get reverse dependencies for {}", crate_name))?;

        Ok(crate_info._crate)
    }

    fn get_reverse_dependencies(&self, crate_name: &str) -> Result<u64, Error> {
        let url = format!("{}/{}/reverse_dependencies", self.api_url, crate_name);

        let reverse_dependencies: ReverseDependencies = serde_json::from_str(&self.get(&url)?)
            .with_context(|| format!("failed to deserialize response from: {}", url))?;

        Ok(reverse_dependencies.meta.total)
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

        std::thread::sleep(API_INTERVAL);

        response
            .text_with_charset("utf-8")
            .with_context(|| format!("response from {} contained invalid characters", url))
    }
}
