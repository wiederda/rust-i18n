use std::collections::HashMap;
use std::fs;

pub mod error;
pub mod translation;
pub mod loader;

pub use error::I18nError;
pub use translation::Translation;
pub use loader::I18nLoader;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::{init, set_locale, current_locale, set_default_locale, set_fallback_locale, t, t_with_placeholders, I18nError, I18nResult};
}

/// Result type for i18n operations
pub type I18nResult<T> = Result<T, I18nError>;

lazy_static::lazy_static! {
    static ref LOCALES: std::sync::RwLock<LoaderContainer> = std::sync::RwLock::new(LoaderContainer::new());
    static ref CURRENT_LOCALE: std::sync::RwLock<String> = std::sync::RwLock::new("en".to_string());
}

/// Container for managing loaded locales
pub struct LoaderContainer {
    pub default_locale: String,
    pub fallback_locale: String,
    pub locales: HashMap<String, HashMap<String, serde_json::Value>>,
}

impl LoaderContainer {
    fn new() -> Self {
        Self {
            default_locale: "en".to_string(),
            fallback_locale: "en".to_string(),
            locales: HashMap::new(),
        }
    }
}

/// Initialize the i18n library with a locales directory
///
/// # Arguments
///
/// * `locales_dir` - Path to the directory containing locale files (e.g., "locales")
pub fn init(locales_dir: &str) -> I18nResult<()> {
    let loader = I18nLoader::new(locales_dir)?;
    let mut locales_map = LOCALES.write().unwrap();

    match fs::read_dir(locales_dir) {
        Ok(entries) => {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                    if let Some(locale) = path.file_stem().and_then(|s| s.to_str()) {
                        match loader.load_locale(locale) {
                            Ok(locale_data) => {
                                locales_map.locales.insert(locale.to_string(), locale_data);
                            }
                            Err(e) => {
                                eprintln!("Warning: Failed to load locale '{}': {}", locale, e);
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            return Err(I18nError::IoError(format!("Failed to read locales directory '{}': {}", locales_dir, e)));
        }
    }

    Ok(())
}

/// Set the current locale
///
/// # Arguments
///
/// * `locale` - The locale to set (e.g., "en", "de", "fr")
pub fn set_locale(locale: &str) -> I18nResult<()> {
    let locales = LOCALES.read().unwrap();
    if !locales.locales.contains_key(locale) {
        return Err(I18nError::MissingLocale(locale.to_string()));
    }
    drop(locales);
    let mut current = CURRENT_LOCALE.write().unwrap();
    *current = locale.to_string();
    Ok(())
}

/// Get the current locale
pub fn current_locale() -> String {
    CURRENT_LOCALE.read().unwrap().clone()
}

/// Set the default locale
///
/// # Arguments
///
/// * `locale` - The default locale to set
pub fn set_default_locale(locale: &str) {
    let mut locales = LOCALES.write().unwrap();
    locales.default_locale = locale.to_string();
}

/// Set the fallback locale
///
/// # Arguments
///
/// * `locale` - The fallback locale to set
pub fn set_fallback_locale(locale: &str) {
    let mut locales = LOCALES.write().unwrap();
    locales.fallback_locale = locale.to_string();
}

/// Get a translation by key
///
/// # Arguments
///
/// * `key` - The translation key (e.g., "hello.world")
pub fn t(key: &str) -> String {
    let locales = LOCALES.read().unwrap();
    let current_locale = current_locale();

    // Try to get from current locale first
    if let Some(translation) = get_translation(&locales.locales, &current_locale, key) {
        return translate(translation);
    }

    // Fall back to fallback locale
    if let Some(translation) = get_translation(&locales.locales, &locales.fallback_locale, key) {
        return translate(translation);
    }

    // Return the key as fallback if no translation found
    key.to_string()
}

/// Get a translation with placeholders
///
/// # Arguments
///
/// * `key` - The translation key
/// * `placeholders` - A map of placeholder names to values
pub fn t_with_placeholders(key: &str, placeholders: &HashMap<&str, &str>) -> String {
    let translated = t(key);

    let mut result = translated;
    for (placeholder, value) in placeholders {
        let placeholder_str = format!("{{{}}}", placeholder);
        result = result.replace(&placeholder_str, value);
    }

    result
}

/// Get a translation with placeholders using a macro (convenience)
#[macro_export]
macro_rules! t_with_placeholders {
    ($key:expr, $placeholders:expr) => {
        $crate::t_with_placeholders($key, $placeholders)
    };
}

fn get_translation<'a>(
    locales: &'a HashMap<String, HashMap<String, serde_json::Value>>,
    locale: &str,
    key: &str,
) -> Option<&'a serde_json::Value> {
    locales.get(locale).and_then(|locale_map| locale_map.get(key))
}

fn translate(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Null => "".to_string(),
        serde_json::Value::Array(arr) => {
            arr.iter()
                .map(|v| translate(v))
                .collect::<Vec<String>>()
                .join(", ")
        }
        serde_json::Value::Object(obj) => {
            obj.iter()
                .map(|(k, v)| format!("{}: {}", k, translate(v)))
                .collect::<Vec<String>>()
                .join(", ")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translation_key_creation() {
        let key = translation::TranslationKey::new("hello.world");
        assert_eq!(key.as_str(), "hello.world");
        assert_eq!(key.split(), vec!["hello", "world"]);
    }

    #[test]
    fn test_translation_key_from_str() {
        let key: translation::TranslationKey = "test.key".into();
        assert_eq!(key.as_str(), "test.key");
    }

    #[test]
    fn test_translation_key_from_string() {
        let key: translation::TranslationKey = String::from("test.key").into();
        assert_eq!(key.as_str(), "test.key");
    }

    #[test]
    fn test_translate_string() {
        let value = serde_json::Value::String("Hello".to_string());
        assert_eq!(translate(&value), "Hello");
    }

    #[test]
    fn test_translate_number() {
        let value = serde_json::Value::Number(serde_json::Number::from(42));
        assert_eq!(translate(&value), "42");
    }

    #[test]
    fn test_translate_bool() {
        let value = serde_json::Value::Bool(true);
        assert_eq!(translate(&value), "true");
    }

    #[test]
    fn test_translate_null() {
        let value = serde_json::Value::Null;
        assert_eq!(translate(&value), "");
    }

    #[test]
    fn test_translate_array() {
        let value = serde_json::json!(["a", "b", "c"]);
        assert_eq!(translate(&value), "a, b, c");
    }

    #[test]
    fn test_get_translation_missing() {
        let locales = HashMap::new();
        assert!(get_translation(&locales, "en", "key").is_none());
    }

    #[test]
    fn test_error_display() {
        let err = I18nError::InvalidJson("test".to_string());
        assert_eq!(err.to_string(), "Invalid JSON format: test");

        let err = I18nError::MissingLocale("de".to_string());
        assert_eq!(err.to_string(), "Missing locale file: de");

        let err = I18nError::MissingTranslation("en".to_string(), "key".to_string());
        assert_eq!(err.to_string(), "Missing translation key 'key' in locale en");

        let err = I18nError::IoError("file not found".to_string());
        assert_eq!(err.to_string(), "IO error: file not found");
    }
}