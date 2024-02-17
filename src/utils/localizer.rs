use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::sync::RwLock;
use tracing::info;

pub struct Localizer {
    // Language code (e.g., "en", "fr")
    language: RwLock<String>,
    // Localization resources, e.g., a map from keys to translations
    resources: HashMap<String, String>,
}

impl Localizer {
    pub fn new(initial_language: &str) -> Localizer {
        let mut localizer = Localizer {
            language: RwLock::new(initial_language.to_string()),
            resources: HashMap::new(),
        };
        localizer.load_resources();
        localizer
    }

    pub fn set_language(&mut self, language: &str) {
        {
            let mut lang = self.language.write().unwrap();
            *lang = language.to_string();
        }
        self.load_resources();
    }

    pub fn get(&self, key: &str) -> String {
        // Retrieve a localized string by key
        info!("Getting localization key: {}", key);
        self.resources
            .get(key)
            .cloned()
            .unwrap_or_else(|| "Missing".to_string())
    }

    pub fn load_resources(&mut self) {
        let lang = self.language.read().unwrap();
        let mut path = PathBuf::from("resources/locales");
        path.push(format!("{}.json", *lang));

        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to open Localization file: {:?}", e);
                return;
            }
        };

        let mut contents = String::new();
        if let Err(e) = file.read_to_string(&mut contents) {
            eprintln!("Failed to read localization file: {:?}", e);
            return;
        }

        let resources: Value = match serde_json::from_str(&contents) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Failed to parse JSON: {:?}", e);
                return;
            }
        };

        if let Some(obj) = resources.as_object() {
            self.resources.clear();
            for (key, value) in obj.iter() {
                if let Some(val_str) = value.as_str() {
                    self.resources.insert(key.clone(), val_str.to_string());
                }
            }
        }
    }
}
