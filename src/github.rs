use crate::http_client::GetRequest;
use anyhow::{Context, Error};

const API_BASE_URL: &str = "https://api.github.com/repos";

pub fn get_contributor_count(http_client: &impl GetRequest, repo_url: &str) -> Result<u16, Error> {
    let contributors_url = format!(
        "{API_BASE_URL}/{}/contributors",
        sanitise_repo_url(repo_url)?
    );

    match serde_json::from_str::<serde_json::Value>(&http_client.get(&contributors_url)?)
        .with_context(|| format!("failed to deserialize response from: {contributors_url}"))?
        .as_array()
    {
        Some(contributors) => Ok(contributors.len() as u16),
        None => Ok(0),
    }
}

fn sanitise_repo_url(repo_url: &str) -> anyhow::Result<&str> {
    let mut repo_url = repo_url
        .strip_prefix("https://github.com/")
        .context("could not extract owner and name from repository url")?;

    repo_url = repo_url.strip_suffix(".git").unwrap_or(repo_url);

    Ok(repo_url.strip_suffix("/").unwrap_or(repo_url))
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
            .set([Err(anyhow::anyhow!("deliberate test error"))]);

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

        spy.get.returns.set([Ok("invalid JSON".to_string())]);

        assert_eq!(
            "failed to deserialize response from: https://api.github.com/repos/invalid/json/contributors",
            get_contributor_count(&spy, "https://github.com/invalid/json")
                .unwrap_err()
                .to_string()
        )
    }

    #[test]
    fn contributor_url_returning_non_array_json_returns_0_contributors() {
        let spy = GetRequestSpy::default();

        spy.get.returns.set([Ok(
            r#"{"not_array": 10, "actually_object": 100}"#.to_string()
        )]);

        assert_eq!(
            0,
            get_contributor_count(&spy, "https://github.com/not/array").unwrap()
        )
    }

    #[test]
    fn contributor_url_returning_valid_json_array() {
        let spy = GetRequestSpy::default();

        spy.get.returns.set([Ok(r#"[1,2,3,4,5]"#.to_string())]);

        assert_eq!(
            5,
            get_contributor_count(&spy, "https://github.com/valid/array").unwrap()
        )
    }

    #[test]
    fn contributor_url_ending_with_git_returning_valid_json_array() {
        let spy = GetRequestSpy::default();

        spy.get.returns.set([Ok(r#"[1,2]"#.to_string())]);

        assert_eq!(
            2,
            get_contributor_count(&spy, "https://github.com/valid/repo.git").unwrap()
        )
    }

    #[test]
    fn contributor_url_ending_with_slash_returning_valid_json_array() {
        let spy = GetRequestSpy::default();

        spy.get.returns.set([Ok(r#"[1,2,3]"#.to_string())]);

        assert_eq!(
            3,
            get_contributor_count(&spy, "https://github.com/valid/repo/").unwrap()
        )
    }
}
