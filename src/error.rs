use std::fmt;

/// Errors that can occur during i18n operations
#[derive(Debug, Clone)]
pub enum I18nError {
    /// Invalid JSON file format
    InvalidJson(String),
    /// Missing locale file
    MissingLocale(String),
    /// Missing translation key
    MissingTranslation(String, String), // (locale, key)
    /// IO error when reading files
    IoError(String),
}

impl fmt::Display for I18nError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            I18nError::InvalidJson(msg) => write!(f, "Invalid JSON format: {}", msg),
            I18nError::MissingLocale(locale) => write!(f, "Missing locale file: {}", locale),
            I18nError::MissingTranslation(locale, key) => write!(f, "Missing translation key '{}' in locale {}", key, locale),
            I18nError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for I18nError {}