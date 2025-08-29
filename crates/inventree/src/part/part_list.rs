use crate::part::{InventreePart, PartRepository};
use serde::Serialize;

#[derive(Serialize, Default, Debug)]
pub struct PartListQuery {
    pub category: Option<u64>,
    pub parameters: Option<bool>,
}

impl PartRepository {
    pub async fn list(&self, query: &Option<PartListQuery>) -> anyhow::Result<Vec<InventreePart>> {
        let res = self.0.get_request("part/", &query).await?;
        Ok(res)
    }
}
