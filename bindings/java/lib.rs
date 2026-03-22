//! Java bindings for BrepRs using JNI
//!
//! This module provides Java Native Interface (JNI) bindings for BrepRs.

use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString, JValue};
use jni::sys::{jdouble, jint, jlong, jstring};

use breprs::i18n::{I18n, Language, MessageKey};

/// Initialize i18n with automatic system language detection
#[no_mangle]
pub extern "system" fn Java_com_breprs_I18n_init(_env: JNIEnv, _class: JClass) {
    I18n::init();
}

/// Set the current language
///
/// # Safety
///
/// The `lang_code` must be a valid Java string.
#[no_mangle]
pub unsafe extern "system" fn Java_com_breprs_I18n_setLanguage(
    env: JNIEnv,
    _class: JClass,
    lang_code: JString,
) -> jint {
    let lang_code: String = match env.get_string(&lang_code) {
        Ok(s) => s.into(),
        Err(_) => return -1,
    };

    if let Some(lang) = Language::from_code(&lang_code) {
        I18n::set_language(lang);
        0
    } else {
        -1
    }
}

/// Get the current language code
///
/// # Safety
///
/// Returns a Java string that must be released by the JVM.
#[no_mangle]
pub unsafe extern "system" fn Java_com_breprs_I18n_currentLanguage(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    let lang_code = I18n::current_language().code().to_string();
    match env.new_string(lang_code) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Translate a message key
///
/// # Safety
///
/// The `key` must be a valid Java string.
/// Returns a Java string that must be released by the JVM.
#[no_mangle]
pub unsafe extern "system" fn Java_com_breprs_I18n_translate(
    env: JNIEnv,
    _class: JClass,
    key: JString,
) -> jstring {
    let key: String = match env.get_string(&key) {
        Ok(s) => s.into(),
        Err(_) => return std::ptr::null_mut(),
    };

    let msg_key = match parse_message_key(&key) {
        Ok(k) => k,
        Err(_) => return std::ptr::null_mut(),
    };

    let translation = I18n::tr(msg_key).to_string();
    match env.new_string(translation) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
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
