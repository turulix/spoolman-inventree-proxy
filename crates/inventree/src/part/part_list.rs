use serde::Serialize;
use crate::part::{InventreePart, PartRepository};

#[derive(Serialize)]
pub struct PartListQuery {
    pub category: Option<u64>,
    pub parameters: Option<bool>
}

impl PartRepository {
    pub async fn list(&self, query: &PartListQuery) -> anyhow::Result<Vec<InventreePart>> {
        let url = format!("{}/api/part/", self.0.base_url);
        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header("Authorization", format!("Token {}", self.0.api_key))
            .query(query)
            .send()
            .await?
            .json::<Vec<InventreePart>>()
            .await?;
        Ok(response)
    }
}
