use crate::context::Context;
use crate::routes::spool::Spool;
use crate::routes::{ApiError, ApiResult};
use actix_web::web::{Data, Json};
use actix_web::{put, web};
use inventree::stock::StockItemId;
use serde::Deserialize;
use std::f64::consts::PI;
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, IntoParams)]
struct UseSpoolParams {
    spool_id: u64,
}

#[derive(Deserialize, ToSchema, Debug)]
struct UseSpoolBody {
    use_length: Option<f64>,
    use_weight: Option<f64>,
}

#[utoipa::path(
    tags = ["Spool"],
    params(UseSpoolParams),
    responses(
        (status = 200, description = "Successfully Updated Filament", body = Spool)
    )
)]
#[put("/spool/{spool_id}/use")]
async fn use_spool_route(
    ctx: Data<Context>,
    path: web::Path<UseSpoolParams>,
    body: Json<UseSpoolBody>,
) -> ApiResult<Json<Spool>> {
    if body.use_length.is_none() && body.use_weight.is_none() {
        return Err(ApiError::bad_request(
            "Either use_length or use_weight must be provided",
        ));
    }

    let mut stock_item = ctx
        .inv
        .stock()
        .retrieve(StockItemId(path.spool_id), &Some(Default::default()))
        .await?;

    let part = ctx
        .inv
        .part()
        .retrieve(stock_item.part, &Some(Default::default()))
        .await?;

    let spool = Spool::from_inventree(&stock_item, &part);

    let used_weight = body.use_weight.unwrap_or_else(|| {
        let use_length = body.use_length.unwrap(); // mm
        let filament_diameter = spool.filament.diameter; // mm

        let radius = filament_diameter / 2.0; // mm
        let volume_cm3 = (PI * (radius * radius) * use_length) / 1000.0; // cm^3
        volume_cm3 * spool.filament.density
    });

    let mut conn = ctx.db.begin().await?;
    // Update the pending spool usage
    ctx.db
        .update_pending_spool_usage(&mut conn, stock_item.pk, used_weight)
        .await?;

    // Re-fetch pending usage to adjust quantity
    let already_pending = ctx
        .db
        .select_pending_spool_usage(&mut conn, stock_item.pk)
        .await?
        .unwrap_or(0.0);

    // Ensure we don't go below zero
    stock_item.quantity = (stock_item.quantity - already_pending).max(0.0);

    let spool = Spool::from_inventree(&stock_item, &part);
    conn.commit().await?;

    // ctx.inv
    //     .stock()
    //     .remove_create(&RemoveCreateBody {
    //         notes: "Used via Spoolman Proxy".to_string(),
    //         items: vec![RemoveCreateInner {
    //             pk: stock_item.pk,
    //             quantity: format!("{used_weight:.5}"),
    //         }],
    //     })
    //     .await
    //     .unwrap();

    Ok(Json(spool))
}
