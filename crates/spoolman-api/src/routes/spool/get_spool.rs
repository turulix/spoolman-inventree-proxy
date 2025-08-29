use crate::context::Context;
use crate::routes::ApiResult;
use crate::routes::spool::Spool;
use actix_web::web::{Data, Json};
use actix_web::{get, web};
use inventree::stock::StockItemId;
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
struct GetSpoolParams {
    spool_id: u64,
}

#[utoipa::path(
    tags = ["Spool"],
    params(GetSpoolParams),
    responses(
        (status = 200, description = "Return a specific spool.", body = Spool)
    )
)]
#[get("/spool/{spool_id}")]
async fn get_spool_route(
    ctx: Data<Context>,
    path: web::Path<GetSpoolParams>,
) -> ApiResult<Json<Spool>> {
    let stock_item = ctx
        .inv
        .stock()
        .retrieve(StockItemId(path.spool_id), &Some(Default::default()))
        .await
        .unwrap();

    let part = ctx
        .inv
        .part()
        .retrieve(stock_item.part, &Some(Default::default()))
        .await
        .unwrap();

    let spool = Spool::from_inventree(&stock_item, &part);

    Ok(Json(spool))
}
