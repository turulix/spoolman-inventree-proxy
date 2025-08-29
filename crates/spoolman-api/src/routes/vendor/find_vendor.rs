use crate::routes::ApiResult;
use crate::routes::spool::Spool;
use crate::routes::vendor::Vendor;
use actix_web::get;
use actix_web::web::Json;
use log::warn;

#[utoipa::path(
    tags = ["Vendor"],
    responses(
        (status = 200, description = "Return a specific spool.", body = Spool)
    )
)]
#[get("/vendor")]
async fn find_vendor_route() -> ApiResult<Json<Vendor>> {
    warn!("find_vendor_route is a stub and needs to be implemented");
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
