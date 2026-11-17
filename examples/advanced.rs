use rust_i18n::prelude::*;
use std::collections::HashMap;

fn main() {
    println!("=== rust-i18n Advanced Example ===\n");

    // Initialize with error handling
    match init("locales") {
        Ok(_) => println!("✓ i18n initialized successfully"),
        Err(e) => {
            eprintln!("✗ Failed to initialize i18n: {}", e);
            return;
        }
    }

    // Show current locale
    println!("Current locale: {}\n", current_locale());

    // Demonstrate all translations in English
    println!("--- English Translations ---");
    demonstrate_translations();

    // Switch to German
    println!("\n--- German Translations ---");
    if let Err(e) = set_locale("de") {
        eprintln!("Error switching to German: {}", e);
    } else {
        println!("Current locale: {}", current_locale());
        demonstrate_translations();
    }

    // Switch to French
    println!("\n--- French Translations ---");
    if let Err(e) = set_locale("fr") {
        eprintln!("Error switching to French: {}", e);
    } else {
        println!("Current locale: {}", current_locale());
        demonstrate_translations();
    }

    // Demonstrate fallback
    println!("\n--- Fallback Demo ---");
    set_locale("en").unwrap();
    set_fallback_locale("en");
    println!("Fallback locale: en");

    // Try a missing key
    let missing = t("nonexistent.key");
    println!("Missing key '{}': {}", "nonexistent.key", missing);

    // Demonstrate placeholder types
    println!("\n--- Placeholder Examples ---");
    demonstrate_placeholders();

    // Demonstrate error handling
    println!("\n--- Error Handling Demo ---");
    demonstrate_errors();

    println!("\n=== Advanced Example Complete ===");
}

fn demonstrate_translations() {
    let keys = [
        "hello",
        "greeting",
        "welcome.user",
        "file.count",
        "status.success",
        "error.not_found",
        "app.title",
    ];

    for key in keys.iter() {
        println!("  {}: {}", key, t(key));
    }
}

fn demonstrate_placeholders() {
    // String placeholder
    let mut placeholders = HashMap::new();
    placeholders.insert("name", "Alice");
    let welcome = t_with_placeholders("welcome.user", &placeholders);
    println!("  Welcome message: {}", welcome);

    // Number placeholder
    placeholders.clear();
    placeholders.insert("count", "5");
    let file_count = t_with_placeholders("file.count", &placeholders);
    println!("  File count: {}", file_count);

    // Multiple placeholders
    placeholders.clear();
    placeholders.insert("name", "Bob");
    placeholders.insert("count", "10");
    let multi = t_with_placeholders("welcome.user", &placeholders);
    println!("  Multi-placeholder: {}", multi);
}

fn demonstrate_errors() {
    // Try to set an invalid locale
    match set_locale("xx") {
        Ok(_) => println!("  ✓ Set locale to xx"),
        Err(e) => println!("  ✗ Error setting locale: {}", e),
    }

    // Try to initialize with invalid directory
    match init("nonexistent_directory") {
        Ok(_) => println!("  ✓ Initialized with nonexistent directory"),
        Err(e) => println!("  ✗ Error initializing: {}", e),
    }
}