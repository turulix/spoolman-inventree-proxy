use crate::routes::ApiResult;
use actix_web::post;
use actix_web::web::Json;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
struct BackupResponse {
    pub path: String,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Return a health check.", body = BackupResponse)
    )
)]
#[post("/backup")]
async fn backup_route() -> ApiResult<Json<BackupResponse>> {
    Ok(Json(BackupResponse {
        path: "NOT IMPLEMENTED".to_string(),
    }))
}
