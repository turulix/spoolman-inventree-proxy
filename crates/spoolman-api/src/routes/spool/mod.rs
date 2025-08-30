use crate::utils::SpoolParameters;
use std::f64::consts::PI;
mod find_spool;
mod get_spool;
mod use_spool;

use crate::routes::filament::Filament;
use crate::routes::spool::find_spool::find_spool_route;
use crate::routes::spool::get_spool::get_spool_route;
use crate::routes::spool::use_spool::use_spool_route;
use crate::routes::vendor::Vendor;
use chrono::{DateTime, Utc};
use inventree::part::InventreePart;
use inventree::stock::InventreeStockItem;
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_actix_web::service_config::ServiceConfig;

pub fn configure_router(cfg: &mut ServiceConfig) {
    cfg.service(find_spool_route)
        .service(get_spool_route)
        .service(use_spool_route);
}

#[derive(Serialize, ToSchema)]
pub struct Spool {
    id: u64,
    registered: DateTime<Utc>,
    first_used: Option<DateTime<Utc>>,
    last_used: Option<DateTime<Utc>>,
    filament: Filament,
    price: Option<f64>,
    remaining_weight: Option<f64>,
    initial_weight: Option<f64>,
    spool_weight: Option<f64>,
    used_weight: f64,
    remaining_length: Option<f64>,
    used_length: f64,
    location: Option<String>,
    lot_nr: Option<String>,
    archived: bool,
    extra: serde_json::Value,
}

impl Spool {
    pub fn from_inventree(stock: &InventreeStockItem, part: &InventreePart) -> Self {
        let stock = stock.clone();
        let part = part.clone();

        let initial_weight = stock
            .supplier_part_detail
            .as_ref()
            .map(|x| x.pack_quantity_native);

        let used_weight = initial_weight.map(|x| x - stock.quantity).unwrap(); //TODO: Handle unwrap safely.
        let density = part.filament_density();
        let diameter = part.filament_diameter();
        let filament_material = part.filament_material();
        let spool_weight = part.spool_weight();

        let initial_length = {
            let volume_cm3 = initial_weight.map(|x| x / density); // in cm^3
            let volume_mm3 = volume_cm3.map(|x| x * 1000.0); // in mm^3

            volume_mm3.map(|volume_mm3| volume_mm3 / (PI * (diameter / 2.0) * (diameter / 2.0))) // in mm
        };

        let used_length = {
            let volume_cm3 = used_weight / density; // in cm^3
            let volume_mm3 = volume_cm3 * 1000.0; // in mm^3

            volume_mm3 / (PI * (diameter / 2.0) * (diameter / 2.0)) // in mm
        };

        Spool {
            id: stock.pk.0,
            registered: Default::default(),
            first_used: None,
            last_used: Some(stock.updated),
            filament: Filament {
                id: part.pk.0,
                registered: Default::default(),
                name: Some(part.name.clone()),
                vendor: Some(Vendor {
                    id: 0,
                    registered: Default::default(),
                    name: "TODO".to_string(),
                    comment: None,
                    empty_spool_weight: None,
                    external_id: None,
                    extras: None,
                }), //TODO: Implement vendor
                material: Some(filament_material),
                price: stock
                    .purchase_price
                    .and_then(|x| initial_weight.map(|y| x * y)),
                density,
                diameter,
                weight: initial_weight,
                spool_weight,
                article_number: stock.mpn,
                comment: part.notes.clone(),
                settings_extruder_temp: part.extruder_temp(),
                settings_bed_temp: part.bed_temp(),
                color_hex: part.filament_color(),
                multi_color_hexes: None,
                multi_color_direction: None,
                external_id: part.ipn,
                extras: Default::default(),
            },
            price: stock.purchase_price,
            remaining_weight: Some(stock.quantity),
            initial_weight,
            spool_weight,
            used_weight,
            remaining_length: initial_length.map(|x| x - used_length),
            used_length,
            location: stock.location_detail.map(|x| x.pathstring),
            lot_nr: stock.batch,
            archived: !part.active,
            extra: Default::default(),
        }
    }
}
