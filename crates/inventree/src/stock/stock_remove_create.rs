use crate::stock::{StockItemId, StockRepository};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveCreateBody {
    pub items: Vec<RemoveCreateInner>,
    pub notes: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveCreateInner {
    pub pk: StockItemId,
    /// API DOCS:
    /// This value is:
    ///
    /// string <decimal> ^-?\d{0,10}(?:\.\d{0,5})?$
    pub quantity: String,
}

#[derive(Serialize)]
struct RemoveCreateQuery;

impl StockRepository {
    pub async fn remove_create(&self, body: &RemoveCreateBody) -> anyhow::Result<serde_json::Value> {
        let res = self
            .0
            .post_request("stock/remove/", &body, &Some(RemoveCreateQuery))
            .await?;

        Ok(res)
    }
}
