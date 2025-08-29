use chrono::DateTime;

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct Vendor {
    pub id: u64,
    pub registered: DateTime<chrono::Utc>,
    pub name: String,
    pub comment: Option<String>,
    pub empty_spool_weight: Option<f64>,
    pub external_id: Option<String>,
    pub extras: Option<serde_json::Value>,
}