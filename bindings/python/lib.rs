//! Python bindings for BrepRs
//!
//! This module provides Python bindings for BrepRs using PyO3.

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use breprs::i18n::{I18n, Language, MessageKey};
use breprs::geometry::Point;
use breprs::topology::TopoDS_Shape;

/// Python wrapper for BrepRs internationalization
#[pyclass(name = "I18n")]
pub struct PyI18n;

#[pymethods]
impl PyI18n {
    /// Initialize i18n with automatic system language detection
    #[staticmethod]
    fn init() {
        I18n::init();
    }

    /// Set the current language
    #[staticmethod]
    fn set_language(lang_code: &str) -> PyResult<bool> {
        if let Some(lang) = Language::from_code(lang_code) {
            I18n::set_language(lang);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Get the current language code
    #[staticmethod]
    fn current_language() -> String {
        I18n::current_language().code().to_string()
    }

    /// Translate a message key
    #[staticmethod]
    fn translate(key: &str) -> String {
        if let Ok(msg_key) = parse_message_key(key) {
            I18n::tr(msg_key).to_string()
        } else {
            format!("Unknown key: {}", key)
        }
    }

    /// Get all available languages
    #[staticmethod]
    fn available_languages() -> Vec<String> {
        Language::all()
            .iter()
            .map(|l| l.code().to_string())
            .collect()
    }
}

/// Python wrapper for Point
#[pyclass(name = "Point")]
#[derive(Clone)]
pub struct PyPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[pymethods]
impl PyPoint {
    #[new]
    fn new(x: f64, y: f64, z: f64) -> Self {
        PyPoint { x, y, z }
    }

    fn __repr__(&self) -> String {
        format!("Point({}, {}, {})", self.x, self.y, self.z)
    }

    fn __str__(&self) -> String {
        self.__repr__()
    }
}

impl From<Point> for PyPoint {
    fn from(point: Point) -> Self {
        PyPoint {
            x: point.x,
            y: point.y,
            z: point.z,
        }
    }
}

impl From<PyPoint> for Point {
    fn from(py_point: PyPoint) -> Self {
        Point::new(py_point.x, py_point.y, py_point.z)
    }
}

/// Python wrapper for Shape
#[pyclass(name = "Shape")]
pub struct PyShape {
    shape: TopoDS_Shape,
}

#[pymethods]
impl PyShape {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(PyShape {
            shape: TopoDS_Shape::new(),
        })
    }

    /// Get the shape type
    fn shape_type(&self) -> String {
        format!("{:?}", self.shape.shape_type())
    }

    /// Check if shape is null
    fn is_null(&self) -> bool {
        self.shape.is_null()
    }

    fn __repr__(&self) -> String {
        format!("Shape(type={})", self.shape_type())
    }
}

/// Parse message key from string
fn parse_message_key(s: &str) -> PyResult<MessageKey> {
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
        _ => Err(PyErr::new::<pyo3::exceptions::PyValueError>(format!("Unknown message key: {}", s))),
    }
}

/// BrepRs Python module
#[pymodule]
fn breprs_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyI18n>()?;
    m.add_class::<PyPoint>()?;
    m.add_class::<PyShape>()?;
    Ok(())
}
