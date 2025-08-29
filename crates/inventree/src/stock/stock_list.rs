use crate::stock::{InventreeStockItem, StockRepository};
use serde::Serialize;

#[derive(Serialize, Debug, Clone, Default)]
pub struct StockListQuery {
    pub category: Option<u64>,
    pub location_detail: Option<bool>,
    pub supplier_part_detail: Option<bool>,
}

impl StockRepository {
    pub async fn list(
        &self,
        query: &Option<StockListQuery>,
    ) -> Result<Vec<InventreeStockItem>, anyhow::Error> {
        let res: Vec<InventreeStockItem> = self.0.get_request("stock/", query).await?;
        Ok(res)
    }
}
