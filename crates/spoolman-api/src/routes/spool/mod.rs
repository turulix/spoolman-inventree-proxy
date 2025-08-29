mod get;

use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_actix_web::service_config::ServiceConfig;
use crate::routes::filament::Filament;
use crate::routes::spool::get::find_spool_route;

pub fn configure_router(cfg: &mut ServiceConfig) {
    cfg.service(find_spool_route);
}

#[derive(Serialize, ToSchema)]
pub struct Spool {
    id: u64,
    registered: DateTime<Utc>,
    first_used: Option<DateTime<Utc>>,
    last_used: Option<DateTime<Utc>>,
    filament: Filament,
    price: Option<f64>,
    remaining_weight: Option<f64>,
    initial_weight: Option<f64>,
    spool_weight: Option<f64>,
    used_weight: Option<f64>,
    remaining_length: Option<f64>,
    used_length: Option<f64>,
    location: Option<String>,
    lot_nr: Option<String>,
    archived: bool,
    extra: serde_json::Value,
}