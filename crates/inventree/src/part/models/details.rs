use crate::part::PartId;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct PartDetails {
    pub pk: PartId,
    #[serde(rename = "IPN")]
    pub ipn: Option<String>,
    pub barcode_hash: String,
    pub category_default_location: Option<u64>,
    pub default_location: Option<u64>,
    pub default_expiry: u64,
    pub name: String,
    pub revision: Option<String>,
    pub full_name: String,
    pub description: String,
    pub image: Option<String>,
    pub thumbnail: Option<String>,
    pub active: bool,
    pub locked: bool,
    pub assembly: bool,
    pub component: bool,
    pub is_template: bool,
    pub purchaseable: bool,
    pub salable: bool,
    pub testable: bool,
    pub trackable: bool,
    #[serde(rename = "virtual")]
    pub r#virtual: bool,
    pub units: Option<String>,
    pub pricing_min: Option<f64>,
    pub pricing_max: Option<f64>,
}
