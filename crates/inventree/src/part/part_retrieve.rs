use crate::part::{InventreePart, PartId, PartRepository};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct PartRetrieveQuery {
    pub parameters: Option<bool>,
}

impl Default for PartRetrieveQuery {
    fn default() -> Self {
        Self {
            parameters: Some(true),
        }
    }
}

impl PartRepository {
    pub async fn retrieve(
        &self,
        part_id: PartId,
        query: &Option<PartRetrieveQuery>,
    ) -> anyhow::Result<InventreePart> {
        let res = self
            .0
            .get_request(&format!("part/{}/", part_id.0), &query)
            .await?;
        Ok(res)
    }
}
