use crate::part::{InventreePart, PartRepository};
use serde::Serialize;

#[derive(Serialize)]
pub struct PartListQuery {
    pub category: Option<u64>,
    pub parameters: Option<bool>,
}

impl PartRepository {
    pub async fn list(&self, query: &PartListQuery) -> anyhow::Result<Vec<InventreePart>> {
        let url = format!("{}/api/part/", self.0.base_url);
        let response = self
            .0
            .client
            .get(&url)
            .query(query)
            .send()
            .await?
            .json::<Vec<InventreePart>>()
            .await?;
        Ok(response)
    }
}
