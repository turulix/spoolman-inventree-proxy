mod find_vendor;

use crate::routes::vendor::find_vendor::find_vendor_route;
use chrono::DateTime;
use utoipa_actix_web::service_config::ServiceConfig;

pub fn configure_router(cfg: &mut ServiceConfig) {
    cfg.service(find_vendor_route);
}

#[derive(serde::Serialize, utoipa::ToSchema, Clone)]
pub struct Vendor {
    pub id: u64,
    pub registered: DateTime<chrono::Utc>,
    pub name: String,
    pub comment: Option<String>,
    pub empty_spool_weight: Option<f64>,
    pub external_id: Option<String>,
    pub extras: Option<serde_json::Value>,
}
