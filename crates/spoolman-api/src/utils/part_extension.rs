use inventree::part::InventreePart;
use log::warn;
use settings::SETTINGS;

pub trait SpoolParameters {
    fn filament_density(&self) -> f64;
    fn filament_diameter(&self) -> f64;
    fn filament_material(&self) -> String;
    fn filament_color(&self) -> Option<String>;
    fn extruder_temp(&self) -> Option<f64>;
    fn bed_temp(&self) -> Option<f64>;
    fn spool_weight(&self) -> Option<f64>;
}

impl SpoolParameters for InventreePart {
    fn filament_density(&self) -> f64 {
        match self.select_parameter_numeric(&SETTINGS.parameters.filament_density) {
            None => {
                warn!(
                    "Filament density parameter not found for part {} | {}",
                    self.pk.0, self.full_name
                );
                1.24
            }
            Some(x) => x,
        }
    }

    fn filament_diameter(&self) -> f64 {
        match self.select_parameter_numeric(&SETTINGS.parameters.filament_diameter) {
            None => {
                warn!(
                    "Filament diameter parameter not found for part {} | {}",
                    self.pk.0, self.full_name
                );
                1.75
            }
            Some(x) => x,
        }
    }

    fn filament_material(&self) -> String {
        match self.select_parameter_string(&SETTINGS.parameters.filament_material) {
            None => {
                warn!(
                    "Filament material parameter not found for part {} | {}",
                    self.pk.0, self.full_name
                );
                "PLA".to_string()
            }
            Some(x) => x,
        }
    }

    fn filament_color(&self) -> Option<String> {
        self.select_parameter_string(&SETTINGS.parameters.filament_hex_color)
            .map(|x| x.strip_prefix("#").unwrap_or(&x).to_lowercase())
    }

    fn extruder_temp(&self) -> Option<f64> {
        self.select_parameter_numeric(&SETTINGS.parameters.extruder_temp)
    }

    fn bed_temp(&self) -> Option<f64> {
        self.select_parameter_numeric(&SETTINGS.parameters.bed_temp)
    }

    fn spool_weight(&self) -> Option<f64> {
        self.select_parameter_numeric(&SETTINGS.parameters.spool_weight)
    }
}
