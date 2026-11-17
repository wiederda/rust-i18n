use rust_i18n::*;
use std::collections::HashMap;

fn main() {
    // Initialize the i18n system with the locales directory
    init("locales").expect("Failed to initialize i18n");

    println!("=== rust-i18n Example ===\n");

    // Get current locale
    println!("Current locale: {}", current_locale());

    // Get a simple translation
    let hello = t("hello");
    println!("Hello: {}", hello);

    // Get a hierarchical translation
    let title = t("app.title");
    println!("App title: {}", title);

    // Use placeholders
    let mut placeholders: HashMap<&str, &str> = HashMap::new();
    placeholders.insert("name", "John");
    let welcome = t_with_placeholders("welcome.user", &placeholders);
    println!("Welcome: {}", welcome);

    // Switch to German
    println!("\n--- Switching to German ---");
    set_locale("de").expect("Failed to set German locale");
    println!("Current locale: {}", current_locale());

    let hello_de = t("hello");
    println!("Hello: {}", hello_de);

    let welcome_de = t_with_placeholders("welcome.user", &placeholders);
    println!("Welcome: {}", welcome_de);

    // Switch to French
    println!("\n--- Switching to French ---");
    set_locale("fr").expect("Failed to set French locale");
    println!("Current locale: {}", current_locale());

    let hello_fr = t("hello");
    println!("Hello: {}", hello_fr);

    let welcome_fr = t_with_placeholders("welcome.user", &placeholders);
    println!("Welcome: {}", welcome_fr);

    // Demonstrate fallback
    println!("\n--- Fallback Example ---");
    set_locale("en").expect("Failed to set English locale");
    let missing_key = t("nonexistent.key");
    println!("Missing key fallback: {}", missing_key);

    println!("\n=== Example Complete ===");
}