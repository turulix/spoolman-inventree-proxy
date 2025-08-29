mod get;

use crate::routes::vendor::Vendor;
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_actix_web::service_config::ServiceConfig;

pub fn configure_router(cfg: &mut ServiceConfig) {
    //cfg.service(find_filament_route);
}

#[derive(Serialize, ToSchema)]
pub struct Filament {
    /// Unique internal ID of this filament type. (Part in InvenTree)
    pub id: u64,
    /// When the filament was registered in the database. UTC Timezone.
    pub registered: chrono::DateTime<chrono::Utc>,
    /// Filament name, to distinguish this filament type among others from the same vendor. Should contain its color for example.
    pub name: Option<String>,
    /// The vendor of this filament type.
    pub vendor: Option<Vendor>,
    /// The material of this filament, e.g. PLA.
    pub material: Option<String>,
    /// The price of this filament in the system configured currency.
    pub price: Option<f64>,
    /// The density of this filament in g/cm3.
    pub density: f64,
    /// The diameter of this filament in mm.
    pub diameter: f64,
    /// The weight of the filament in a full spool, in grams.
    pub weight: Option<f64>,
    /// The empty spool weight, in grams.
    pub spool_weight: Option<f64>,
    /// Vendor article number, e.g. EAN, QR code, etc.
    pub article_number: Option<String>,
    /// Free text comment about this filament type.
    pub comment: Option<String>,
    /// Overridden extruder temperature, in °C.
    pub settings_extruder_temp: Option<f64>,
    /// Overridden bed temperature, in °C.
    pub settings_bed_temp: Option<f64>,
    /// Hexadecimal color code of the filament, e.g. FF0000 for red. Supports alpha channel at the end. If it's a multi-color filament, the multi_color_hexes field is used instead.
    pub color_hex: Option<String>,
    /// Hexadecimal color code of the filament, e.g. FF0000 for red. Supports alpha channel at the end. Specifying multiple colors separated by commas. Also set the multi_color_direction field if you specify multiple colors.
    pub multi_color_hexes: Option<String>,
    /// Type of multi-color filament. Only set if the multi_color_hexes field is set.
    pub multi_color_direction: Option<Vec<String>>,
    /// Set if this filament comes from an external database. This contains the ID in the external database.
    pub external_id: Option<String>,
    /// Extra fields for this filament. All values are JSON-encoded data. Query the /fields endpoint for more details about the fields.
    pub extras: serde_json::Value,
}
