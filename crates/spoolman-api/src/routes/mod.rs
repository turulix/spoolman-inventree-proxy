mod backup;
mod filament;
mod health;
mod info;
mod spool;
mod vendor;

use crate::routes::backup::backup_route;
use crate::routes::health::health_route;
use crate::routes::info::info_route;
use utoipa_actix_web::scope;
use utoipa_actix_web::service_config::ServiceConfig;

pub fn configure_router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api/v1")
            .service(info_route)
            .service(health_route)
            .service(backup_route)
            .configure(filament::configure_router)
            .configure(spool::configure_router),
    );
}

type ApiResult<T> = Result<T, actix_web::Error>;
