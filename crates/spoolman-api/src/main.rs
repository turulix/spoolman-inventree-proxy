mod routes;

use std::fs;
use actix_web::{App, HttpServer};
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use log::{info, warn};
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    info!("Starting Spoolman API Proxy Server...");

    HttpServer::new(move || {
        let (app, api) = App::new()
            .into_utoipa_app()
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
