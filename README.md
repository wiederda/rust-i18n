# rust-i18n

`rust-i18n` is a standalone, reusable Rust library for internationalization (i18n) of applications.

## Features

- **JSON-based language files**: Use JSON as the format for language files
- **Hierarchical keys**: Support for nested translation keys (e.g., `app.title`)
- **Placeholders**: Dynamic values in translations (e.g., `{name}`)
- **Fallback system**: Configurable fallback language for missing translations
- **Runtime language switching**: Change languages without restarting the application
- **Error handling**: Clear error messages for invalid JSON, missing files, and missing keys
- **Platform independent**: Works on all major platforms
- **Minimal dependencies**: Only `serde_json` and `lazy_static`

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust-i18n = { git = "https://github.com/wiederda/rust-i18n" }
```

Or as a GitHub dependency:

```toml
[dependencies]
rust-i18n = { git = "https://github.com/wiederda/rust-i18n.git", branch = "main" }
```

## Usage

### Basic Example

```rust
use rust_i18n::prelude::*;

fn main() {
    // Initialize the i18n system with your locales directory
    rust_i18n::init("locales").expect("Failed to initialize i18n");

    // Get a translation
    let text = rust_i18n::t("hello");
    println!("{}", text);

    // Switch language at runtime
    rust_i18n::set_locale("de").expect("Failed to set locale");
    let text = rust_i18n::t("hello");
    println!("{}", text);
}
```

### Directory Structure

```
locales/
├── de.json
├── en.json
└── fr.json
```

### Example Language File (en.json)

```json
{
    "hello": "Hello",
    "greeting": "Hello",
    "welcome.user": "Welcome {name}",
    "file.count": "You have {count} files",
    "status.success": "Operation completed successfully",
    "error.not_found": "Resource not found",
    "app.title": "My Application"
}
```

### Using Placeholders

```rust
use rust_i18n::prelude::*;
use std::collections::HashMap;

fn main() {
    rust_i18n::init("locales").unwrap();

    let mut placeholders = HashMap::new();
    placeholders.insert("name", "John");
    placeholders.insert("count", "5");

    let text = rust_i18n::t_with_placeholders("welcome.user", &placeholders);
    println!("{}", text); // Output: Welcome John
}
```

### Error Handling

```rust
use rust_i18n::prelude::*;

fn main() {
    match rust_i18n::init("locales") {
        Ok(_) => println!("i18n initialized"),
        Err(e) => eprintln!("Error: {}", e),
    }

    match rust_i18n::set_locale("de") {
        Ok(_) => println!("Locale set to German"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## API Documentation

### Functions

#### `init(locales_dir: &str) -> I18nResult<()>`

Initialize the i18n library with a locales directory.

**Parameters:**
- `locales_dir` - Path to the directory containing locale files (e.g., "locales")

**Returns:**
- `Ok(())` on success
- `Err(I18nError)` on failure

#### `t(key: &str) -> String`

Get a translation by key.

**Parameters:**
- `key` - The translation key (e.g., "hello.world")

**Returns:**
- The translated string, or the key itself if no translation is found

#### `t_with_placeholders(key: &str, placeholders: &HashMap<&str, &str>) -> String`

Get a translation with placeholders replaced.

**Parameters:**
- `key` - The translation key
- `placeholders` - A map of placeholder names to values

**Returns:**
- The translated string with placeholders replaced

#### `set_locale(locale: &str) -> I18nResult<()>`

Set the current locale.

**Parameters:**
- `locale` - The locale to set (e.g., "en", "de", "fr")

**Returns:**
- `Ok(())` on success
- `Err(I18nError::MissingLocale)` if the locale is not loaded

#### `current_locale() -> String`

Get the current locale.

**Returns:**
- The current locale string

#### `set_default_locale(locale: &str)`

Set the default locale.

**Parameters:**
- `locale` - The default locale to set

#### `set_fallback_locale(locale: &str)`

Set the fallback locale.

**Parameters:**
- `locale` - The fallback locale to set

### Error Types

```rust
pub enum I18nError {
    InvalidJson(String),
    MissingLocale(String),
    MissingTranslation(String, String),
    IoError(String),
}
```

## Examples

See the [examples](examples/) directory for more complete examples:

- `examples/basic.rs` - Basic usage example
- `examples/advanced.rs` - Advanced features and error handling

Run an example with:

```bash
cargo run --example basic
cargo run --example advanced
```

## Testing

Run the tests with:

```bash
cargo test
```

Run tests with output:

```bash
cargo test -- --nocapture
```

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Future Enhancements

The following features are planned for future releases:

- Additional language file formats (YAML, TOML, etc.)
- Plural forms support
- Context-dependent translations
- Number and date formatting
- Automatic system language detection
- Tools for checking translation completeness