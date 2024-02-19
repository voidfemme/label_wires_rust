use crate::config::settings::Settings as WireLabSettings;
use crate::utils::localizer::Localizer;
use once_cell::sync::Lazy;
use std::path::PathBuf;
use std::sync::Arc;

pub mod command;
pub mod config;
pub mod controllers;
pub mod models;
pub mod ui;
pub mod utils;

pub static LOCALIZER: Lazy<Arc<Localizer>> = Lazy::new(|| Arc::new(Localizer::new("en")));

pub static SETTINGS: Lazy<Arc<WireLabSettings>> = Lazy::new(|| {
    Arc::new(WireLabSettings::new(PathBuf::from("resources/config/settings.json")).unwrap())
});

pub fn get_localizer() -> Arc<Localizer> {
    LOCALIZER.clone()
}

pub fn get_settings() -> Arc<WireLabSettings> {
    SETTINGS.clone()
}
