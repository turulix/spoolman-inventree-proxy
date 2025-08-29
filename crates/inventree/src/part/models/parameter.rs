use crate::part::models::parameter_template_details::PartParameterTemplateDetails;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct PartParameter {
    pub data: String,
    pub data_numeric: Option<f64>,
    pub template_detail: PartParameterTemplateDetails,
}