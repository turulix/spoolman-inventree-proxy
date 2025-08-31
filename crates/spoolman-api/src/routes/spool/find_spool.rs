use crate::context::Context;
use crate::routes::ApiResult;
use crate::routes::spool::Spool;
use actix_web::rt::spawn;
use actix_web::web::Data;
use actix_web::{HttpRequest, HttpResponse, get, web};
use actix_ws::{Message, Session};
use anyhow::anyhow;
use futures_util::StreamExt;
use inventree::part::PartListQuery;
use inventree::stock::StockListQuery;
use log::debug;
use settings::SETTINGS;
use std::sync::LazyLock;
use tokio::sync::Mutex;

pub static WEBSOCKET_SESSIONS: LazyLock<Mutex<Vec<Session>>> = LazyLock::new(|| Mutex::new(vec![]));

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
        .is_some_and(|h| h == "websocket");

    if is_websocket {
        let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)
            .map_err(|e| anyhow!("WebSocket handshake error: {e:?}"))?;

        WEBSOCKET_SESSIONS.lock().await.push(session.clone());

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

    let mut stockitems = ctx
        .inv
        .stock()
        .list(&Some(StockListQuery {
            category: Some(SETTINGS.category_id),
            supplier_part_detail: Some(true),
            location_detail: Some(true),
        }))
        .await?;

    // Adjust quantities based on pending spool usage
    let mut conn = ctx.db.begin().await?;
    for si in stockitems.iter_mut() {
        let pending = ctx.db.select_pending_spool_usage(&mut conn, si.pk).await?;
        if let Some(x) = pending {
            si.quantity = (si.quantity - x).max(0.0);
        }
    }
    drop(conn);

    let parts = ctx
        .inv
        .part()
        .list(&Some(PartListQuery {
            category: Some(SETTINGS.category_id),
            parameters: Some(true),
        }))
        .await?;

    let spools = stockitems
        .into_iter()
        .filter_map(|si| {
            let part = parts.iter().find(|p| p.pk == si.part)?;
            Some(Spool::from_inventree(&si, part))
        })
        .collect::<Vec<Spool>>();

    Ok(HttpResponse::Ok().json(spools))
}
