use crate::part::PartRepository;
use log::{debug, trace};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, ClientBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use stock::StockRepository;

mod location;
pub mod part;
pub mod stock;
pub mod supplier;

#[derive(Clone)]
pub struct InventreeApiClient {
    base_url: String,
    api_key: String,
    client: Client,
}

impl InventreeApiClient {
    pub fn new(base_url: &str, api_key: &str) -> Self {
        let mut header_map = HeaderMap::new();

        let auth_header = HeaderValue::from_str(&format!("Token {api_key}")).unwrap();
        header_map.insert("Authorization", auth_header);

        Self {
            base_url: base_url
                .to_string()
                .strip_suffix("/")
                .unwrap_or(&base_url)
                .to_string(),
            api_key: api_key.to_string(),
            client: ClientBuilder::new()
                .default_headers(header_map)
                .build()
                .unwrap(),
        }
    }

    pub fn part(&self) -> PartRepository {
        PartRepository(self.clone())
    }

    pub fn stock(&self) -> StockRepository {
        StockRepository(self.clone())
    }

    async fn get_request<T: DeserializeOwned + Debug, B: Serialize + Debug>(
        &self,
        endpoint: &str,
        query: &Option<B>,
    ) -> Result<T, anyhow::Error> {
        let x = format!("{}/api/{}", self.base_url, endpoint);

        let mut req = self.client.get(&x);

        if let Some(query) = query {
            req = req.query(&query);
        }

        trace!("GET Request to {x} with query: {query:?}");

        let res = req.send().await?.error_for_status()?.text().await?;

        debug!("API Response for {x}: {res}",);

        let res: T = serde_json::from_str(&res)?;
        Ok(res)
    }

    async fn post_request<T: DeserializeOwned + Debug, B: Serialize + Debug, P: Serialize>(
        &self,
        endpoint: &str,
        body: &B,
        query: &Option<P>,
    ) -> Result<T, anyhow::Error> {
        let x = format!("{}/api/{}", self.base_url, endpoint);

        let mut req = self.client.post(&x).json(body);

        if let Some(query) = query {
            req = req.query(&query);
        }

        trace!("POST Request to {x} with body: {body:?}");

        let res = req.send().await?.error_for_status()?.text().await?;

        debug!("API Response for {x}: {res}",);

        let res: T = serde_json::from_str(&res)?;
        Ok(res)
    }
}
