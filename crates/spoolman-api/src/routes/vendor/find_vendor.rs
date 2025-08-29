use actix_web::{get, web, HttpRequest};
use actix_web::web::Json;
use crate::routes::ApiResult;
use crate::routes::spool::Spool;
use crate::routes::vendor::Vendor;

#[utoipa::path(
    tags = ["Vendor"],
    responses(
        (status = 200, description = "Return a specific spool.", body = Spool)
    )
)]
#[get("/vendor")]
async fn find_vendor_route(
    req: HttpRequest,
    body: web::Payload,
) -> ApiResult<Json<Vendor>> {
    Ok(Json(Vendor {
        id: 1,
        registered: Default::default(),
        name: "Test Vendor".to_string(),
        comment: None,
        empty_spool_weight: None,
        external_id: None,
        extras: Default::default(),
    }))
}