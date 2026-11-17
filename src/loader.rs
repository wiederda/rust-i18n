use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde_json::Value;

use crate::error::I18nError;

/// Loader for i18n locale files
pub struct I18nLoader {
    locales_dir: String,
}

impl I18nLoader {
    /// Create a new loader
    pub fn new(locales_dir: &str) -> Result<Self, I18nError> {
        let path = Path::new(locales_dir);
        if !path.exists() {
            return Err(I18nError::MissingLocale(locales_dir.to_string()));
        }
        
        if !path.is_dir() {
            return Err(I18nError::InvalidJson(format!("Not a directory: {}", locales_dir)));
        }
        
        Ok(Self {
            locales_dir: locales_dir.to_string(),
        })
    }
    
    /// Get available locales
    pub fn get_available_locales(&self) -> Vec<String> {
        let mut locales = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.locales_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                    if let Some(locale) = path.file_stem().and_then(|s| s.to_str()) {
                        locales.push(locale.to_string());
                    }
                }
            }
        }
        locales
    }
    
    /// Load a specific locale file
    pub fn load_locale(&self, locale: &str) -> Result<HashMap<String, Value>, I18nError> {
        let file_path = format!("{}/{}.json", self.locales_dir, locale);
        let content = fs::read_to_string(file_path)
            .map_err(|e| I18nError::IoError(format!("Failed to read locale file {}: {}", locale, e)))?;
        
        let json_value: Value = serde_json::from_str(&content)
            .map_err(|e| I18nError::InvalidJson(format!("Invalid JSON in locale file {}: {}", locale, e)))?;
        
        match json_value {
            Value::Object(obj) => {
                let map: HashMap<String, Value> = obj.into_iter().collect();
                Ok(map)
            },
            _ => Err(I18nError::InvalidJson(format!("Invalid format in locale file: {}", locale))),
        }
    }
}