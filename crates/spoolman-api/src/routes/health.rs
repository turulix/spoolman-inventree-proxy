use crate::routes::ApiResult;
use actix_web::get;
use actix_web::web::Json;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
struct HealthResponse {
    pub status: String,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Return a health check.", body = HealthResponse)
    )
)]
#[get("/health")]
async fn health_route() -> ApiResult<Json<HealthResponse>> {
    Ok(Json(HealthResponse {
        status: "healthy".to_string(),
    }))
}
