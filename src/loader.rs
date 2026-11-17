use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::error::I18nError;

/// Loader for i18n locale files
pub struct I18nLoader {
    locales_dir: PathBuf,
}

impl I18nLoader {
    /// Create a new loader
    pub fn new(locales_dir: impl AsRef<Path>) -> Result<Self, I18nError> {
        let path = locales_dir.as_ref();
        if !path.exists() {
            return Err(I18nError::MissingLocale(path.display().to_string()));
        }

        if !path.is_dir() {
            return Err(I18nError::InvalidJson(format!(
                "Not a directory: {}",
                path.display()
            )));
        }

        Ok(Self {
            locales_dir: path.to_path_buf(),
        })
    }

    /// Get available locales
    pub fn get_available_locales(&self) -> Vec<String> {
        let mut locales = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.locales_dir) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                    if let Some(locale) = path.file_stem().and_then(|s| s.to_str()) {
                        locales.push(locale.to_string());
                    }
                }
            }
        }
        locales.sort();
        locales
    }

    /// Load every `.json` locale in this directory. Any unreadable or invalid
    /// file fails the whole operation so callers can replace catalogs atomically.
    pub fn load_all(&self) -> Result<HashMap<String, HashMap<String, Value>>, I18nError> {
        let entries = fs::read_dir(&self.locales_dir).map_err(|e| {
            I18nError::IoError(format!(
                "Failed to read locales directory '{}': {}",
                self.locales_dir.display(),
                e
            ))
        })?;
        let mut paths = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|e| {
                I18nError::IoError(format!("Failed to read locale directory entry: {}", e))
            })?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("json") {
                paths.push(path);
            }
        }
        paths.sort();

        let mut locales = HashMap::new();
        for path in paths {
            let locale = path
                .file_stem()
                .and_then(|name| name.to_str())
                .ok_or_else(|| {
                    I18nError::IoError(format!("Invalid locale filename: {}", path.display()))
                })?;
            let data = self.load_locale(locale)?;
            locales.insert(locale.to_string(), data);
        }
        Ok(locales)
    }

    /// Load a specific locale file
    pub fn load_locale(&self, locale: &str) -> Result<HashMap<String, Value>, I18nError> {
        if locale.is_empty()
            || !locale
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            return Err(I18nError::MissingLocale(locale.to_string()));
        }
        let file_path = self.locales_dir.join(format!("{}.json", locale));
        let content = fs::read_to_string(&file_path).map_err(|e| {
            I18nError::IoError(format!(
                "Failed to read locale file {}: {}",
                file_path.display(),
                e
            ))
        })?;

        let json_value: Value = serde_json::from_str(&content).map_err(|e| {
            I18nError::InvalidJson(format!("Invalid JSON in locale file {}: {}", locale, e))
        })?;

        match json_value {
            Value::Object(obj) => {
                let map: HashMap<String, Value> = obj.into_iter().collect();
                Ok(map)
            }
            _ => Err(I18nError::InvalidJson(format!(
                "Invalid format in locale file: {}",
                locale
            ))),
        }
    }
}
