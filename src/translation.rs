use serde_json::Value;

/// Translation key structure for hierarchical keys
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TranslationKey {
    key: String,
}

impl TranslationKey {
    /// Create a new translation key
    pub fn new(key: &str) -> Self {
        Self { key: key.to_string() }
    }

    /// Get the key as string
    pub fn as_str(&self) -> &str {
        &self.key
    }

    /// Split the key by dots to get hierarchical parts
    pub fn split(&self) -> Vec<&str> {
        self.key.split('.').collect()
    }
}

impl From<&str> for TranslationKey {
    fn from(key: &str) -> Self {
        Self::new(key)
    }
}

impl From<String> for TranslationKey {
    fn from(key: String) -> Self {
        Self::new(&key)
    }
}

/// Translation trait for different value types
pub trait Translation {
    /// Get the translation value
    fn value(&self) -> &Value;

    /// Get the translation as string
    fn as_string(&self) -> String {
        match self.value() {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null => "".to_string(),
            _ => "".to_string(),
        }
    }
}

impl Translation for Value {
    fn value(&self) -> &Value {
        self
    }
}