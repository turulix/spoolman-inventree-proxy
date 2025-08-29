use crate::routes::spool::Spool;
use crate::routes::ApiResult;
use actix_web::get;
use actix_web::web::Json;
use inventree::InventreeApiClient;
use inventree::part::PartListQuery;

#[utoipa::path(
    tags = ["Spool"],
    responses(
        (status = 200, description = "Return a health check.", body = Vec<Spool>)
    )
)]
#[get("/spool")]
async fn find_spool_route() -> ApiResult<Json<Vec<Spool>>> {
    unimplemented!()
}
