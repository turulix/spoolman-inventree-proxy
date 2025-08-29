use crate::part::PartId;
use crate::stock::yyyy_mm_dd_hh_mm_format;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(transparent)]
pub struct SupplierPartId(pub u64);

#[derive(Deserialize, Debug, Clone)]
pub struct SupplierPartDetails {
    pub description: Option<String>,
    pub in_stock: Option<bool>,
    pub link: Option<String>,
    pub active: bool,
    pub manufacturer_part: u64,
    #[serde(rename = "MPN")]
    pub mpn: Option<String>,
    pub note: Option<String>,
    pub pk: SupplierPartId,
    pub barcode_hash: String,
    pub packaging: Option<String>,
    pub pack_quantity: Option<String>,
    pub pack_quantity_native: f64,
    pub part: PartId,
    #[serde(rename = "SKU")]
    pub sku: String,
    pub supplier: u64,
    //TODO: Implement this field
    // #[serde(with = "yyyy_mm_dd_hh_mm_format")]
    // pub updated: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}
