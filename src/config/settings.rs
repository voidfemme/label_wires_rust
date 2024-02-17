use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::path::PathBuf;
use tracing::{debug, error, info, warn};

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    language: String,
    default_wire_file_directory: String,
    default_csv_directory: String,
    default_save_location: String,
    csv_save_location: String,
    default_csv_delimiter: String,
    file_path: PathBuf,
}

impl Settings {
    pub fn new(file_path: PathBuf) -> Result<Settings, Box<dyn std::error::Error>> {
        if file_path.is_file() {
            let file_path_str = file_path.to_string_lossy();
            info!("Loading settings from {:?}", file_path_str);

            // Print or log the full file path
            info!("Reading settings from: {}", file_path_str);

            let contents = fs::read_to_string(file_path).map_err(|e| {
                error!("Failed to read settings file: {:?}", e);
                e
            })?;

            serde_json::from_str(&contents).map_err(|e| {
                error!("Failed to parse settings file: {:?}", e);
                e.into()
            })
        } else {
            warn!(
                "Settings file not found at {:?}, using default settings.",
                file_path
            );
            Ok(Settings::default())
        }
    }

    pub fn default() -> Self {
        info!("Using default settings.");
        Self {
            language: "en".to_string(),
            default_wire_file_directory: "".to_string(),
            default_csv_directory: "".to_string(),
            default_save_location: "/home/rsp/documents".to_string(),
            csv_save_location: "/home/rsp/documents".to_string(),
            default_csv_delimiter: "|".to_string(),
            file_path: "resources/data/settings.json".into(),
        }
    }

    pub fn get(&self, setting_key: &str) -> Option<&str> {
        match setting_key {
            "language" => {
                debug!("Accessed 'language' setting.");
                Some(&self.language)
            }
            "default_wire_file_directory" => {
                debug!("Accessed 'default_wire_file_directory' setting.");
                Some(&self.default_wire_file_directory)
            }
            "default_csv_directory" => {
                debug!("Accessed 'default_csv_directory' setting.");
                Some(&self.default_csv_directory)
            }
            "default_save_location" => {
                debug!("Accessed 'default_save_location' setting.");
                Some(&self.default_save_location)
            }
            "csv_save_location" => {
                debug!("Accessed 'csv_save_location' setting.");
                Some(&self.csv_save_location)
            }
            "default_csv_delimiter" => {
                debug!("Accessed 'default_csv_delimiter' setting.");
                Some(&self.default_csv_delimiter)
            }
            _ => None,
        }
    }

    pub fn print(&self) {
        println!("Current Settings:");
        println!("Language: {}", self.language);
        println!(
            "Default Wire File Directory: {}",
            self.default_wire_file_directory
        );
        println!("Default CSV Directory: {}", self.default_csv_directory);
        println!("Default Save Location: {}", self.default_save_location);
        println!("CSV Save Location: {}", self.csv_save_location);
        println!("Default CSV Delimiter: {}", self.default_csv_delimiter);
        println!("Settings File Path: {}", self.file_path.display());
    }
}
