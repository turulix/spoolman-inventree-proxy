mod backup;
mod filament;
mod health;
mod info;
mod spool;
mod vendor;

use crate::routes::backup::backup_route;
use crate::routes::health::health_route;
use crate::routes::info::info_route;
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::web::Redirect;
use actix_web::HttpResponse;
use anyhow::Error;
use serde::Serialize;
use settings::SETTINGS;
use std::fmt::{Debug, Display, Formatter};
use utoipa_actix_web::scope;
use utoipa_actix_web::service_config::ServiceConfig;

pub fn configure_router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api/v1")
            .service(info_route)
            .service(health_route)
            .service(backup_route)
            .configure(filament::configure_router)
            .configure(spool::configure_router)
            .configure(vendor::configure_router),
    )
    .route(
        "/",
        actix_web::web::to(|| async {
            let base_url = SETTINGS
                .inventree_url
                .strip_suffix("/")
                .unwrap_or(&SETTINGS.inventree_url);
            Redirect::to(format!(
                "{base_url}/web/part/category/{}/parts",
                SETTINGS.category_id
            ))
            .temporary()
        }),
    );
}

type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, Serialize)]
pub struct ApiError {
    #[serde(skip)]
    pub status_code: StatusCode,
    pub message: String,
}

impl ApiError {
    pub fn bad_request<T: ToString>(message: T) -> Self {
        ApiError {
            status_code: StatusCode::BAD_REQUEST,
            message: message.to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn not_found<T: ToString>(message: T) -> Self {
        ApiError {
            status_code: StatusCode::NOT_FOUND,
            message: message.to_string(),
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl actix_web::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        self.status_code
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code).json(self)
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(value: Error) -> Self {
        ApiError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: value.to_string(),
        }
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(value: sqlx::Error) -> Self {
        ApiError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: value.to_string(),
        }
    }
}
