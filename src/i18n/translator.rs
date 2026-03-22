//! Translator implementation for internationalization
//!
//! This module provides the main translation functionality with automatic
//! language detection and thread-safe language switching.

use std::sync::atomic::{AtomicUsize, Ordering};

use super::language::Language;
use super::messages::{get_translations, MessageKey};

/// Global language setting (stored as index for atomic operations)
static CURRENT_LANGUAGE_INDEX: AtomicUsize = AtomicUsize::new(0);

/// Language index mapping for atomic operations
const LANGUAGE_INDEXES: &[Language] = &[
    Language::English,
    Language::SimplifiedChinese,
    Language::TraditionalChinese,
    Language::French,
    Language::German,
    Language::Russian,
];

/// Get the index of a language
fn language_to_index(lang: Language) -> usize {
    LANGUAGE_INDEXES.iter().position(|&l| l == lang).unwrap_or(0)
}

/// Get the language from an index
fn index_to_language(index: usize) -> Language {
    LANGUAGE_INDEXES.get(index).copied().unwrap_or(Language::English)
}

/// Internationalization manager
pub struct I18n;

impl I18n {
    /// Initialize the i18n system with automatic language detection
    /// 
    /// This function attempts to detect the system language and set it.
    /// If no matching language is found, it defaults to English.
    pub fn init() {
        if let Some(lang) = Language::from_system_locale() {
            Self::set_language(lang);
        } else {
            Self::set_language(Language::English);
        }
    }

    /// Set the current language
    pub fn set_language(lang: Language) {
        CURRENT_LANGUAGE_INDEX.store(language_to_index(lang), Ordering::SeqCst);
    }

    /// Get the current language
    pub fn current_language() -> Language {
        index_to_language(CURRENT_LANGUAGE_INDEX.load(Ordering::SeqCst))
    }

    /// Translate a message key to the current language
    pub fn tr(key: MessageKey) -> &'static str {
        let lang = Self::current_language();
        let translations = get_translations(lang);
        
        translations.get(&key).copied().unwrap_or_else(|| {
            get_translations(Language::English).get(&key).copied().unwrap_or("???")
        })
    }

    /// Translate a message key with format parameters
    /// 
    /// # Parameters
    /// - `key`: The message key to translate
    /// - `params`: Parameters to substitute into the message (replaces `{}`)
    pub fn tr_fmt(key: MessageKey, params: &[&str]) -> String {
        let template = Self::tr(key);
        
        let mut result = template.to_string();
        for param in params {
            if let Some(pos) = result.find("{}") {
                result.replace_range(pos..pos + 2, param);
            }
        }
        
        result
    }

    /// Translate a message key with a single parameter
    pub fn tr_one(key: MessageKey, param: &str) -> String {
        Self::tr_fmt(key, &[param])
    }

    /// Translate a message key with two parameters
    pub fn tr_two(key: MessageKey, param1: &str, param2: &str) -> String {
        Self::tr_fmt(key, &[param1, param2])
    }

    /// Translate a message key to a specific language
    pub fn tr_to(key: MessageKey, lang: Language) -> &'static str {
        let translations = get_translations(lang);
        
        translations.get(&key).copied().unwrap_or_else(|| {
            get_translations(Language::English).get(&key).copied().unwrap_or("???")
        })
    }

    /// Check if a translation exists for the current language
    pub fn has_translation(key: MessageKey) -> bool {
        let lang = Self::current_language();
        let translations = get_translations(lang);
        translations.contains_key(&key)
    }

    /// Get all available languages
    pub fn available_languages() -> &'static [Language] {
        Language::all()
    }

    /// Detect and return the system language without setting it
    pub fn detect_system_language() -> Language {
        Language::from_system_locale().unwrap_or(Language::English)
    }

    /// Parse a language code and set the language if valid
    /// Returns true if the language was set successfully
    pub fn set_language_from_code(code: &str) -> bool {
        if let Some(lang) = Language::from_code(code) {
            Self::set_language(lang);
            true
        } else {
            false
        }
    }
}

impl Default for I18n {
    fn default() -> Self {
        Self::init();
        Self
    }
}

/// Convenience macro for translating messages
#[macro_export]
macro_rules! tr {
    ($key:expr) => {
        $crate::i18n::I18n::tr($key)
    };
}

/// Convenience macro for translating messages with parameters
#[macro_export]
macro_rules! tr_fmt {
    ($key:expr, $($param:expr),+ $(,)?) => {
        $crate::i18n::I18n::tr_fmt($key, &[$($param),+])
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_detection() {
        let detected = I18n::detect_system_language();
        assert!(Language::all().contains(&detected));
    }

    #[test]
    fn test_language_switching() {
        I18n::set_language(Language::English);
        assert_eq!(I18n::current_language(), Language::English);
        
        I18n::set_language(Language::SimplifiedChinese);
        assert_eq!(I18n::current_language(), Language::SimplifiedChinese);
        
        I18n::set_language(Language::German);
        assert_eq!(I18n::current_language(), Language::German);
    }

    #[test]
    fn test_translation() {
        I18n::set_language(Language::English);
        assert_eq!(I18n::tr(MessageKey::ErrorInvalidShape), "Invalid shape");
        
        I18n::set_language(Language::SimplifiedChinese);
        assert_eq!(I18n::tr(MessageKey::ErrorInvalidShape), "无效的形状");
        
        I18n::set_language(Language::French);
        assert_eq!(I18n::tr(MessageKey::ErrorInvalidShape), "Forme invalide");
    }

    #[test]
    fn test_format_translation() {
        I18n::set_language(Language::English);
        let msg = I18n::tr_one(MessageKey::ErrorFileNotFound, "test.step");
        assert_eq!(msg, "File not found: test.step");
        
        I18n::set_language(Language::SimplifiedChinese);
        let msg = I18n::tr_one(MessageKey::ErrorFileNotFound, "test.step");
        assert_eq!(msg, "文件未找到: test.step");
    }

    #[test]
    fn test_set_language_from_code() {
        assert!(I18n::set_language_from_code("zh-CN"));
        assert_eq!(I18n::current_language(), Language::SimplifiedChinese);
        
        assert!(I18n::set_language_from_code("fr"));
        assert_eq!(I18n::current_language(), Language::French);
        
        assert!(!I18n::set_language_from_code("unknown"));
    }

    #[test]
    fn test_available_languages() {
        let languages = I18n::available_languages();
        assert_eq!(languages.len(), 6);
        assert!(languages.contains(&Language::English));
        assert!(languages.contains(&Language::SimplifiedChinese));
        assert!(languages.contains(&Language::TraditionalChinese));
        assert!(languages.contains(&Language::French));
        assert!(languages.contains(&Language::German));
        assert!(languages.contains(&Language::Russian));
    }
}
