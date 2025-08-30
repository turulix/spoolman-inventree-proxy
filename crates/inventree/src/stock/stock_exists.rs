use crate::stock::{StockItemId, StockRepository};

impl StockRepository {
    pub async fn exists(&self, stock_part_id: StockItemId) -> Result<bool, anyhow::Error> {
        let url = format!("{}/api/stock/{}/", self.0.base_url, stock_part_id.0);
        let res = self.0.client.get(url).send().await?;

        if res.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(false);
        }
        res.error_for_status_ref()?;

        Ok(true)
    }
}
