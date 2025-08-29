use crate::stock::{InventreeStockItem, StockItemId, StockRepository};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct StockRetrieveQuery {
    pub location_detail: Option<bool>,
    pub supplier_part_detail: Option<bool>,
    pub part_detail: Option<bool>,
}

impl Default for StockRetrieveQuery {
    fn default() -> Self {
        StockRetrieveQuery {
            location_detail: Some(true),
            supplier_part_detail: Some(true),
            part_detail: Some(true),
        }
    }
}

impl StockRepository {
    pub async fn retrieve(
        &self,
        stock_part_id: StockItemId,
        query: &Option<StockRetrieveQuery>,
    ) -> Result<InventreeStockItem, anyhow::Error> {
        let res: InventreeStockItem = self
            .0
            .get_request(&format!("stock/{}/", stock_part_id.0), query)
            .await?;
        Ok(res)
    }
}
