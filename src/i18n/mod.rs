//! Internationalization (i18n) Module
//!
//! This module provides comprehensive internationalization support for BrepRs,
//! enabling multi-language support for error messages, warnings, and user-facing strings.
//!
//! # Supported Languages
//!
//! - English (en) - Default
//! - Simplified Chinese (zh-CN)
//! - Traditional Chinese (zh-TW)
//! - French (fr)
//! - German (de)
//! - Russian (ru)
//!
//! # Features
//!
//! - Automatic system language detection
//! - Thread-safe language switching
//! - Hot reload support for runtime translation updates
//! - Fallback to English for missing translations
//!
//! # Usage
//!
//! ```rust
//! use breprs::i18n::{I18n, Language, MessageKey};
//!
//! // Initialize with automatic system language detection
//! I18n::init();
//!
//! // Or set a specific language
//! I18n::set_language(Language::SimplifiedChinese);
//!
//! // Get a translated message
//! let msg = I18n::tr(MessageKey::ErrorInvalidShape);
//! println!("{}", msg); // Output: Invalid shape
//! 
//! // Format a message with parameters
//! let msg = I18n::tr_fmt(MessageKey::ErrorFileNotFound, &["model.step"]);
//! println!("{}", msg); // Output: File not found: model.step
//! ```
//!
//! # Hot Reload
//!
//! ```rust
//! use breprs::i18n::{I18nHotReload, Language, MessageKey};
//!
//! // Create a hot reload manager
//! let hot_reload = I18nHotReload::new("./translations");
//!
//! // Enable hot reload
//! hot_reload.set_enabled(true);
//!
//! // Add custom translations at runtime
//! hot_reload.set_translation(
//!     Language::English,
//!     MessageKey::ErrorInvalidShape,
//!     "Custom error message".to_string(),
//! );
//!
//! // Save translations to files
//! hot_reload.save_translations().unwrap();
//! ```

mod language;
mod messages;
mod translator;

#[cfg(feature = "serde")]
mod hot_reload;

pub use language::Language;
pub use messages::MessageKey;
pub use translator::I18n;

#[cfg(feature = "serde")]
pub use hot_reload::I18nHotReload;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_switching() {
        I18n::set_language(Language::English);
        assert_eq!(I18n::tr(MessageKey::ErrorInvalidShape), "Invalid shape");

        I18n::set_language(Language::SimplifiedChinese);
        assert_eq!(I18n::tr(MessageKey::ErrorInvalidShape), "无效的形状");

        I18n::set_language(Language::TraditionalChinese);
        assert_eq!(I18n::tr(MessageKey::ErrorInvalidShape), "無效的形狀");

        I18n::set_language(Language::French);
        assert_eq!(I18n::tr(MessageKey::ErrorInvalidShape), "Forme invalide");

        I18n::set_language(Language::German);
        assert_eq!(I18n::tr(MessageKey::ErrorInvalidShape), "Ungültige Form");

        I18n::set_language(Language::Russian);
        assert_eq!(I18n::tr(MessageKey::ErrorInvalidShape), "Недопустимая форма");
    }

    #[test]
    fn test_format_message() {
        I18n::set_language(Language::English);
        let msg = I18n::tr_fmt(MessageKey::ErrorFileNotFound, &["test.step"]);
        assert!(msg.contains("test.step"));

        I18n::set_language(Language::SimplifiedChinese);
        let msg = I18n::tr_fmt(MessageKey::ErrorFileNotFound, &["test.step"]);
        assert!(msg.contains("test.step"));
    }

    #[test]
    fn test_language_detection() {
        let detected = I18n::detect_system_language();
        assert!(Language::all().contains(&detected));
    }

    #[test]
    fn test_set_language_from_code() {
        assert!(I18n::set_language_from_code("zh-CN"));
        assert_eq!(I18n::current_language(), Language::SimplifiedChinese);

        assert!(I18n::set_language_from_code("fr"));
        assert_eq!(I18n::current_language(), Language::French);

        assert!(!I18n::set_language_from_code("unknown"));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_hot_reload() {
        let hot_reload = I18nHotReload::default();
        
        hot_reload.set_translation(
            Language::English,
            MessageKey::ErrorInvalidShape,
            "Custom message".to_string(),
        );
        
        let translation = hot_reload.get_translation(Language::English, MessageKey::ErrorInvalidShape);
        assert_eq!(translation, Some("Custom message".to_string()));
    }
}
