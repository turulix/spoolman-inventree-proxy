use crate::part::PartRepository;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, ClientBuilder};

pub mod part;

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
}
