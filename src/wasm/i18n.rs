//! WebAssembly bindings for internationalization

use wasm_bindgen::prelude::*;

#[cfg(feature = "serde-wasm-bindgen")]
use serde_wasm_bindgen;

/// Language codes supported by the library
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    /// English
    English,
    /// Simplified Chinese
    SimplifiedChinese,
    /// Traditional Chinese
    TraditionalChinese,
    /// French
    French,
    /// German
    German,
    /// Russian
    Russian,
}

impl Language {
    /// Get language code as string
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

    /// Get language name
    pub fn name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::SimplifiedChinese => "简体中文",
            Language::TraditionalChinese => "繁體中文",
            Language::French => "Français",
            Language::German => "Deutsch",
            Language::Russian => "Русский",
        }
    }
}

/// Message keys for translation
#[wasm_bindgen]
pub enum MessageKey {
    /// Unknown error
    ErrorUnknown,
    /// File not found
    ErrorFileNotFound,
    /// Invalid format
    ErrorInvalidFormat,
    /// Operation failed
    ErrorOperationFailed,
    /// File label
    LabelFile,
    /// Shape label
    LabelShape,
    /// Vertex label
    LabelVertex,
    /// Edge label
    LabelEdge,
    /// Face label
    LabelFace,
    /// Solid label
    LabelSolid,
    /// Boolean fuse operation
    OpBooleanFuse,
    /// Boolean cut operation
    OpBooleanCut,
    /// Boolean common operation
    OpBooleanCommon,
    /// Boolean section operation
    OpBooleanSection,
    /// Primitive created
    OpPrimitiveCreated,
    /// Shape created
    OpShapeCreated,
    /// Operation completed
    OpCompleted,
    /// Operation cancelled
    OpCancelled,
}

impl MessageKey {
    /// Get message key as string
    pub fn key(&self) -> &'static str {
        match self {
            MessageKey::ErrorUnknown => "ErrorUnknown",
            MessageKey::ErrorFileNotFound => "ErrorFileNotFound",
            MessageKey::ErrorInvalidFormat => "ErrorInvalidFormat",
            MessageKey::ErrorOperationFailed => "ErrorOperationFailed",
            MessageKey::LabelFile => "LabelFile",
            MessageKey::LabelShape => "LabelShape",
            MessageKey::LabelVertex => "LabelVertex",
            MessageKey::LabelEdge => "LabelEdge",
            MessageKey::LabelFace => "LabelFace",
            MessageKey::LabelSolid => "LabelSolid",
            MessageKey::OpBooleanFuse => "OpBooleanFuse",
            MessageKey::OpBooleanCut => "OpBooleanCut",
            MessageKey::OpBooleanCommon => "OpBooleanCommon",
            MessageKey::OpBooleanSection => "OpBooleanSection",
            MessageKey::OpPrimitiveCreated => "OpPrimitiveCreated",
            MessageKey::OpShapeCreated => "OpShapeCreated",
            MessageKey::OpCompleted => "OpCompleted",
            MessageKey::OpCancelled => "OpCancelled",
        }
    }
}

/// Translation dictionary
#[wasm_bindgen]
pub struct TranslationDictionary {
    language: Language,
    translations: std::collections::HashMap<String, String>,
}

#[wasm_bindgen]
impl TranslationDictionary {
    /// Create a new translation dictionary
    #[wasm_bindgen(constructor)]
    pub fn new(language: Language) -> Self {
        let translations = Self::get_translations(language);
        Self {
            language,
            translations,
        }
    }

    /// Get translations for a language
    fn get_translations(language: Language) -> std::collections::HashMap<String, String> {
        let mut translations = std::collections::HashMap::new();

        match language {
            Language::English => {
                translations.insert("ErrorUnknown".to_string(), "Unknown error".to_string());
                translations.insert("ErrorFileNotFound".to_string(), "File not found".to_string());
                translations.insert("ErrorInvalidFormat".to_string(), "Invalid format".to_string());
                translations.insert("ErrorOperationFailed".to_string(), "Operation failed".to_string());
                translations.insert("LabelFile".to_string(), "File".to_string());
                translations.insert("LabelShape".to_string(), "Shape".to_string());
                translations.insert("LabelVertex".to_string(), "Vertex".to_string());
                translations.insert("LabelEdge".to_string(), "Edge".to_string());
                translations.insert("LabelFace".to_string(), "Face".to_string());
                translations.insert("LabelSolid".to_string(), "Solid".to_string());
                translations.insert("OpBooleanFuse".to_string(), "Fuse".to_string());
                translations.insert("OpBooleanCut".to_string(), "Cut".to_string());
                translations.insert("OpBooleanCommon".to_string(), "Common".to_string());
                translations.insert("OpBooleanSection".to_string(), "Section".to_string());
                translations.insert("OpPrimitiveCreated".to_string(), "Primitive created".to_string());
                translations.insert("OpShapeCreated".to_string(), "Shape created".to_string());
                translations.insert("OpCompleted".to_string(), "Operation completed".to_string());
                translations.insert("OpCancelled".to_string(), "Operation cancelled".to_string());
            }
            Language::SimplifiedChinese => {
                translations.insert("ErrorUnknown".to_string(), "未知错误".to_string());
                translations.insert("ErrorFileNotFound".to_string(), "文件未找到".to_string());
                translations.insert("ErrorInvalidFormat".to_string(), "无效格式".to_string());
                translations.insert("ErrorOperationFailed".to_string(), "操作失败".to_string());
                translations.insert("LabelFile".to_string(), "文件".to_string());
                translations.insert("LabelShape".to_string(), "形状".to_string());
                translations.insert("LabelVertex".to_string(), "顶点".to_string());
                translations.insert("LabelEdge".to_string(), "边".to_string());
                translations.insert("LabelFace".to_string(), "面".to_string());
                translations.insert("LabelSolid".to_string(), "实体".to_string());
                translations.insert("OpBooleanFuse".to_string(), "融合".to_string());
                translations.insert("OpBooleanCut".to_string(), "切割".to_string());
                translations.insert("OpBooleanCommon".to_string(), "相交".to_string());
                translations.insert("OpBooleanSection".to_string(), "截面".to_string());
                translations.insert("OpPrimitiveCreated".to_string(), "基本几何体已创建".to_string());
                translations.insert("OpShapeCreated".to_string(), "形状已创建".to_string());
                translations.insert("OpCompleted".to_string(), "操作已完成".to_string());
                translations.insert("OpCancelled".to_string(), "操作已取消".to_string());
            }
            Language::TraditionalChinese => {
                translations.insert("ErrorUnknown".to_string(), "未知錯誤".to_string());
                translations.insert("ErrorFileNotFound".to_string(), "檔案未找到".to_string());
                translations.insert("ErrorInvalidFormat".to_string(), "無效格式".to_string());
                translations.insert("ErrorOperationFailed".to_string(), "操作失敗".to_string());
                translations.insert("LabelFile".to_string(), "檔案".to_string());
                translations.insert("LabelShape".to_string(), "形狀".to_string());
                translations.insert("LabelVertex".to_string(), "頂點".to_string());
                translations.insert("LabelEdge".to_string(), "邊".to_string());
                translations.insert("LabelFace".to_string(), "面".to_string());
                translations.insert("LabelSolid".to_string(), "實體".to_string());
                translations.insert("OpBooleanFuse".to_string(), "融合".to_string());
                translations.insert("OpBooleanCut".to_string(), "切割".to_string());
                translations.insert("OpBooleanCommon".to_string(), "相交".to_string());
                translations.insert("OpBooleanSection".to_string(), "截面".to_string());
                translations.insert("OpPrimitiveCreated".to_string(), "基本幾何體已建立".to_string());
                translations.insert("OpShapeCreated".to_string(), "形狀已建立".to_string());
                translations.insert("OpCompleted".to_string(), "操作已完成".to_string());
                translations.insert("OpCancelled".to_string(), "操作已取消".to_string());
            }
            Language::French => {
                translations.insert("ErrorUnknown".to_string(), "Erreur inconnue".to_string());
                translations.insert("ErrorFileNotFound".to_string(), "Fichier non trouvé".to_string());
                translations.insert("ErrorInvalidFormat".to_string(), "Format invalide".to_string());
                translations.insert("ErrorOperationFailed".to_string(), "Opération échouée".to_string());
                translations.insert("LabelFile".to_string(), "Fichier".to_string());
                translations.insert("LabelShape".to_string(), "Forme".to_string());
                translations.insert("LabelVertex".to_string(), "Sommet".to_string());
                translations.insert("LabelEdge".to_string(), "Arête".to_string());
                translations.insert("LabelFace".to_string(), "Face".to_string());
                translations.insert("LabelSolid".to_string(), "Solide".to_string());
                translations.insert("OpBooleanFuse".to_string(), "Fusion".to_string());
                translations.insert("OpBooleanCut".to_string(), "Coupe".to_string());
                translations.insert("OpBooleanCommon".to_string(), "Commun".to_string());
                translations.insert("OpBooleanSection".to_string(), "Section".to_string());
                translations.insert("OpPrimitiveCreated".to_string(), "Primitive créé".to_string());
                translations.insert("OpShapeCreated".to_string(), "Forme créée".to_string());
                translations.insert("OpCompleted".to_string(), "Opération terminée".to_string());
                translations.insert("OpCancelled".to_string(), "Opération annulée".to_string());
            }
            Language::German => {
                translations.insert("ErrorUnknown".to_string(), "Unbekannter Fehler".to_string());
                translations.insert("ErrorFileNotFound".to_string(), "Datei nicht gefunden".to_string());
                translations.insert("ErrorInvalidFormat".to_string(), "Ungültiges Format".to_string());
                translations.insert("ErrorOperationFailed".to_string(), "Operation fehlgeschlagen".to_string());
                translations.insert("LabelFile".to_string(), "Datei".to_string());
                translations.insert("LabelShape".to_string(), "Form".to_string());
                translations.insert("LabelVertex".to_string(), "Scheitelpunkt".to_string());
                translations.insert("LabelEdge".to_string(), "Kante".to_string());
                translations.insert("LabelFace".to_string(), "Fläche".to_string());
                translations.insert("LabelSolid".to_string(), "Körper".to_string());
                translations.insert("OpBooleanFuse".to_string(), "Vereinigen".to_string());
                translations.insert("OpBooleanCut".to_string(), "Schneiden".to_string());
                translations.insert("OpBooleanCommon".to_string(), "Gemeinsam".to_string());
                translations.insert("OpBooleanSection".to_string(), "Schnitt".to_string());
                translations.insert("OpPrimitiveCreated".to_string(), "Primitiv erstellt".to_string());
                translations.insert("OpShapeCreated".to_string(), "Form erstellt".to_string());
                translations.insert("OpCompleted".to_string(), "Operation abgeschlossen".to_string());
                translations.insert("OpCancelled".to_string(), "Operation abgebrochen".to_string());
            }
            Language::Russian => {
                translations.insert("ErrorUnknown".to_string(), "Неизвестная ошибка".to_string());
                translations.insert("ErrorFileNotFound".to_string(), "Файл не найден".to_string());
                translations.insert("ErrorInvalidFormat".to_string(), "Неверный формат".to_string());
                translations.insert("ErrorOperationFailed".to_string(), "Операция не удалась".to_string());
                translations.insert("LabelFile".to_string(), "Файл".to_string());
                translations.insert("LabelShape".to_string(), "Форма".to_string());
                translations.insert("LabelVertex".to_string(), "Вершина".to_string());
                translations.insert("LabelEdge".to_string(), "Ребро".to_string());
                translations.insert("LabelFace".to_string(), "Грань".to_string());
                translations.insert("LabelSolid".to_string(), "Тело".to_string());
                translations.insert("OpBooleanFuse".to_string(), "Объединение".to_string());
                translations.insert("OpBooleanCut".to_string(), "Вырезание".to_string());
                translations.insert("OpBooleanCommon".to_string(), "Пересечение".to_string());
                translations.insert("OpBooleanSection".to_string(), "Сечение".to_string());
                translations.insert("OpPrimitiveCreated".to_string(), "Примитив создан".to_string());
                translations.insert("OpShapeCreated".to_string(), "Форма создана".to_string());
                translations.insert("OpCompleted".to_string(), "Операция завершена".to_string());
                translations.insert("OpCancelled".to_string(), "Операция отменена".to_string());
            }
        }

        translations
    }

    /// Get translation for a key
    #[wasm_bindgen(js_name = translate)]
    pub fn translate(&self, key: MessageKey) -> String {
        let key_str = key.key();
        self.translations
            .get(key_str)
            .cloned()
            .unwrap_or_else(|| key_str.to_string())
    }

    /// Get translation for a key string
    #[wasm_bindgen(js_name = translateKey)]
    pub fn translate_key(&self, key: String) -> String {
        self.translations
            .get(&key)
            .cloned()
            .unwrap_or_else(|| key.clone())
    }

    /// Get language
    #[wasm_bindgen(getter, js_name = language)]
    pub fn language(&self) -> Language {
        self.language
    }

    /// Get all translations
    #[wasm_bindgen(js_name = getAllTranslations)]
    pub fn get_all_translations(&self) -> JsValue {
        #[cfg(feature = "serde-wasm-bindgen")]
        {
            serde_wasm_bindgen::to_value(&self.translations).unwrap_or(JsValue::NULL)
        }
        #[cfg(not(feature = "serde-wasm-bindgen"))]
        {
            JsValue::NULL
        }
    }
}

/// Internationalization manager
#[wasm_bindgen(js_name = I18n)]
pub struct WasmI18n {
    current_language: Language,
    dictionaries: std::collections::HashMap<String, TranslationDictionary>,
}

#[wasm_bindgen(js_class = I18n)]
impl WasmI18n {
    /// Create a new I18n manager
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut dictionaries = std::collections::HashMap::new();
        
        for language in [
            Language::English,
            Language::SimplifiedChinese,
            Language::TraditionalChinese,
            Language::French,
            Language::German,
            Language::Russian,
        ] {
            dictionaries.insert(language.code().to_string(), TranslationDictionary::new(language));
        }

        Self {
            current_language: Language::English,
            dictionaries,
        }
    }

    /// Initialize I18n with default language
    #[wasm_bindgen(js_name = init)]
    pub fn init() -> Self {
        Self::new()
    }

    /// Set current language
    #[wasm_bindgen(js_name = setLanguage)]
    pub fn set_language(&mut self, language: Language) {
        self.current_language = language;
    }

    /// Get current language
    #[wasm_bindgen(getter, js_name = currentLanguage)]
    pub fn current_language(&self) -> Language {
        self.current_language
    }

    /// Translate a message key
    #[wasm_bindgen(js_name = translate)]
    pub fn translate(&self, key: MessageKey) -> String {
        let dict = self.dictionaries.get(self.current_language.code());
        match dict {
            Some(d) => d.translate(key),
            None => key.key().to_string(),
        }
    }

    /// Translate a key string
    #[wasm_bindgen(js_name = translateKey)]
    pub fn translate_key(&self, key: String) -> String {
        let dict = self.dictionaries.get(self.current_language.code());
        match dict {
            Some(d) => d.translate_key(key.clone()),
            None => key.clone(),
        }
    }

    /// Get available languages
    #[wasm_bindgen(js_name = getAvailableLanguages)]
    pub fn get_available_languages(&self) -> Vec<Language> {
        vec![
            Language::English,
            Language::SimplifiedChinese,
            Language::TraditionalChinese,
            Language::French,
            Language::German,
            Language::Russian,
        ]
    }

    /// Get language codes
    #[wasm_bindgen(js_name = getLanguageCodes)]
    pub fn get_language_codes(&self) -> Vec<String> {
        self.dictionaries.keys().cloned().collect()
    }

    /// Add custom translation
    #[wasm_bindgen(js_name = addCustomTranslation)]
    pub fn add_custom_translation(&mut self, language: Language, key: String, value: String) {
        if let Some(dict) = self.dictionaries.get_mut(language.code()) {
            dict.translations.insert(key, value);
        }
    }

    /// Get language name
    #[wasm_bindgen(js_name = getLanguageName)]
    pub fn get_language_name(&self, language: Language) -> String {
        language.name().to_string()
    }

    /// Get language code
    #[wasm_bindgen(js_name = getLanguageCode)]
    pub fn get_language_code(&self, language: Language) -> String {
        language.code().to_string()
    }
}

impl Default for WasmI18n {
    fn default() -> Self {
        Self::new()
    }
}
