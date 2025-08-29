use crate::context::Context;
use crate::routes::spool::Spool;
use crate::routes::ApiResult;
use actix_web::rt::spawn;
use actix_web::web::Data;
use actix_web::{get, web, HttpRequest, HttpResponse};
use actix_ws::Message;
use futures_util::StreamExt;
use log::debug;
use inventree::part::PartListQuery;
use inventree::stock::StockListQuery;
use settings::SETTINGS;

#[utoipa::path(
    tags = ["Spool"],
    responses(
        (status = 200, description = "Return a health check.", body = Vec<Spool>)
    )
)]
#[get("/spool")]
async fn find_spool_route(
    ctx: Data<Context>,
    req: HttpRequest,
    body: web::Payload,
) -> ApiResult<HttpResponse> {
    let is_websocket = req
        .headers()
        .get("upgrade")
        .map_or(false, |h| h == "websocket");

    if is_websocket {
        let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

        spawn(async move {
            while let Some(Ok(msg)) = msg_stream.next().await {
                debug!("got WS message: {msg:?}");
                match msg {
                    Message::Text(_) => {}
                    Message::Binary(_) => {}
                    Message::Continuation(_) => {}
                    Message::Ping(x) => {
                        let _ = session.pong(&x).await;
                    }
                    Message::Pong(_) => {}
                    Message::Close(_) => {}
                    Message::Nop => {}
                }
            }

            let _ = session.close(None).await;
        });

        return Ok(response);
    }

    let stockitems = ctx
        .inv
        .stock()
        .list(&Some(StockListQuery {
            category: Some(SETTINGS.category_id),
            supplier_part_detail: Some(true),
            location_detail: Some(true),
            ..Default::default()
        }))
        .await
        .unwrap();

    let parts = ctx
        .inv
        .part()
        .list(&Some(PartListQuery {
            category: Some(SETTINGS.category_id),
            parameters: Some(true),
            ..Default::default()
        }))
        .await
        .unwrap();

    let spools = stockitems
        .into_iter()
        .filter_map(|si| {
            let part = parts.iter().find(|p| p.pk == si.part)?;
            Some(Spool::from_inventree(&si, part))
        })
        .collect::<Vec<Spool>>();

    Ok(HttpResponse::Ok().json(spools))
}
