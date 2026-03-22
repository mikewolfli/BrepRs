//! Hot reload support for i18n translations
//!
//! This module provides runtime translation updates without requiring recompilation.
//! It supports loading translations from external files and watching for changes.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use super::language::Language;
use super::messages::MessageKey;

/// Hot reload manager for translations
pub struct I18nHotReload {
    /// Path to translation files directory
    translation_dir: PathBuf,
    /// Custom translations loaded at runtime
    custom_translations: Arc<RwLock<HashMap<Language, HashMap<MessageKey, String>>>>,
    /// Whether hot reload is enabled
    enabled: AtomicBool,
    /// Last reload time
    last_reload: RwLock<Instant>,
    /// Reload interval in seconds
    reload_interval: Duration,
    /// File modification times for change detection
    file_mtimes: RwLock<HashMap<PathBuf, std::time::SystemTime>>,
}

impl I18nHotReload {
    /// Create a new hot reload manager
    pub fn new<P: AsRef<Path>>(translation_dir: P) -> Self {
        Self {
            translation_dir: translation_dir.as_ref().to_path_buf(),
            custom_translations: Arc::new(RwLock::new(HashMap::new())),
            enabled: AtomicBool::new(false),
            last_reload: RwLock::new(Instant::now() - Duration::from_secs(3600)),
            reload_interval: Duration::from_secs(5),
            file_mtimes: RwLock::new(HashMap::new()),
        }
    }

    /// Enable or disable hot reload
    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::SeqCst);
    }

    /// Check if hot reload is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::SeqCst)
    }

    /// Set the reload interval
    pub fn set_reload_interval(&self, interval: Duration) {
        let mut last_reload = self.last_reload.write().unwrap();
        *last_reload = Instant::now() - interval;
    }

    /// Load translations from the translation directory
    pub fn load_translations(&self) -> Result<(), String> {
        if !self.translation_dir.exists() {
            return Err(format!(
                "Translation directory does not exist: {:?}",
                self.translation_dir
            ));
        }

        let mut custom = self.custom_translations.write().unwrap();
        let mut mtimes = self.file_mtimes.write().unwrap();

        for lang in Language::all() {
            let filename = format!("{}.json", lang.code());
            let path = self.translation_dir.join(&filename);

            if path.exists() {
                if let Ok(metadata) = std::fs::metadata(&path) {
                    if let Ok(modified) = metadata.modified() {
                        mtimes.insert(path.clone(), modified);
                    }
                }

                if let Ok(content) = std::fs::read_to_string(&path) {
                    if let Ok(translations) = serde_json::from_str::<HashMap<String, String>>(&content) {
                        let lang_translations: HashMap<MessageKey, String> = translations
                            .into_iter()
                            .filter_map(|(k, v)| {
                                parse_message_key(&k).map(|key| (key, v))
                            })
                            .collect();
                        
                        custom.insert(*lang, lang_translations);
                    }
                }
            }
        }

        Ok(())
    }

    /// Check if any translation files have been modified and reload if necessary
    pub fn check_and_reload(&self) -> bool {
        if !self.is_enabled() {
            return false;
        }

        let last_reload = self.last_reload.read().unwrap();
        if last_reload.elapsed() < self.reload_interval {
            return false;
        }
        drop(last_reload);

        let mut needs_reload = false;
        {
            let mtimes = self.file_mtimes.read().unwrap();
            for lang in Language::all() {
                let filename = format!("{}.json", lang.code());
                let path = self.translation_dir.join(&filename);

                if path.exists() {
                    if let Ok(metadata) = std::fs::metadata(&path) {
                        if let Ok(modified) = metadata.modified() {
                            if let Some(&last_mtime) = mtimes.get(&path) {
                                if modified > last_mtime {
                                    needs_reload = true;
                                    break;
                                }
                            } else {
                                needs_reload = true;
                                break;
                            }
                        }
                    }
                }
            }
        }

        if needs_reload {
            if self.load_translations().is_ok() {
                let mut last_reload = self.last_reload.write().unwrap();
                *last_reload = Instant::now();
                return true;
            }
        }

        false
    }

    /// Get a custom translation for a key
    pub fn get_translation(&self, lang: Language, key: MessageKey) -> Option<String> {
        let custom = self.custom_translations.read().unwrap();
        custom.get(&lang).and_then(|t| t.get(&key).cloned())
    }

    /// Add or update a translation at runtime
    pub fn set_translation(&self, lang: Language, key: MessageKey, value: String) {
        let mut custom = self.custom_translations.write().unwrap();
        custom.entry(lang).or_insert_with(HashMap::new).insert(key, value);
    }

    /// Remove a custom translation
    pub fn remove_translation(&self, lang: Language, key: &MessageKey) -> bool {
        let mut custom = self.custom_translations.write().unwrap();
        if let Some(translations) = custom.get_mut(&lang) {
            translations.remove(key).is_some()
        } else {
            false
        }
    }

    /// Clear all custom translations for a language
    pub fn clear_language(&self, lang: Language) {
        let mut custom = self.custom_translations.write().unwrap();
        custom.remove(&lang);
    }

    /// Clear all custom translations
    pub fn clear_all(&self) {
        let mut custom = self.custom_translations.write().unwrap();
        custom.clear();
    }

    /// Save custom translations to files
    pub fn save_translations(&self) -> Result<(), String> {
        if !self.translation_dir.exists() {
            std::fs::create_dir_all(&self.translation_dir)
                .map_err(|e| format!("Failed to create translation directory: {}", e))?;
        }

        let custom = self.custom_translations.read().unwrap();

        for (lang, translations) in custom.iter() {
            let filename = format!("{}.json", lang.code());
            let path = self.translation_dir.join(&filename);

            let string_translations: HashMap<String, String> = translations
                .iter()
                .map(|(k, v)| (message_key_to_string(*k), v.clone()))
                .collect();

            let content = serde_json::to_string_pretty(&string_translations)
                .map_err(|e| format!("Failed to serialize translations: {}", e))?;

            std::fs::write(&path, content)
                .map_err(|e| format!("Failed to write translation file: {}", e))?;
        }

        Ok(())
    }

    /// Get the translation directory path
    pub fn translation_dir(&self) -> &Path {
        &self.translation_dir
    }

    /// Set the translation directory path
    pub fn set_translation_dir<P: AsRef<Path>>(&mut self, dir: P) {
        self.translation_dir = dir.as_ref().to_path_buf();
    }
}

impl Default for I18nHotReload {
    fn default() -> Self {
        Self::new("./translations")
    }
}

/// Parse a string to a MessageKey
fn parse_message_key(s: &str) -> Option<MessageKey> {
    use MessageKey::*;
    
    match s {
        "ErrorUnknown" => Some(ErrorUnknown),
        "ErrorInvalidInput" => Some(ErrorInvalidInput),
        "ErrorInvalidShape" => Some(ErrorInvalidShape),
        "ErrorNullPointer" => Some(ErrorNullPointer),
        "ErrorIndexOutOfRange" => Some(ErrorIndexOutOfRange),
        "ErrorOperationFailed" => Some(ErrorOperationFailed),
        "ErrorNotImplemented" => Some(ErrorNotImplemented),
        "ErrorInternalError" => Some(ErrorInternalError),
        "ErrorFileNotFound" => Some(ErrorFileNotFound),
        "ErrorFileRead" => Some(ErrorFileRead),
        "ErrorFileWrite" => Some(ErrorFileWrite),
        "ErrorFileFormat" => Some(ErrorFileFormat),
        "ErrorFileCorrupted" => Some(ErrorFileCorrupted),
        "ErrorFilePermission" => Some(ErrorFilePermission),
        "ErrorInvalidPoint" => Some(ErrorInvalidPoint),
        "ErrorInvalidVector" => Some(ErrorInvalidVector),
        "ErrorInvalidCurve" => Some(ErrorInvalidCurve),
        "ErrorInvalidSurface" => Some(ErrorInvalidSurface),
        "ErrorInvalidEdge" => Some(ErrorInvalidEdge),
        "ErrorInvalidFace" => Some(ErrorInvalidFace),
        "ErrorInvalidWire" => Some(ErrorInvalidWire),
        "ErrorInvalidShell" => Some(ErrorInvalidShell),
        "ErrorInvalidSolid" => Some(ErrorInvalidSolid),
        "ErrorDegenerateGeometry" => Some(ErrorDegenerateGeometry),
        "ErrorSelfIntersection" => Some(ErrorSelfIntersection),
        "ErrorNonManifold" => Some(ErrorNonManifold),
        "ErrorTopologyInvalid" => Some(ErrorTopologyInvalid),
        "ErrorTopologyBroken" => Some(ErrorTopologyBroken),
        "ErrorTopologyOrientation" => Some(ErrorTopologyOrientation),
        "ErrorTopologyConnectivity" => Some(ErrorTopologyConnectivity),
        "ErrorBooleanFuse" => Some(ErrorBooleanFuse),
        "ErrorBooleanCut" => Some(ErrorBooleanCut),
        "ErrorBooleanCommon" => Some(ErrorBooleanCommon),
        "ErrorBooleanSection" => Some(ErrorBooleanSection),
        "ErrorBooleanOverlap" => Some(ErrorBooleanOverlap),
        "ErrorMeshGeneration" => Some(ErrorMeshGeneration),
        "ErrorMeshInvalid" => Some(ErrorMeshInvalid),
        "ErrorMeshDegenerate" => Some(ErrorMeshDegenerate),
        "ErrorMeshSelfIntersection" => Some(ErrorMeshSelfIntersection),
        "ErrorFilletFailed" => Some(ErrorFilletFailed),
        "ErrorChamferFailed" => Some(ErrorChamferFailed),
        "ErrorOffsetFailed" => Some(ErrorOffsetFailed),
        "ErrorDraftFailed" => Some(ErrorDraftFailed),
        "ErrorShellFailed" => Some(ErrorShellFailed),
        "ErrorThicknessFailed" => Some(ErrorThicknessFailed),
        "ErrorConstraintUnsatisfied" => Some(ErrorConstraintUnsatisfied),
        "ErrorConstraintOverConstrained" => Some(ErrorConstraintOverConstrained),
        "ErrorConstraintUnderConstrained" => Some(ErrorConstraintUnderConstrained),
        "ErrorConstraintConflict" => Some(ErrorConstraintConflict),
        "WarningPrecisionLoss" => Some(WarningPrecisionLoss),
        "WarningApproximationUsed" => Some(WarningApproximationUsed),
        "WarningDegenerateResult" => Some(WarningDegenerateResult),
        "WarningLowQuality" => Some(WarningLowQuality),
        "WarningIncompleteOperation" => Some(WarningIncompleteOperation),
        "WarningDeprecated" => Some(WarningDeprecated),
        "InfoOperationStarted" => Some(InfoOperationStarted),
        "InfoOperationCompleted" => Some(InfoOperationCompleted),
        "InfoOperationCancelled" => Some(InfoOperationCancelled),
        "InfoProgress" => Some(InfoProgress),
        "InfoReady" => Some(InfoReady),
        "InfoLoading" => Some(InfoLoading),
        "InfoSaving" => Some(InfoSaving),
        "InfoExporting" => Some(InfoExporting),
        "InfoImporting" => Some(InfoImporting),
        "LabelFile" => Some(LabelFile),
        "LabelEdit" => Some(LabelEdit),
        "LabelView" => Some(LabelView),
        "LabelTools" => Some(LabelTools),
        "LabelHelp" => Some(LabelHelp),
        "LabelNew" => Some(LabelNew),
        "LabelOpen" => Some(LabelOpen),
        "LabelSave" => Some(LabelSave),
        "LabelSaveAs" => Some(LabelSaveAs),
        "LabelExport" => Some(LabelExport),
        "LabelImport" => Some(LabelImport),
        "LabelClose" => Some(LabelClose),
        "LabelExit" => Some(LabelExit),
        "LabelUndo" => Some(LabelUndo),
        "LabelRedo" => Some(LabelRedo),
        "LabelCut" => Some(LabelCut),
        "LabelCopy" => Some(LabelCopy),
        "LabelPaste" => Some(LabelPaste),
        "LabelDelete" => Some(LabelDelete),
        "LabelSelectAll" => Some(LabelSelectAll),
        "LabelPreferences" => Some(LabelPreferences),
        "LabelAbout" => Some(LabelAbout),
        "ShapeVertex" => Some(ShapeVertex),
        "ShapeEdge" => Some(ShapeEdge),
        "ShapeWire" => Some(ShapeWire),
        "ShapeFace" => Some(ShapeFace),
        "ShapeShell" => Some(ShapeShell),
        "ShapeSolid" => Some(ShapeSolid),
        "ShapeCompound" => Some(ShapeCompound),
        "ShapeCompSolid" => Some(ShapeCompSolid),
        "GeomPoint" => Some(GeomPoint),
        "GeomLine" => Some(GeomLine),
        "GeomCircle" => Some(GeomCircle),
        "GeomEllipse" => Some(GeomEllipse),
        "GeomBezier" => Some(GeomBezier),
        "GeomBSpline" => Some(GeomBSpline),
        "GeomNurbs" => Some(GeomNurbs),
        "GeomPlane" => Some(GeomPlane),
        "GeomCylinder" => Some(GeomCylinder),
        "GeomSphere" => Some(GeomSphere),
        "GeomCone" => Some(GeomCone),
        "GeomTorus" => Some(GeomTorus),
        "OpBooleanFuse" => Some(OpBooleanFuse),
        "OpBooleanCut" => Some(OpBooleanCut),
        "OpBooleanCommon" => Some(OpBooleanCommon),
        "OpBooleanSection" => Some(OpBooleanSection),
        "OpFillet" => Some(OpFillet),
        "OpChamfer" => Some(OpChamfer),
        "OpOffset" => Some(OpOffset),
        "OpDraft" => Some(OpDraft),
        "OpShell" => Some(OpShell),
        "OpThickness" => Some(OpThickness),
        "OpMirror" => Some(OpMirror),
        "OpRotate" => Some(OpRotate),
        "OpTranslate" => Some(OpTranslate),
        "OpScale" => Some(OpScale),
        "UnitMillimeter" => Some(UnitMillimeter),
        "UnitCentimeter" => Some(UnitCentimeter),
        "UnitMeter" => Some(UnitMeter),
        "UnitInch" => Some(UnitInch),
        "UnitFoot" => Some(UnitFoot),
        "UnitDegree" => Some(UnitDegree),
        "UnitRadian" => Some(UnitRadian),
        _ => None,
    }
}

/// Convert a MessageKey to a string
fn message_key_to_string(key: MessageKey) -> String {
    use MessageKey::*;
    
    match key {
        ErrorUnknown => "ErrorUnknown",
        ErrorInvalidInput => "ErrorInvalidInput",
        ErrorInvalidShape => "ErrorInvalidShape",
        ErrorNullPointer => "ErrorNullPointer",
        ErrorIndexOutOfRange => "ErrorIndexOutOfRange",
        ErrorOperationFailed => "ErrorOperationFailed",
        ErrorNotImplemented => "ErrorNotImplemented",
        ErrorInternalError => "ErrorInternalError",
        ErrorFileNotFound => "ErrorFileNotFound",
        ErrorFileRead => "ErrorFileRead",
        ErrorFileWrite => "ErrorFileWrite",
        ErrorFileFormat => "ErrorFileFormat",
        ErrorFileCorrupted => "ErrorFileCorrupted",
        ErrorFilePermission => "ErrorFilePermission",
        ErrorInvalidPoint => "ErrorInvalidPoint",
        ErrorInvalidVector => "ErrorInvalidVector",
        ErrorInvalidCurve => "ErrorInvalidCurve",
        ErrorInvalidSurface => "ErrorInvalidSurface",
        ErrorInvalidEdge => "ErrorInvalidEdge",
        ErrorInvalidFace => "ErrorInvalidFace",
        ErrorInvalidWire => "ErrorInvalidWire",
        ErrorInvalidShell => "ErrorInvalidShell",
        ErrorInvalidSolid => "ErrorInvalidSolid",
        ErrorDegenerateGeometry => "ErrorDegenerateGeometry",
        ErrorSelfIntersection => "ErrorSelfIntersection",
        ErrorNonManifold => "ErrorNonManifold",
        ErrorTopologyInvalid => "ErrorTopologyInvalid",
        ErrorTopologyBroken => "ErrorTopologyBroken",
        ErrorTopologyOrientation => "ErrorTopologyOrientation",
        ErrorTopologyConnectivity => "ErrorTopologyConnectivity",
        ErrorBooleanFuse => "ErrorBooleanFuse",
        ErrorBooleanCut => "ErrorBooleanCut",
        ErrorBooleanCommon => "ErrorBooleanCommon",
        ErrorBooleanSection => "ErrorBooleanSection",
        ErrorBooleanOverlap => "ErrorBooleanOverlap",
        ErrorMeshGeneration => "ErrorMeshGeneration",
        ErrorMeshInvalid => "ErrorMeshInvalid",
        ErrorMeshDegenerate => "ErrorMeshDegenerate",
        ErrorMeshSelfIntersection => "ErrorMeshSelfIntersection",
        ErrorFilletFailed => "ErrorFilletFailed",
        ErrorChamferFailed => "ErrorChamferFailed",
        ErrorOffsetFailed => "ErrorOffsetFailed",
        ErrorDraftFailed => "ErrorDraftFailed",
        ErrorShellFailed => "ErrorShellFailed",
        ErrorThicknessFailed => "ErrorThicknessFailed",
        ErrorConstraintUnsatisfied => "ErrorConstraintUnsatisfied",
        ErrorConstraintOverConstrained => "ErrorConstraintOverConstrained",
        ErrorConstraintUnderConstrained => "ErrorConstraintUnderConstrained",
        ErrorConstraintConflict => "ErrorConstraintConflict",
        WarningPrecisionLoss => "WarningPrecisionLoss",
        WarningApproximationUsed => "WarningApproximationUsed",
        WarningDegenerateResult => "WarningDegenerateResult",
        WarningLowQuality => "WarningLowQuality",
        WarningIncompleteOperation => "WarningIncompleteOperation",
        WarningDeprecated => "WarningDeprecated",
        InfoOperationStarted => "InfoOperationStarted",
        InfoOperationCompleted => "InfoOperationCompleted",
        InfoOperationCancelled => "InfoOperationCancelled",
        InfoProgress => "InfoProgress",
        InfoReady => "InfoReady",
        InfoLoading => "InfoLoading",
        InfoSaving => "InfoSaving",
        InfoExporting => "InfoExporting",
        InfoImporting => "InfoImporting",
        LabelFile => "LabelFile",
        LabelEdit => "LabelEdit",
        LabelView => "LabelView",
        LabelTools => "LabelTools",
        LabelHelp => "LabelHelp",
        LabelNew => "LabelNew",
        LabelOpen => "LabelOpen",
        LabelSave => "LabelSave",
        LabelSaveAs => "LabelSaveAs",
        LabelExport => "LabelExport",
        LabelImport => "LabelImport",
        LabelClose => "LabelClose",
        LabelExit => "LabelExit",
        LabelUndo => "LabelUndo",
        LabelRedo => "LabelRedo",
        LabelCut => "LabelCut",
        LabelCopy => "LabelCopy",
        LabelPaste => "LabelPaste",
        LabelDelete => "LabelDelete",
        LabelSelectAll => "LabelSelectAll",
        LabelPreferences => "LabelPreferences",
        LabelAbout => "LabelAbout",
        ShapeVertex => "ShapeVertex",
        ShapeEdge => "ShapeEdge",
        ShapeWire => "ShapeWire",
        ShapeFace => "ShapeFace",
        ShapeShell => "ShapeShell",
        ShapeSolid => "ShapeSolid",
        ShapeCompound => "ShapeCompound",
        ShapeCompSolid => "ShapeCompSolid",
        GeomPoint => "GeomPoint",
        GeomLine => "GeomLine",
        GeomCircle => "GeomCircle",
        GeomEllipse => "GeomEllipse",
        GeomBezier => "GeomBezier",
        GeomBSpline => "GeomBSpline",
        GeomNurbs => "GeomNurbs",
        GeomPlane => "GeomPlane",
        GeomCylinder => "GeomCylinder",
        GeomSphere => "GeomSphere",
        GeomCone => "GeomCone",
        GeomTorus => "GeomTorus",
        OpBooleanFuse => "OpBooleanFuse",
        OpBooleanCut => "OpBooleanCut",
        OpBooleanCommon => "OpBooleanCommon",
        OpBooleanSection => "OpBooleanSection",
        OpFillet => "OpFillet",
        OpChamfer => "OpChamfer",
        OpOffset => "OpOffset",
        OpDraft => "OpDraft",
        OpShell => "OpShell",
        OpThickness => "OpThickness",
        OpMirror => "OpMirror",
        OpRotate => "OpRotate",
        OpTranslate => "OpTranslate",
        OpScale => "OpScale",
        UnitMillimeter => "UnitMillimeter",
        UnitCentimeter => "UnitCentimeter",
        UnitMeter => "UnitMeter",
        UnitInch => "UnitInch",
        UnitFoot => "UnitFoot",
        UnitDegree => "UnitDegree",
        UnitRadian => "UnitRadian",
    }.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hot_reload_creation() {
        let hot_reload = I18nHotReload::new("./translations");
        assert!(!hot_reload.is_enabled());
    }

    #[test]
    fn test_set_translation() {
        let hot_reload = I18nHotReload::default();
        
        hot_reload.set_translation(
            Language::English,
            MessageKey::ErrorInvalidShape,
            "Custom invalid shape message".to_string(),
        );
        
        let translation = hot_reload.get_translation(Language::English, MessageKey::ErrorInvalidShape);
        assert_eq!(translation, Some("Custom invalid shape message".to_string()));
    }

    #[test]
    fn test_remove_translation() {
        let hot_reload = I18nHotReload::default();
        
        hot_reload.set_translation(
            Language::English,
            MessageKey::ErrorInvalidShape,
            "Test".to_string(),
        );
        
        assert!(hot_reload.remove_translation(Language::English, &MessageKey::ErrorInvalidShape));
        assert!(hot_reload.get_translation(Language::English, MessageKey::ErrorInvalidShape).is_none());
    }

    #[test]
    fn test_clear_language() {
        let hot_reload = I18nHotReload::default();
        
        hot_reload.set_translation(Language::English, MessageKey::ErrorInvalidShape, "Test".to_string());
        hot_reload.set_translation(Language::English, MessageKey::ErrorInvalidInput, "Test2".to_string());
        
        hot_reload.clear_language(Language::English);
        
        assert!(hot_reload.get_translation(Language::English, MessageKey::ErrorInvalidShape).is_none());
        assert!(hot_reload.get_translation(Language::English, MessageKey::ErrorInvalidInput).is_none());
    }

    #[test]
    fn test_message_key_parsing() {
        assert_eq!(parse_message_key("ErrorInvalidShape"), Some(MessageKey::ErrorInvalidShape));
        assert_eq!(parse_message_key("ErrorFileNotFound"), Some(MessageKey::ErrorFileNotFound));
        assert_eq!(parse_message_key("UnknownKey"), None);
    }

    #[test]
    fn test_message_key_to_string() {
        assert_eq!(message_key_to_string(MessageKey::ErrorInvalidShape), "ErrorInvalidShape");
        assert_eq!(message_key_to_string(MessageKey::ErrorFileNotFound), "ErrorFileNotFound");
    }
}
