use crate::context::Context;
use crate::routes::spool::Spool;
use crate::routes::ApiResult;
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
    let mut si = ctx
        .inv
        .stock()
        .retrieve(StockItemId(path.spool_id), &Some(Default::default()))
        .await?;

    // Adjust quantity based on pending usage
    let mut conn = ctx.db.begin().await?;
    let pending = ctx.db.select_pending_spool_usage(&mut conn, si.pk).await?;
    if let Some(x) = pending {
        si.quantity = (si.quantity - x).max(0.0);
    }
    drop(conn);

    let part = ctx
        .inv
        .part()
        .retrieve(si.part, &Some(Default::default()))
        .await?;

    let spool = Spool::from_inventree(&si, &part);

    Ok(Json(spool))
}
