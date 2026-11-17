use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Mutex;

static TEST_LOCK: Mutex<()> = Mutex::new(());

fn create_test_locales() {
    if !Path::new("tests/locales").exists() {
        std::fs::create_dir_all("tests/locales").unwrap();
    }

    let en_json = r#"{
        "hello": "Hello",
        "greeting": "Hello",
        "welcome.user": "Welcome {name}",
        "number": 42
    }"#;

    let de_json = r#"{
        "hello": "Hallo",
        "greeting": "Hallo",
        "welcome.user": "Willkommen {name}",
        "number": 42
    }"#;

    fs::write("tests/locales/en.json", en_json).unwrap();
    fs::write("tests/locales/de.json", de_json).unwrap();
}

fn cleanup_test_locales() {
    let _ = fs::remove_dir_all("tests/locales");
}

fn reset_locale() {
    let _ = rust_i18n::set_locale("en");
}

#[test]
fn test_basic_translation() {
    let _guard = TEST_LOCK.lock().unwrap();
    create_test_locales();
    let _ = rust_i18n::init("tests/locales");
    reset_locale();
    let text = rust_i18n::t("hello");
    assert_eq!(text, "Hello");
    cleanup_test_locales();
}

#[test]
fn test_translation_with_placeholders() {
    let _guard = TEST_LOCK.lock().unwrap();
    create_test_locales();
    let _ = rust_i18n::init("tests/locales");
    reset_locale();
    let mut placeholders: HashMap<&str, &str> = HashMap::new();
    placeholders.insert("name", "World");
    let text = rust_i18n::t_with_placeholders("welcome.user", &placeholders);
    assert_eq!(text, "Welcome World");
    cleanup_test_locales();
}

#[test]
fn test_locale_switching() {
    let _guard = TEST_LOCK.lock().unwrap();
    create_test_locales();
    let _ = rust_i18n::init("tests/locales");
    reset_locale();
    let en_text = rust_i18n::t("hello");
    assert_eq!(en_text, "Hello");
    let _ = rust_i18n::set_locale("de");
    let de_text = rust_i18n::t("hello");
    assert_eq!(de_text, "Hallo");
    cleanup_test_locales();
}

#[test]
fn test_fallback() {
    let _guard = TEST_LOCK.lock().unwrap();
    create_test_locales();
    let _ = rust_i18n::init("tests/locales");
    reset_locale();
    let text = rust_i18n::t("greeting");
    assert!(text.contains("Hello"));
    cleanup_test_locales();
}