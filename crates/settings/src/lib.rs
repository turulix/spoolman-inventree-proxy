mod settings;
mod parameters;

use std::sync::LazyLock;
use crate::settings::Settings;

pub static SETTINGS: LazyLock<Settings> =
    LazyLock::new(|| Settings::new().expect("Failed to Parse Config"));