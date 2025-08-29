mod details;
mod parameter;
mod parameter_template_details;

use chrono::NaiveDate;
pub use details::*;
pub use parameter::*;
pub use parameter_template_details::*;

#[derive(serde::Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
#[serde(transparent)]
pub struct PartId(pub u64);

#[derive(serde::Deserialize, Debug, Clone)]
pub struct InventreePart {
    pub active: bool,
    pub category: Option<u64>,
    pub category_name: Option<String>,
    pub full_name: String,
    #[serde(rename = "IPN")]
    pub ipn: Option<String>,
    pub name: String,
    pub pk: PartId,
    pub creation_date: NaiveDate,
    pub notes: Option<String>,
    #[serde(default)]
    pub parameters: Vec<PartParameter>,
}

impl InventreePart {
    pub fn select_parameter_numeric(&self, name: &str) -> Option<f64> {
        self.parameters
            .iter()
            .find(|x| x.template_detail.name == name)
            .and_then(|x| x.data_numeric)
    }

    pub fn select_parameter_string(&self, name: &str) -> Option<String> {
        self.parameters
            .iter()
            .find(|x| x.template_detail.name == name)
            .map(|x| x.data.clone())
    }
}
