use crate::http_client::GetRequest;
use anyhow::{Context, Error};
use chrono::{DateTime, Utc};
use field_names::FieldNames;
use serde::Deserialize;

const API_URL: &str = "https://crates.io/api/v1/crates";
#[derive(Deserialize, Debug)]
pub struct CrateInfo {
    #[serde(rename = "crate")]
    _crate: Crate,
}

#[derive(Deserialize, Debug, FieldNames)]
pub struct Crate {
    pub name: String,
    pub downloads: u64,
    #[serde(skip)]
    pub contributors: u16,
    #[serde(skip)]
    pub reverse_dependencies: u64,
    pub versions: Vec<u64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub repository: String,
}

impl Crate {
    pub fn table_heading() -> String {
        ["|", &Self::FIELDS.join("|"), "|\n"].join("")
    }

    pub fn table_divider() -> String {
        ["|", &vec!["-"; Self::FIELDS.len()].join("|"), "|\n"].join("")
    }

    pub fn table_entry(&self) -> String {
        format!(
            "|{}|{}|{}|{}|{}|{}|{}|{}|\n",
            self.name,
            self.downloads,
            if self.contributors == 30 {
                "30+".to_string()
            } else {
                self.contributors.to_string()
            },
            self.reverse_dependencies,
            self.versions.len(),
            self.created_at.format("%d/%m/%Y"),
            self.updated_at.format("%d/%m/%Y"),
            self.repository
        )
    }
}

#[derive(Deserialize, Debug)]
struct ReverseDependencies {
    meta: Meta,
}

#[derive(Deserialize, Debug)]
struct Meta {
    total: u64,
}

pub fn get_crate_info(http_client: &impl GetRequest, crate_name: &str) -> Result<Crate, Error> {
    // crates.io api policy - https://crates.io/data-access#api
    #[cfg(not(test))]
    std::thread::sleep(std::time::Duration::from_secs(1));

    let url = format!("{}/{}", API_URL, &crate_name);

    let mut crate_info: CrateInfo = serde_json::from_str(&http_client.get(&url)?)
        .with_context(|| format!("failed to deserialize response from: {}", url))?;

    // crates.io treats - and _ the same, set crate name to cargo tree name
    // so when appending we don't get the name again
    crate_info._crate.name = crate_name.to_string();

    crate_info._crate.reverse_dependencies = get_reverse_dependencies(http_client, crate_name)
        .with_context(|| format!("failed to get reverse dependencies for {}", crate_name))?;

    Ok(crate_info._crate)
}

fn get_reverse_dependencies(http_client: &impl GetRequest, crate_name: &str) -> Result<u64, Error> {
    let url = format!("{}/{}/reverse_dependencies", API_URL, crate_name);

    let reverse_dependencies: ReverseDependencies =
        serde_json::from_str(&http_client.get(&url)?)
            .with_context(|| format!("failed to deserialize response from: {}", url))?;

    Ok(reverse_dependencies.meta.total)
}

#[cfg(test)]
mod tests {
    use crate::crates_io::{Crate, get_crate_info};
    use crate::http_client::GetRequestSpy;

    #[test]
    fn crate_produces_expected_table_headings() {
        assert_eq!(
            "|name|downloads|contributors|reverse_dependencies|versions|created_at|updated_at|repository|\n",
            Crate::table_heading()
        )
    }

    #[test]
    fn crate_produces_expected_table_divider() {
        assert_eq!("|-|-|-|-|-|-|-|-|\n", Crate::table_divider())
    }

    #[test]
    fn crate_produces_expected_table_line() {
        assert_eq!(
            "|example|100|20|10|2|01/01/1970|01/01/1970|https://github.com/lhalf/trust-list-rs|\n",
            Crate {
                name: "example".to_string(),
                downloads: 100,
                contributors: 20,
                reverse_dependencies: 10,
                versions: vec![0, 1],
                created_at: Default::default(),
                updated_at: Default::default(),
                repository: "https://github.com/lhalf/trust-list-rs".to_string(),
            }
            .table_entry()
        )
    }

    #[test]
    fn fails_to_reach_crate_url() {
        let spy = GetRequestSpy::default();

        spy.get
            .returns
            .push_back(Err(anyhow::anyhow!("deliberate test error")));

        assert_eq!(
            "deliberate test error",
            get_crate_info(&spy, "invalid").unwrap_err().to_string()
        )
    }
}
