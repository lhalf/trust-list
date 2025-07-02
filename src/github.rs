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

#[cfg(test)]
mod tests {
    use crate::github::get_contributor_count;
    use crate::http_client::GetRequestSpy;

    #[test]
    fn repo_url_with_invalid_base_url() {
        assert_eq!(
            "could not extract owner and name from repository url",
            get_contributor_count(&GetRequestSpy::default(), "http://invalid/url/user/repo")
                .unwrap_err()
                .to_string()
        )
    }

    #[test]
    fn fails_to_reach_contributor_url() {
        let spy = GetRequestSpy::default();

        spy.get
            .returns
            .push_back(Err(anyhow::anyhow!("deliberate test error")));

        assert_eq!(
            "deliberate test error",
            get_contributor_count(&spy, "https://github.com/cannot/reach")
                .unwrap_err()
                .to_string()
        )
    }

    #[test]
    fn contributor_url_returns_invalid_json() {
        let spy = GetRequestSpy::default();

        spy.get.returns.push_back(Ok("invalid JSON".to_string()));

        assert_eq!(
            "failed to deserialize response from: https://api.github.com/repos/invalid/json/contributors",
            get_contributor_count(&spy, "https://github.com/invalid/json")
                .unwrap_err()
                .to_string()
        )
    }
}
