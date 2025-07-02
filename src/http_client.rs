use anyhow::Context;

pub const USER_AGENT: &str = "moc.kooltuo@tsil-tsurt";

pub trait GetRequest {
    fn get(&self, url: &str) -> Result<String, anyhow::Error>;
}

impl GetRequest for reqwest::blocking::Client {
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
