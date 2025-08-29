use crate::location::LocationDetails;
use crate::part::{PartDetails, PartId};
use crate::supplier::{SupplierPartDetails, SupplierPartId};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct StockItemId(pub u64);

#[derive(Deserialize, Debug, Clone)]
pub struct InventreeStockItem {
    pub pk: StockItemId,
    pub part: PartId,
    pub quantity: f64,
    pub serial: Option<String>,
    pub batch: Option<String>,
    pub location: Option<u64>,
    pub belongs_to: Option<u64>,
    pub build: Option<u64>,
    pub consumed_by: Option<u64>,
    pub customer: Option<u64>,
    pub delete_on_deplete: bool,
    pub expiry_date: Option<NaiveDate>,
    pub in_stock: bool,
    pub is_building: bool,
    pub link: String,
    pub notes: Option<String>,
    pub owner: Option<u64>,
    pub packaging: Option<String>,
    pub parent: Option<u64>,
    pub purchase_order: Option<u64>,
    pub purchase_order_reference: Option<String>,
    pub sales_order: Option<u64>,
    pub sales_order_reference: Option<String>,
    pub status: u64,
    pub status_text: String,
    pub status_custom_key: Option<u64>,
    pub supplier_part: SupplierPartId,
    #[serde(rename = "SKU")]
    pub sku: Option<String>,
    #[serde(rename = "MPN")]
    pub mpn: Option<String>,
    pub barcode_hash: String,
    #[serde(with = "yyyy_mm_dd_hh_mm_format")]
    pub updated: DateTime<Utc>, //TODO: check format 2025-08-29 00:11
    pub stocktake_date: Option<NaiveDate>,
    pub purchase_price: Option<f64>,
    pub purchase_price_currency: Option<String>,
    pub allocated: f64,
    pub expired: bool,
    pub installed_items: Option<u64>,
    pub child_items: Option<u64>,
    pub tracking_items: Option<u64>,
    pub tags: Vec<String>,
    pub supplier_part_detail: Option<SupplierPartDetails>,
    pub part_detail: Option<PartDetails>,
    pub location_detail: Option<LocationDetails>,
}

pub(crate) mod yyyy_mm_dd_hh_mm_format {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
    }
}
