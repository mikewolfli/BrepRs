//! PHP bindings for BrepRs using ext-php-rs
//!
//! This module provides PHP bindings for BrepRs.

use ext_php_rs::prelude::*;

use breprs::i18n::{I18n, Language, MessageKey};

/// Initialize i18n with automatic system language detection
#[php_function]
pub fn i18n_init() {
    I18n::init();
}

/// Set the current language
#[php_function]
pub fn i18n_set_language(lang_code: String) -> PhpResult<bool> {
    if let Some(lang) = Language::from_code(&lang_code) {
        I18n::set_language(lang);
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Get the current language code
#[php_function]
pub fn i18n_current_language() -> String {
    I18n::current_language().code().to_string()
}

/// Translate a message key
#[php_function]
pub fn i18n_translate(key: String) -> String {
    if let Ok(msg_key) = parse_message_key(&key) {
        I18n::tr(msg_key).to_string()
    } else {
        format!("Unknown key: {}", key)
    }
}

/// Get all available languages
#[php_function]
pub fn i18n_available_languages() -> Vec<String> {
    Language::all()
        .iter()
        .map(|l| l.code().to_string())
        .collect()
}

/// Parse message key from string
fn parse_message_key(s: &str) -> Result<MessageKey, ()> {
    use MessageKey::*;
    match s {
        "ErrorUnknown" => Ok(ErrorUnknown),
        "ErrorInvalidInput" => Ok(ErrorInvalidInput),
        "ErrorInvalidShape" => Ok(ErrorInvalidShape),
        "ErrorNullPointer" => Ok(ErrorNullPointer),
        "ErrorIndexOutOfRange" => Ok(ErrorIndexOutOfRange),
        "ErrorOperationFailed" => Ok(ErrorOperationFailed),
        "ErrorNotImplemented" => Ok(ErrorNotImplemented),
        "ErrorInternalError" => Ok(ErrorInternalError),
        "ErrorFileNotFound" => Ok(ErrorFileNotFound),
        "ErrorFileRead" => Ok(ErrorFileRead),
        "ErrorFileWrite" => Ok(ErrorFileWrite),
        "ErrorFileFormat" => Ok(ErrorFileFormat),
        "ErrorFileCorrupted" => Ok(ErrorFileCorrupted),
        "ErrorFilePermission" => Ok(ErrorFilePermission),
        "ErrorInvalidPoint" => Ok(ErrorInvalidPoint),
        "ErrorInvalidVector" => Ok(ErrorInvalidVector),
        "ErrorInvalidCurve" => Ok(ErrorInvalidCurve),
        "ErrorInvalidSurface" => Ok(ErrorInvalidSurface),
        "ErrorInvalidEdge" => Ok(ErrorInvalidEdge),
        "ErrorInvalidFace" => Ok(ErrorInvalidFace),
        "ErrorInvalidWire" => Ok(ErrorInvalidWire),
        "ErrorInvalidShell" => Ok(ErrorInvalidShell),
        "ErrorInvalidSolid" => Ok(ErrorInvalidSolid),
        "ErrorDegenerateGeometry" => Ok(ErrorDegenerateGeometry),
        "ErrorSelfIntersection" => Ok(ErrorSelfIntersection),
        "ErrorNonManifold" => Ok(ErrorNonManifold),
        "ErrorTopologyInvalid" => Ok(ErrorTopologyInvalid),
        "ErrorTopologyBroken" => Ok(ErrorTopologyBroken),
        "ErrorTopologyOrientation" => Ok(ErrorTopologyOrientation),
        "ErrorTopologyConnectivity" => Ok(ErrorTopologyConnectivity),
        "ErrorBooleanFuse" => Ok(ErrorBooleanFuse),
        "ErrorBooleanCut" => Ok(ErrorBooleanCut),
        "ErrorBooleanCommon" => Ok(ErrorBooleanCommon),
        "ErrorBooleanSection" => Ok(ErrorBooleanSection),
        "ErrorBooleanOverlap" => Ok(ErrorBooleanOverlap),
        "ErrorMeshGeneration" => Ok(ErrorMeshGeneration),
        "ErrorMeshInvalid" => Ok(ErrorMeshInvalid),
        "ErrorMeshDegenerate" => Ok(ErrorMeshDegenerate),
        "ErrorMeshSelfIntersection" => Ok(ErrorMeshSelfIntersection),
        "ErrorFilletFailed" => Ok(ErrorFilletFailed),
        "ErrorChamferFailed" => Ok(ErrorChamferFailed),
        "ErrorOffsetFailed" => Ok(ErrorOffsetFailed),
        "ErrorDraftFailed" => Ok(ErrorDraftFailed),
        "ErrorShellFailed" => Ok(ErrorShellFailed),
        "ErrorThicknessFailed" => Ok(ErrorThicknessFailed),
        "ErrorConstraintUnsatisfied" => Ok(ErrorConstraintUnsatisfied),
        "ErrorConstraintOverConstrained" => Ok(ErrorConstraintOverConstrained),
        "ErrorConstraintUnderConstrained" => Ok(ErrorConstraintUnderConstrained),
        "ErrorConstraintConflict" => Ok(ErrorConstraintConflict),
        "WarningPrecisionLoss" => Ok(WarningPrecisionLoss),
        "WarningApproximationUsed" => Ok(WarningApproximationUsed),
        "WarningDegenerateResult" => Ok(WarningDegenerateResult),
        "WarningLowQuality" => Ok(WarningLowQuality),
        "WarningIncompleteOperation" => Ok(WarningIncompleteOperation),
        "WarningDeprecated" => Ok(WarningDeprecated),
        "InfoOperationStarted" => Ok(InfoOperationStarted),
        "InfoOperationCompleted" => Ok(InfoOperationCompleted),
        "InfoOperationCancelled" => Ok(InfoOperationCancelled),
        "InfoProgress" => Ok(InfoProgress),
        "InfoReady" => Ok(InfoReady),
        "InfoLoading" => Ok(InfoLoading),
        "InfoSaving" => Ok(InfoSaving),
        "InfoExporting" => Ok(InfoExporting),
        "InfoImporting" => Ok(InfoImporting),
        "LabelFile" => Ok(LabelFile),
        "LabelEdit" => Ok(LabelEdit),
        "LabelView" => Ok(LabelView),
        "LabelTools" => Ok(LabelTools),
        "LabelHelp" => Ok(LabelHelp),
        "LabelNew" => Ok(LabelNew),
        "LabelOpen" => Ok(LabelOpen),
        "LabelSave" => Ok(LabelSave),
        "LabelSaveAs" => Ok(LabelSaveAs),
        "LabelExport" => Ok(LabelExport),
        "LabelImport" => Ok(LabelImport),
        "LabelClose" => Ok(LabelClose),
        "LabelExit" => Ok(LabelExit),
        "LabelUndo" => Ok(LabelUndo),
        "LabelRedo" => Ok(LabelRedo),
        "LabelCut" => Ok(LabelCut),
        "LabelCopy" => Ok(LabelCopy),
        "LabelPaste" => Ok(LabelPaste),
        "LabelDelete" => Ok(LabelDelete),
        "LabelSelectAll" => Ok(LabelSelectAll),
        "LabelPreferences" => Ok(LabelPreferences),
        "LabelAbout" => Ok(LabelAbout),
        "ShapeVertex" => Ok(ShapeVertex),
        "ShapeEdge" => Ok(ShapeEdge),
        "ShapeWire" => Ok(ShapeWire),
        "ShapeFace" => Ok(ShapeFace),
        "ShapeShell" => Ok(ShapeShell),
        "ShapeSolid" => Ok(ShapeSolid),
        "ShapeCompound" => Ok(ShapeCompound),
        "ShapeCompSolid" => Ok(ShapeCompSolid),
        "GeomPoint" => Ok(GeomPoint),
        "GeomLine" => Ok(GeomLine),
        "GeomCircle" => Ok(GeomCircle),
        "GeomEllipse" => Ok(GeomEllipse),
        "GeomBezier" => Ok(GeomBezier),
        "GeomBSpline" => Ok(GeomBSpline),
        "GeomNurbs" => Ok(GeomNurbs),
        "GeomPlane" => Ok(GeomPlane),
        "GeomCylinder" => Ok(GeomCylinder),
        "GeomSphere" => Ok(GeomSphere),
        "GeomCone" => Ok(GeomCone),
        "GeomTorus" => Ok(GeomTorus),
        "OpBooleanFuse" => Ok(OpBooleanFuse),
        "OpBooleanCut" => Ok(OpBooleanCut),
        "OpBooleanCommon" => Ok(OpBooleanCommon),
        "OpBooleanSection" => Ok(OpBooleanSection),
        "OpFillet" => Ok(OpFillet),
        "OpChamfer" => Ok(OpChamfer),
        "OpOffset" => Ok(OpOffset),
        "OpDraft" => Ok(OpDraft),
        "OpShell" => Ok(OpShell),
        "OpThickness" => Ok(OpThickness),
        "OpMirror" => Ok(OpMirror),
        "OpRotate" => Ok(OpRotate),
        "OpTranslate" => Ok(OpTranslate),
        "OpScale" => Ok(OpScale),
        "UnitMillimeter" => Ok(UnitMillimeter),
        "UnitCentimeter" => Ok(UnitCentimeter),
        "UnitMeter" => Ok(UnitMeter),
        "UnitInch" => Ok(UnitInch),
        "UnitFoot" => Ok(UnitFoot),
        "UnitDegree" => Ok(UnitDegree),
        "UnitRadian" => Ok(UnitRadian),
        _ => Err(()),
    }
}

/// PHP module information
#[php_module]
pub fn module(module: ModuleBuilder) -> PhpResult<ModuleBuilder> {
    module
        .function(i18n_init)
        .function(i18n_set_language)
        .function(i18n_current_language)
        .function(i18n_translate)
        .function(i18n_available_languages)
}
