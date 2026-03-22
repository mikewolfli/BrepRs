//! C FFI bindings for BrepRs
//!
//! This module provides C-compatible FFI bindings for BrepRs.
//!
//! # Safety
//!
//! All pointers returned by these functions must be freed using the
//! corresponding free functions to avoid memory leaks.

use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_double, c_int, c_void};
use std::ptr;

use breprs::i18n::{I18n, Language, MessageKey};
use breprs::geometry::Point;
use breprs::topology::TopoDS_Shape;

/// Opaque handle to a BrepRs Point
#[repr(C)]
pub struct BrepPoint {
    x: c_double,
    y: c_double,
    z: c_double,
}

/// Opaque handle to a BrepRs Shape
#[repr(C)]
pub struct BrepShape {
    _private: [u8; 0],
}

/// Result codes for BrepRs operations
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BrepResult {
    Success = 0,
    Error = -1,
    InvalidInput = -2,
    NullPointer = -3,
    OutOfMemory = -4,
}

/// Initialize BrepRs i18n with automatic system language detection
#[no_mangle]
pub extern "C" fn breprs_i18n_init() -> BrepResult {
    I18n::init();
    BrepResult::Success
}

/// Set the current language
///
/// # Safety
///
/// The `lang_code` must be a valid null-terminated C string.
#[no_mangle]
pub unsafe extern "C" fn breprs_i18n_set_language(lang_code: *const c_char) -> BrepResult {
    if lang_code.is_null() {
        return BrepResult::NullPointer;
    }

    let c_str = match CStr::from_ptr(lang_code).to_str() {
        Ok(s) => s,
        Err(_) => return BrepResult::InvalidInput,
    };

    if let Some(lang) = Language::from_code(c_str) {
        I18n::set_language(lang);
        BrepResult::Success
    } else {
        BrepResult::InvalidInput
    }
}

/// Get the current language code
///
/// # Safety
///
/// The returned string must be freed using `breprs_free_string`.
#[no_mangle]
pub unsafe extern "C" fn breprs_i18n_current_language() -> *mut c_char {
    let lang_code = I18n::current_language().code().to_string();
    match CString::new(lang_code) {
        Ok(s) => s.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// Translate a message key
///
/// # Safety
///
/// The `key` must be a valid null-terminated C string.
/// The returned string must be freed using `breprs_free_string`.
#[no_mangle]
pub unsafe extern "C" fn breprs_i18n_translate(key: *const c_char) -> *mut c_char {
    if key.is_null() {
        return ptr::null_mut();
    }

    let c_str = match CStr::from_ptr(key).to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let msg_key = match parse_message_key(c_str) {
        Ok(k) => k,
        Err(_) => return ptr::null_mut(),
    };

    let translation = I18n::tr(msg_key).to_string();
    match CString::new(translation) {
        Ok(s) => s.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// Create a new point
#[no_mangle]
pub extern "C" fn breprs_point_new(x: c_double, y: c_double, z: c_double) -> *mut BrepPoint {
    let point = Box::new(BrepPoint { x, y, z });
    Box::into_raw(point)
}

/// Get point coordinates
///
/// # Safety
///
/// The `point` must be a valid pointer created by `breprs_point_new`.
#[no_mangle]
pub unsafe extern "C" fn breprs_point_get_coords(
    point: *const BrepPoint,
    x: *mut c_double,
    y: *mut c_double,
    z: *mut c_double,
) -> BrepResult {
    if point.is_null() || x.is_null() || y.is_null() || z.is_null() {
        return BrepResult::NullPointer;
    }

    *x = (*point).x;
    *y = (*point).y;
    *z = (*point).z;
    BrepResult::Success
}

/// Free a point
///
/// # Safety
///
/// The `point` must be a valid pointer created by `breprs_point_new`.
#[no_mangle]
pub unsafe extern "C" fn breprs_point_free(point: *mut BrepPoint) {
    if !point.is_null() {
        let _ = Box::from_raw(point);
    }
}

/// Create a new shape
#[no_mangle]
pub extern "C" fn breprs_shape_new() -> *mut BrepShape {
    let shape = Box::new(BrepShape { _private: [] });
    Box::into_raw(shape)
}

/// Check if shape is null
///
/// # Safety
///
/// The `shape` must be a valid pointer created by `breprs_shape_new`.
#[no_mangle]
pub unsafe extern "C" fn breprs_shape_is_null(shape: *const BrepShape) -> c_int {
    if shape.is_null() {
        return 1;
    }
    0
}

/// Free a shape
///
/// # Safety
///
/// The `shape` must be a valid pointer created by `breprs_shape_new`.
#[no_mangle]
pub unsafe extern "C" fn breprs_shape_free(shape: *mut BrepShape) {
    if !shape.is_null() {
        let _ = Box::from_raw(shape);
    }
}

/// Free a string allocated by BrepRs
///
/// # Safety
///
/// The `str` must be a valid pointer returned by a BrepRs function.
#[no_mangle]
pub unsafe extern "C" fn breprs_free_string(str: *mut c_char) {
    if !str.is_null() {
        let _ = CString::from_raw(str);
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
