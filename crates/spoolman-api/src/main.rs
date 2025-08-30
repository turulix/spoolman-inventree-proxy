mod context;
mod cron;
mod db;
mod routes;
mod utils;

use crate::context::Context;
use crate::cron::flush_pending_task::start_flushing_job;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::web::Data;
use actix_web::{App, Error, HttpServer};
use inventree::InventreeApiClient;
use log::{info, warn};
use settings::SETTINGS;
use std::fs;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

async fn default_service(req: ServiceRequest) -> Result<ServiceResponse, Error> {
    dbg!(&req);
    warn!(
        "A unhandled request was made to: {} {}",
        req.method(),
        req.uri()
    );

    Err(actix_web::error::ErrorNotFound("Not Found"))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    info!("Starting Spoolman API Proxy Server...");

    // Force load settings at startup
    let _ = SETTINGS.clone();
    let inv_client = InventreeApiClient::new(&SETTINGS.inventree_url, &SETTINGS.inventree_token);
    let context = Context {
        inv: inv_client,
        db: db::DbClient::new(&SETTINGS.sqlite_db_path).await,
    };

    start_flushing_job(context.clone());

    HttpServer::new(move || {
        let (app, api) = App::new()
            .app_data(Data::new(context.clone()))
            .into_utoipa_app()
            .default_service(default_service)
            .configure(routes::configure_router)
            .split_for_parts();

        if std::env::var("GENERATE_OPENAPI").is_ok() {
            fs::write("./openapi.json", api.to_pretty_json().unwrap()).unwrap();
            warn!("Generation Mode Enabled - Generating OpenAPI Spec. Exiting...");
            std::process::exit(0);
        }
        app.service(SwaggerUi::new("/spec/swagger-ui/{_:.*}").url("/spec/openapi.json", api))
    })
    .bind(SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 4000))?
    .bind(SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), 4000))?
    .run()
    .await?;

    Ok(())
}
