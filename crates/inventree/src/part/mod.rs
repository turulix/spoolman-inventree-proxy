mod part_list;

use chrono::NaiveDate;
use crate::InventreeApiClient;

pub use part_list::*;

pub struct PartRepository(pub(crate) InventreeApiClient);

#[derive(serde::Deserialize, Debug)]
pub struct InventreePart {
    pub active: bool,
    pub category: Option<u64>,
    pub category_name: Option<String>,
    pub full_name: String,
    #[serde(rename = "IPN")]
    pub ipn: Option<String>,
    pub name: String,
    pub pk: u64,
    pub creation_date: NaiveDate,
    #[serde(default)]
    pub parameters: Vec<PartParameter>
}

#[derive(serde::Deserialize, Debug)]
pub struct PartParameter {
    pub data: String,
    pub data_numeric: Option<f64>,
    pub template_detail: PartParameterTemplateDetail,
}

#[derive(serde::Deserialize, Debug)]
pub struct PartParameterTemplateDetail {
    pub pk: u64,
    pub name: String,
}