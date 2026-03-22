//! Language definitions for internationalization
//!
//! This module defines the supported languages and provides utilities
//! for language detection and conversion.

use std::fmt;
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Supported languages for internationalization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Language {
    /// English (default)
    #[default]
    English,
    /// Simplified Chinese (简体中文)
    SimplifiedChinese,
    /// Traditional Chinese (繁體中文)
    TraditionalChinese,
    /// French (Français)
    French,
    /// German (Deutsch)
    German,
    /// Russian (Русский)
    Russian,
}

impl Language {
    /// Get the ISO 639-1/639-2 language code
    pub fn code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::SimplifiedChinese => "zh-CN",
            Language::TraditionalChinese => "zh-TW",
            Language::French => "fr",
            Language::German => "de",
            Language::Russian => "ru",
        }
    }

    /// Get the native name of the language
    pub fn native_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::SimplifiedChinese => "简体中文",
            Language::TraditionalChinese => "繁體中文",
            Language::French => "Français",
            Language::German => "Deutsch",
            Language::Russian => "Русский",
        }
    }

    /// Get the English name of the language
    pub fn english_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::SimplifiedChinese => "Simplified Chinese",
            Language::TraditionalChinese => "Traditional Chinese",
            Language::French => "French",
            Language::German => "German",
            Language::Russian => "Russian",
        }
    }

    /// Check if the language uses right-to-left text direction
    pub fn is_rtl(&self) -> bool {
        match self {
            Language::English
            | Language::SimplifiedChinese
            | Language::TraditionalChinese
            | Language::French
            | Language::German
            | Language::Russian => false,
        }
    }

    /// Get all supported languages
    pub fn all() -> &'static [Language] {
        &[
            Language::English,
            Language::SimplifiedChinese,
            Language::TraditionalChinese,
            Language::French,
            Language::German,
            Language::Russian,
        ]
    }

    /// Try to detect language from system locale
    pub fn from_system_locale() -> Option<Language> {
        std::env::var("LANG")
            .or_else(|_| std::env::var("LC_ALL"))
            .or_else(|_| std::env::var("LC_MESSAGES"))
            .ok()
            .and_then(|locale| {
                let locale_lower = locale.to_lowercase();
                if locale_lower.starts_with("zh_cn") || locale_lower.starts_with("zh-cn") {
                    Some(Language::SimplifiedChinese)
                } else if locale_lower.starts_with("zh_tw") || locale_lower.starts_with("zh-tw")
                    || locale_lower.starts_with("zh_hk") || locale_lower.starts_with("zh-hk")
                {
                    Some(Language::TraditionalChinese)
                } else if locale_lower.starts_with("fr") {
                    Some(Language::French)
                } else if locale_lower.starts_with("de") {
                    Some(Language::German)
                } else if locale_lower.starts_with("ru") {
                    Some(Language::Russian)
                } else {
                    None
                }
            })
    }

    /// Parse from ISO language code
    pub fn from_code(code: &str) -> Option<Language> {
        match code.to_lowercase().as_str() {
            "en" | "en-us" | "en-gb" => Some(Language::English),
            "zh-cn" | "zh-hans" | "zh" => Some(Language::SimplifiedChinese),
            "zh-tw" | "zh-hant" | "zh-hk" => Some(Language::TraditionalChinese),
            "fr" | "fr-fr" | "fr-ca" => Some(Language::French),
            "de" | "de-de" | "de-at" => Some(Language::German),
            "ru" | "ru-ru" => Some(Language::Russian),
            _ => None,
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.english_name(), self.code())
    }
}

impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_code(s).ok_or_else(|| format!("Unknown language code: {}", s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_codes() {
        assert_eq!(Language::English.code(), "en");
        assert_eq!(Language::SimplifiedChinese.code(), "zh-CN");
        assert_eq!(Language::TraditionalChinese.code(), "zh-TW");
        assert_eq!(Language::French.code(), "fr");
        assert_eq!(Language::German.code(), "de");
        assert_eq!(Language::Russian.code(), "ru");
    }

    #[test]
    fn test_native_names() {
        assert_eq!(Language::English.native_name(), "English");
        assert_eq!(Language::SimplifiedChinese.native_name(), "简体中文");
        assert_eq!(Language::TraditionalChinese.native_name(), "繁體中文");
        assert_eq!(Language::French.native_name(), "Français");
        assert_eq!(Language::German.native_name(), "Deutsch");
        assert_eq!(Language::Russian.native_name(), "Русский");
    }

    #[test]
    fn test_from_code() {
        assert_eq!(Language::from_code("en"), Some(Language::English));
        assert_eq!(Language::from_code("zh-CN"), Some(Language::SimplifiedChinese));
        assert_eq!(Language::from_code("zh-TW"), Some(Language::TraditionalChinese));
        assert_eq!(Language::from_code("fr"), Some(Language::French));
        assert_eq!(Language::from_code("de"), Some(Language::German));
        assert_eq!(Language::from_code("ru"), Some(Language::Russian));
        assert_eq!(Language::from_code("unknown"), None);
    }

    #[test]
    fn test_default_language() {
        assert_eq!(Language::default(), Language::English);
    }
}
