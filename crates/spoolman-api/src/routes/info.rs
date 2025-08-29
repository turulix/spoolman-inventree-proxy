use crate::routes::ApiResult;
use actix_web::get;
use actix_web::web::Json;
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
struct InfoResponse {
    pub version: String,
    pub debug_mode: bool,
    pub automatic_updates: bool,
    pub data_dir: String,
    pub logs_dir: String,
    pub backups_dir: String,
    pub db_type: String,
    pub git_commit: String,
    pub build_date: DateTime<Utc>,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Get API info", body = InfoResponse)
    )
)]
#[get("/info")]
async fn info_route() -> ApiResult<Json<InfoResponse>> {
    Ok(Json(InfoResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
        debug_mode: cfg!(debug_assertions),
        automatic_updates: true,
        data_dir: "./data".to_string(),
        logs_dir: "./logs".to_string(),
        backups_dir: "./backups".to_string(),
        db_type: "sqlite".to_string(),
        git_commit: "TODO".to_string(),
        build_date: Utc::now(),
    }))
}
