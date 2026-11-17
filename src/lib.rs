use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::RwLock;

pub mod error;
pub mod loader;
pub mod translation;

pub use error::I18nError;
pub use loader::I18nLoader;
pub use translation::Translation;

/// Convenient imports for the backwards-compatible global API.
pub mod prelude {
    pub use crate::{
        current_locale, init, set_default_locale, set_fallback_locale, set_locale, t,
        t_with_placeholders, I18n, I18nError, I18nResult,
    };
}

pub type I18nResult<T> = Result<T, I18nError>;

/// An isolated translation catalog for one application or project.
///
/// Each instance owns its locale directory and language selection, so separate
/// projects (or components) can use different catalogs in the same process.
pub struct I18n {
    locales_dir: PathBuf,
    state: RwLock<LocaleState>,
}

struct LocaleState {
    current_locale: String,
    default_locale: String,
    fallback_locale: String,
    locales: HashMap<String, HashMap<String, serde_json::Value>>,
}

impl I18n {
    /// Load all JSON locale files from a project-specific directory.
    ///
    /// The default, fallback, and initial locale are `en` when available;
    /// otherwise they use the first locale found in the directory.
    pub fn new(locales_dir: impl Into<PathBuf>) -> I18nResult<Self> {
        let locales_dir = locales_dir.into();
        let loader = I18nLoader::new(&locales_dir)?;
        let locales = loader.load_all()?;
        if locales.is_empty() {
            return Err(I18nError::MissingLocale(
                "No .json locale files found".to_string(),
            ));
        }
        let initial = if locales.contains_key("en") {
            "en".to_string()
        } else {
            let mut available: Vec<_> = locales.keys().cloned().collect();
            available.sort();
            available[0].clone()
        };
        Ok(Self {
            locales_dir,
            state: RwLock::new(LocaleState {
                current_locale: initial.clone(),
                default_locale: initial.clone(),
                fallback_locale: initial,
                locales,
            }),
        })
    }

    /// Reload every locale atomically. On failure, the current catalog remains usable.
    pub fn reload(&self) -> I18nResult<()> {
        let loader = I18nLoader::new(&self.locales_dir)?;
        let locales = loader.load_all()?;
        if locales.is_empty() {
            return Err(I18nError::MissingLocale(
                "No .json locale files found".to_string(),
            ));
        }
        let mut state = self.state.write().unwrap();
        if !locales.contains_key(&state.current_locale) {
            state.current_locale = if locales.contains_key(&state.default_locale) {
                state.default_locale.clone()
            } else {
                let mut available: Vec<_> = locales.keys().cloned().collect();
                available.sort();
                available[0].clone()
            };
        }
        state.locales = locales;
        Ok(())
    }

    /// List loaded locale identifiers in stable alphabetical order.
    pub fn available_locales(&self) -> Vec<String> {
        let state = self.state.read().unwrap();
        let mut locales: Vec<_> = state.locales.keys().cloned().collect();
        locales.sort();
        locales
    }

    /// Select the active locale.
    pub fn set_locale(&self, locale: &str) -> I18nResult<()> {
        let mut state = self.state.write().unwrap();
        if !state.locales.contains_key(locale) {
            return Err(I18nError::MissingLocale(locale.to_string()));
        }
        state.current_locale = locale.to_string();
        Ok(())
    }

    /// Return the active locale identifier.
    pub fn current_locale(&self) -> String {
        self.state.read().unwrap().current_locale.clone()
    }

    /// Set the default locale used when the active locale is unavailable.
    pub fn set_default_locale(&self, locale: &str) -> I18nResult<()> {
        let mut state = self.state.write().unwrap();
        ensure_locale(&state.locales, locale)?;
        state.default_locale = locale.to_string();
        Ok(())
    }

    /// Set the locale used when a key is missing from the active locale.
    pub fn set_fallback_locale(&self, locale: &str) -> I18nResult<()> {
        let mut state = self.state.write().unwrap();
        ensure_locale(&state.locales, locale)?;
        state.fallback_locale = locale.to_string();
        Ok(())
    }

    /// Resolve a translation, checking active, fallback, then default locale.
    /// Returns the key itself when no translation exists.
    pub fn t(&self, key: &str) -> String {
        let state = self.state.read().unwrap();
        for locale in [
            state.current_locale.as_str(),
            state.fallback_locale.as_str(),
            state.default_locale.as_str(),
        ] {
            if let Some(value) = get_translation(&state.locales, locale, key) {
                return translate(value);
            }
        }
        key.to_string()
    }

    /// Resolve a translation and replace `{name}` style placeholders.
    pub fn t_with_placeholders(&self, key: &str, placeholders: &HashMap<&str, &str>) -> String {
        let mut result = self.t(key);
        for (placeholder, value) in placeholders {
            result = result.replace(&format!("{{{}}}", placeholder), value);
        }
        result
    }
}

fn ensure_locale(
    locales: &HashMap<String, HashMap<String, serde_json::Value>>,
    locale: &str,
) -> I18nResult<()> {
    if locales.contains_key(locale) {
        Ok(())
    } else {
        Err(I18nError::MissingLocale(locale.to_string()))
    }
}

lazy_static::lazy_static! {
    static ref GLOBAL_I18N: RwLock<Option<I18n>> = RwLock::new(None);
}

/// Initialize the backwards-compatible process-wide catalog.
pub fn init(locales_dir: &str) -> I18nResult<()> {
    let catalog = I18n::new(locales_dir)?;
    *GLOBAL_I18N.write().unwrap() = Some(catalog);
    Ok(())
}

fn with_global<T>(f: impl FnOnce(&I18n) -> T) -> Option<T> {
    let global = GLOBAL_I18N.read().unwrap();
    global.as_ref().map(f)
}

pub fn set_locale(locale: &str) -> I18nResult<()> {
    with_global(|i18n| i18n.set_locale(locale))
        .unwrap_or_else(|| Err(I18nError::MissingLocale("i18n is not initialized".into())))
}

pub fn current_locale() -> String {
    with_global(I18n::current_locale).unwrap_or_else(|| "en".to_string())
}

/// Set the default locale for the global catalog. Invalid locales are ignored
/// for compatibility; use [`I18n::set_default_locale`] for checked updates.
pub fn set_default_locale(locale: &str) {
    let _ = with_global(|i18n| i18n.set_default_locale(locale));
}

/// Set the fallback locale for the global catalog. Invalid locales are ignored
/// for compatibility; use [`I18n::set_fallback_locale`] for checked updates.
pub fn set_fallback_locale(locale: &str) {
    let _ = with_global(|i18n| i18n.set_fallback_locale(locale));
}

pub fn t(key: &str) -> String {
    with_global(|i18n| i18n.t(key)).unwrap_or_else(|| key.to_string())
}

pub fn t_with_placeholders(key: &str, placeholders: &HashMap<&str, &str>) -> String {
    with_global(|i18n| i18n.t_with_placeholders(key, placeholders))
        .unwrap_or_else(|| key.to_string())
}

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
    let data = locales.get(locale)?;
    // Preserve support for literal dotted keys before trying nested traversal.
    if let Some(value) = data.get(key) {
        return Some(value);
    }
    let mut parts = key.split('.');
    let first = parts.next()?;
    let mut value = data.get(first)?;
    for part in parts {
        value = value.as_object()?.get(part)?;
    }
    Some(value)
}

fn translate(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Null => String::new(),
        serde_json::Value::Array(values) => {
            values.iter().map(translate).collect::<Vec<_>>().join(", ")
        }
        serde_json::Value::Object(values) => values
            .iter()
            .map(|(key, value)| format!("{}: {}", key, translate(value)))
            .collect::<Vec<_>>()
            .join(", "),
    }
}
