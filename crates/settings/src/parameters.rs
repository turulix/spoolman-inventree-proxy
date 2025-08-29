use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ParameterSettings {
    pub extruder_temp: String,
    pub bed_temp: String,
    pub filament_diameter: String,
    pub filament_density: String,
    pub filament_material: String,
    pub filament_hex_color: String,
    pub spool_weight: String,
}

impl Default for ParameterSettings {
    fn default() -> Self {
        Self {
            extruder_temp: "3DPrint Extruder Temperature".to_string(),
            bed_temp: "3DPrint Bed Temperature".to_string(),
            filament_diameter: "3DPrint Filament Diameter".to_string(),
            filament_density: "3DPrint Filament Density".to_string(),
            filament_material: "3DPrint Filament Material".to_string(),
            filament_hex_color: "3DPrint Filament Color".to_string(),
            spool_weight: "3DPrint Spool Weight".to_string(),
        }
    }
}
