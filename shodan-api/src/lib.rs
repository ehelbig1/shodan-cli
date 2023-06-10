use async_trait::async_trait;

mod error;
pub mod model;

#[async_trait]
pub trait Datasource {
    async fn search_ip(&self, ip: &str) -> Result<model::search_ip::Response, error::Error>;
}

pub struct ShodanApi<'a> {
    http_client: &'a reqwest::Client,
    api_key: String,
    base_url: String,
}

impl<'a> ShodanApi<'a> {
    pub fn new(http_client: &'a reqwest::Client, api_key: &str) -> Self {
        Self {
            http_client,
            api_key: api_key.to_string(),
            base_url: String::from("https://api.shodan.io"),
        }
    }
}

#[async_trait]
impl<'a> Datasource for ShodanApi<'a> {
    async fn search_ip(&self, ip: &str) -> Result<model::search_ip::Response, error::Error> {
        let url = format!(
            "{}/shodan/host/{}?key={}",
            &self.base_url, ip, &self.api_key
        );

        let response = self
            .http_client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(response)
    }
}
