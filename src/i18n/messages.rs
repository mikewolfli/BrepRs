//! Message keys and translations for internationalization
//!
//! This module defines all translatable message keys and their translations
//! in all supported languages.

use std::collections::HashMap;
use std::sync::OnceLock;

use super::Language;

/// Message keys for all translatable strings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MessageKey {
    // =========================================================================
    // Error Messages
    // =========================================================================
    
    // General Errors
    ErrorUnknown,
    ErrorInvalidInput,
    ErrorInvalidShape,
    ErrorNullPointer,
    ErrorIndexOutOfRange,
    ErrorOperationFailed,
    ErrorNotImplemented,
    ErrorInternalError,
    
    // File I/O Errors
    ErrorFileNotFound,
    ErrorFileRead,
    ErrorFileWrite,
    ErrorFileFormat,
    ErrorFileCorrupted,
    ErrorFilePermission,
    
    // Geometry Errors
    ErrorInvalidPoint,
    ErrorInvalidVector,
    ErrorInvalidCurve,
    ErrorInvalidSurface,
    ErrorInvalidEdge,
    ErrorInvalidFace,
    ErrorInvalidWire,
    ErrorInvalidShell,
    ErrorInvalidSolid,
    ErrorDegenerateGeometry,
    ErrorSelfIntersection,
    ErrorNonManifold,
    
    // Topology Errors
    ErrorTopologyInvalid,
    ErrorTopologyBroken,
    ErrorTopologyOrientation,
    ErrorTopologyConnectivity,
    
    // Boolean Operation Errors
    ErrorBooleanFuse,
    ErrorBooleanCut,
    ErrorBooleanCommon,
    ErrorBooleanSection,
    ErrorBooleanOverlap,
    
    // Mesh Errors
    ErrorMeshGeneration,
    ErrorMeshInvalid,
    ErrorMeshDegenerate,
    ErrorMeshSelfIntersection,
    
    // Feature Errors
    ErrorFilletFailed,
    ErrorChamferFailed,
    ErrorOffsetFailed,
    ErrorDraftFailed,
    ErrorShellFailed,
    ErrorThicknessFailed,
    
    // Constraint Errors
    ErrorConstraintUnsatisfied,
    ErrorConstraintOverConstrained,
    ErrorConstraintUnderConstrained,
    ErrorConstraintConflict,
    
    // =========================================================================
    // Warning Messages
    // =========================================================================
    WarningPrecisionLoss,
    WarningApproximationUsed,
    WarningDegenerateResult,
    WarningLowQuality,
    WarningIncompleteOperation,
    WarningDeprecated,
    
    // =========================================================================
    // Info Messages
    // =========================================================================
    InfoOperationStarted,
    InfoOperationCompleted,
    InfoOperationCancelled,
    InfoProgress,
    InfoReady,
    InfoLoading,
    InfoSaving,
    InfoExporting,
    InfoImporting,
    
    // =========================================================================
    // UI Labels
    // =========================================================================
    LabelFile,
    LabelEdit,
    LabelView,
    LabelTools,
    LabelHelp,
    LabelNew,
    LabelOpen,
    LabelSave,
    LabelSaveAs,
    LabelExport,
    LabelImport,
    LabelClose,
    LabelExit,
    LabelUndo,
    LabelRedo,
    LabelCut,
    LabelCopy,
    LabelPaste,
    LabelDelete,
    LabelSelectAll,
    LabelPreferences,
    LabelAbout,
    
    // =========================================================================
    // Shape Types
    // =========================================================================
    ShapeVertex,
    ShapeEdge,
    ShapeWire,
    ShapeFace,
    ShapeShell,
    ShapeSolid,
    ShapeCompound,
    ShapeCompSolid,
    
    // =========================================================================
    // Geometry Types
    // =========================================================================
    GeomPoint,
    GeomLine,
    GeomCircle,
    GeomEllipse,
    GeomBezier,
    GeomBSpline,
    GeomNurbs,
    GeomPlane,
    GeomCylinder,
    GeomSphere,
    GeomCone,
    GeomTorus,
    
    // =========================================================================
    // Operations
    // =========================================================================
    OpBooleanFuse,
    OpBooleanCut,
    OpBooleanCommon,
    OpBooleanSection,
    OpFillet,
    OpChamfer,
    OpOffset,
    OpDraft,
    OpShell,
    OpThickness,
    OpMirror,
    OpRotate,
    OpTranslate,
    OpScale,
    
    // =========================================================================
    // Units
    // =========================================================================
    UnitMillimeter,
    UnitCentimeter,
    UnitMeter,
    UnitInch,
    UnitFoot,
    UnitDegree,
    UnitRadian,
}

/// Translation storage for all languages
static TRANSLATIONS: OnceLock<HashMap<Language, HashMap<MessageKey, &'static str>>> = OnceLock::new();

/// Get the translation map for a specific language
pub fn get_translations(lang: Language) -> &'static HashMap<MessageKey, &'static str> {
    TRANSLATIONS.get_or_init(|| {
        let mut all_translations = HashMap::new();
        
        // English translations (default)
        all_translations.insert(Language::English, english_translations());
        
        // Simplified Chinese translations
        all_translations.insert(Language::SimplifiedChinese, simplified_chinese_translations());
        
        // Traditional Chinese translations
        all_translations.insert(Language::TraditionalChinese, traditional_chinese_translations());
        
        // French translations
        all_translations.insert(Language::French, french_translations());
        
        // German translations
        all_translations.insert(Language::German, german_translations());
        
        // Russian translations
        all_translations.insert(Language::Russian, russian_translations());
        
        all_translations
    }).get(&lang).unwrap()
}

/// English translations
fn english_translations() -> HashMap<MessageKey, &'static str> {
    use MessageKey::*;
    
    let mut m = HashMap::new();
    
    // General Errors
    m.insert(ErrorUnknown, "Unknown error");
    m.insert(ErrorInvalidInput, "Invalid input");
    m.insert(ErrorInvalidShape, "Invalid shape");
    m.insert(ErrorNullPointer, "Null pointer");
    m.insert(ErrorIndexOutOfRange, "Index out of range");
    m.insert(ErrorOperationFailed, "Operation failed");
    m.insert(ErrorNotImplemented, "Not implemented");
    m.insert(ErrorInternalError, "Internal error");
    
    // File I/O Errors
    m.insert(ErrorFileNotFound, "File not found: {}");
    m.insert(ErrorFileRead, "Failed to read file: {}");
    m.insert(ErrorFileWrite, "Failed to write file: {}");
    m.insert(ErrorFileFormat, "Unsupported file format: {}");
    m.insert(ErrorFileCorrupted, "File is corrupted: {}");
    m.insert(ErrorFilePermission, "Permission denied: {}");
    
    // Geometry Errors
    m.insert(ErrorInvalidPoint, "Invalid point");
    m.insert(ErrorInvalidVector, "Invalid vector");
    m.insert(ErrorInvalidCurve, "Invalid curve");
    m.insert(ErrorInvalidSurface, "Invalid surface");
    m.insert(ErrorInvalidEdge, "Invalid edge");
    m.insert(ErrorInvalidFace, "Invalid face");
    m.insert(ErrorInvalidWire, "Invalid wire");
    m.insert(ErrorInvalidShell, "Invalid shell");
    m.insert(ErrorInvalidSolid, "Invalid solid");
    m.insert(ErrorDegenerateGeometry, "Degenerate geometry");
    m.insert(ErrorSelfIntersection, "Self-intersection detected");
    m.insert(ErrorNonManifold, "Non-manifold geometry");
    
    // Topology Errors
    m.insert(ErrorTopologyInvalid, "Invalid topology");
    m.insert(ErrorTopologyBroken, "Broken topology");
    m.insert(ErrorTopologyOrientation, "Invalid topology orientation");
    m.insert(ErrorTopologyConnectivity, "Invalid topology connectivity");
    
    // Boolean Operation Errors
    m.insert(ErrorBooleanFuse, "Boolean fuse operation failed");
    m.insert(ErrorBooleanCut, "Boolean cut operation failed");
    m.insert(ErrorBooleanCommon, "Boolean common operation failed");
    m.insert(ErrorBooleanSection, "Boolean section operation failed");
    m.insert(ErrorBooleanOverlap, "Shapes do not overlap");
    
    // Mesh Errors
    m.insert(ErrorMeshGeneration, "Mesh generation failed");
    m.insert(ErrorMeshInvalid, "Invalid mesh");
    m.insert(ErrorMeshDegenerate, "Degenerate mesh");
    m.insert(ErrorMeshSelfIntersection, "Mesh self-intersection detected");
    
    // Feature Errors
    m.insert(ErrorFilletFailed, "Fillet operation failed");
    m.insert(ErrorChamferFailed, "Chamfer operation failed");
    m.insert(ErrorOffsetFailed, "Offset operation failed");
    m.insert(ErrorDraftFailed, "Draft operation failed");
    m.insert(ErrorShellFailed, "Shell operation failed");
    m.insert(ErrorThicknessFailed, "Thickness operation failed");
    
    // Constraint Errors
    m.insert(ErrorConstraintUnsatisfied, "Constraint not satisfied");
    m.insert(ErrorConstraintOverConstrained, "Over-constrained");
    m.insert(ErrorConstraintUnderConstrained, "Under-constrained");
    m.insert(ErrorConstraintConflict, "Constraint conflict");
    
    // Warnings
    m.insert(WarningPrecisionLoss, "Precision loss detected");
    m.insert(WarningApproximationUsed, "Approximation used");
    m.insert(WarningDegenerateResult, "Degenerate result");
    m.insert(WarningLowQuality, "Low quality result");
    m.insert(WarningIncompleteOperation, "Incomplete operation");
    m.insert(WarningDeprecated, "Deprecated: {}");
    
    // Info
    m.insert(InfoOperationStarted, "Operation started");
    m.insert(InfoOperationCompleted, "Operation completed");
    m.insert(InfoOperationCancelled, "Operation cancelled");
    m.insert(InfoProgress, "Progress: {}%");
    m.insert(InfoReady, "Ready");
    m.insert(InfoLoading, "Loading: {}");
    m.insert(InfoSaving, "Saving: {}");
    m.insert(InfoExporting, "Exporting: {}");
    m.insert(InfoImporting, "Importing: {}");
    
    // UI Labels
    m.insert(LabelFile, "File");
    m.insert(LabelEdit, "Edit");
    m.insert(LabelView, "View");
    m.insert(LabelTools, "Tools");
    m.insert(LabelHelp, "Help");
    m.insert(LabelNew, "New");
    m.insert(LabelOpen, "Open");
    m.insert(LabelSave, "Save");
    m.insert(LabelSaveAs, "Save As");
    m.insert(LabelExport, "Export");
    m.insert(LabelImport, "Import");
    m.insert(LabelClose, "Close");
    m.insert(LabelExit, "Exit");
    m.insert(LabelUndo, "Undo");
    m.insert(LabelRedo, "Redo");
    m.insert(LabelCut, "Cut");
    m.insert(LabelCopy, "Copy");
    m.insert(LabelPaste, "Paste");
    m.insert(LabelDelete, "Delete");
    m.insert(LabelSelectAll, "Select All");
    m.insert(LabelPreferences, "Preferences");
    m.insert(LabelAbout, "About");
    
    // Shape Types
    m.insert(ShapeVertex, "Vertex");
    m.insert(ShapeEdge, "Edge");
    m.insert(ShapeWire, "Wire");
    m.insert(ShapeFace, "Face");
    m.insert(ShapeShell, "Shell");
    m.insert(ShapeSolid, "Solid");
    m.insert(ShapeCompound, "Compound");
    m.insert(ShapeCompSolid, "CompSolid");
    
    // Geometry Types
    m.insert(GeomPoint, "Point");
    m.insert(GeomLine, "Line");
    m.insert(GeomCircle, "Circle");
    m.insert(GeomEllipse, "Ellipse");
    m.insert(GeomBezier, "Bezier");
    m.insert(GeomBSpline, "B-Spline");
    m.insert(GeomNurbs, "NURBS");
    m.insert(GeomPlane, "Plane");
    m.insert(GeomCylinder, "Cylinder");
    m.insert(GeomSphere, "Sphere");
    m.insert(GeomCone, "Cone");
    m.insert(GeomTorus, "Torus");
    
    // Operations
    m.insert(OpBooleanFuse, "Fuse");
    m.insert(OpBooleanCut, "Cut");
    m.insert(OpBooleanCommon, "Common");
    m.insert(OpBooleanSection, "Section");
    m.insert(OpFillet, "Fillet");
    m.insert(OpChamfer, "Chamfer");
    m.insert(OpOffset, "Offset");
    m.insert(OpDraft, "Draft");
    m.insert(OpShell, "Shell");
    m.insert(OpThickness, "Thickness");
    m.insert(OpMirror, "Mirror");
    m.insert(OpRotate, "Rotate");
    m.insert(OpTranslate, "Translate");
    m.insert(OpScale, "Scale");
    
    // Units
    m.insert(UnitMillimeter, "Millimeter");
    m.insert(UnitCentimeter, "Centimeter");
    m.insert(UnitMeter, "Meter");
    m.insert(UnitInch, "Inch");
    m.insert(UnitFoot, "Foot");
    m.insert(UnitDegree, "Degree");
    m.insert(UnitRadian, "Radian");
    
    m
}

/// Simplified Chinese translations (简体中文)
fn simplified_chinese_translations() -> HashMap<MessageKey, &'static str> {
    use MessageKey::*;
    
    let mut m = HashMap::new();
    
    // General Errors
    m.insert(ErrorUnknown, "未知错误");
    m.insert(ErrorInvalidInput, "无效的输入");
    m.insert(ErrorInvalidShape, "无效的形状");
    m.insert(ErrorNullPointer, "空指针");
    m.insert(ErrorIndexOutOfRange, "索引超出范围");
    m.insert(ErrorOperationFailed, "操作失败");
    m.insert(ErrorNotImplemented, "未实现");
    m.insert(ErrorInternalError, "内部错误");
    
    // File I/O Errors
    m.insert(ErrorFileNotFound, "文件未找到: {}");
    m.insert(ErrorFileRead, "读取文件失败: {}");
    m.insert(ErrorFileWrite, "写入文件失败: {}");
    m.insert(ErrorFileFormat, "不支持的文件格式: {}");
    m.insert(ErrorFileCorrupted, "文件已损坏: {}");
    m.insert(ErrorFilePermission, "权限被拒绝: {}");
    
    // Geometry Errors
    m.insert(ErrorInvalidPoint, "无效的点");
    m.insert(ErrorInvalidVector, "无效的向量");
    m.insert(ErrorInvalidCurve, "无效的曲线");
    m.insert(ErrorInvalidSurface, "无效的曲面");
    m.insert(ErrorInvalidEdge, "无效的边");
    m.insert(ErrorInvalidFace, "无效的面");
    m.insert(ErrorInvalidWire, "无效的线框");
    m.insert(ErrorInvalidShell, "无效的壳");
    m.insert(ErrorInvalidSolid, "无效的实体");
    m.insert(ErrorDegenerateGeometry, "退化几何体");
    m.insert(ErrorSelfIntersection, "检测到自相交");
    m.insert(ErrorNonManifold, "非流形几何体");
    
    // Topology Errors
    m.insert(ErrorTopologyInvalid, "无效的拓扑");
    m.insert(ErrorTopologyBroken, "拓扑断裂");
    m.insert(ErrorTopologyOrientation, "无效的拓扑方向");
    m.insert(ErrorTopologyConnectivity, "无效的拓扑连接");
    
    // Boolean Operation Errors
    m.insert(ErrorBooleanFuse, "布尔并集操作失败");
    m.insert(ErrorBooleanCut, "布尔差集操作失败");
    m.insert(ErrorBooleanCommon, "布尔交集操作失败");
    m.insert(ErrorBooleanSection, "布尔截面操作失败");
    m.insert(ErrorBooleanOverlap, "形状不重叠");
    
    // Mesh Errors
    m.insert(ErrorMeshGeneration, "网格生成失败");
    m.insert(ErrorMeshInvalid, "无效的网格");
    m.insert(ErrorMeshDegenerate, "退化网格");
    m.insert(ErrorMeshSelfIntersection, "检测到网格自相交");
    
    // Feature Errors
    m.insert(ErrorFilletFailed, "圆角操作失败");
    m.insert(ErrorChamferFailed, "倒角操作失败");
    m.insert(ErrorOffsetFailed, "偏移操作失败");
    m.insert(ErrorDraftFailed, "拔模操作失败");
    m.insert(ErrorShellFailed, "抽壳操作失败");
    m.insert(ErrorThicknessFailed, "厚度操作失败");
    
    // Constraint Errors
    m.insert(ErrorConstraintUnsatisfied, "约束未满足");
    m.insert(ErrorConstraintOverConstrained, "过度约束");
    m.insert(ErrorConstraintUnderConstrained, "约束不足");
    m.insert(ErrorConstraintConflict, "约束冲突");
    
    // Warnings
    m.insert(WarningPrecisionLoss, "检测到精度损失");
    m.insert(WarningApproximationUsed, "使用了近似值");
    m.insert(WarningDegenerateResult, "退化结果");
    m.insert(WarningLowQuality, "低质量结果");
    m.insert(WarningIncompleteOperation, "操作未完成");
    m.insert(WarningDeprecated, "已弃用: {}");
    
    // Info
    m.insert(InfoOperationStarted, "操作已开始");
    m.insert(InfoOperationCompleted, "操作已完成");
    m.insert(InfoOperationCancelled, "操作已取消");
    m.insert(InfoProgress, "进度: {}%");
    m.insert(InfoReady, "就绪");
    m.insert(InfoLoading, "加载中: {}");
    m.insert(InfoSaving, "保存中: {}");
    m.insert(InfoExporting, "导出中: {}");
    m.insert(InfoImporting, "导入中: {}");
    
    // UI Labels
    m.insert(LabelFile, "文件");
    m.insert(LabelEdit, "编辑");
    m.insert(LabelView, "视图");
    m.insert(LabelTools, "工具");
    m.insert(LabelHelp, "帮助");
    m.insert(LabelNew, "新建");
    m.insert(LabelOpen, "打开");
    m.insert(LabelSave, "保存");
    m.insert(LabelSaveAs, "另存为");
    m.insert(LabelExport, "导出");
    m.insert(LabelImport, "导入");
    m.insert(LabelClose, "关闭");
    m.insert(LabelExit, "退出");
    m.insert(LabelUndo, "撤销");
    m.insert(LabelRedo, "重做");
    m.insert(LabelCut, "剪切");
    m.insert(LabelCopy, "复制");
    m.insert(LabelPaste, "粘贴");
    m.insert(LabelDelete, "删除");
    m.insert(LabelSelectAll, "全选");
    m.insert(LabelPreferences, "首选项");
    m.insert(LabelAbout, "关于");
    
    // Shape Types
    m.insert(ShapeVertex, "顶点");
    m.insert(ShapeEdge, "边");
    m.insert(ShapeWire, "线框");
    m.insert(ShapeFace, "面");
    m.insert(ShapeShell, "壳");
    m.insert(ShapeSolid, "实体");
    m.insert(ShapeCompound, "复合体");
    m.insert(ShapeCompSolid, "复合实体");
    
    // Geometry Types
    m.insert(GeomPoint, "点");
    m.insert(GeomLine, "直线");
    m.insert(GeomCircle, "圆");
    m.insert(GeomEllipse, "椭圆");
    m.insert(GeomBezier, "贝塞尔");
    m.insert(GeomBSpline, "B样条");
    m.insert(GeomNurbs, "NURBS");
    m.insert(GeomPlane, "平面");
    m.insert(GeomCylinder, "圆柱");
    m.insert(GeomSphere, "球");
    m.insert(GeomCone, "圆锥");
    m.insert(GeomTorus, "圆环");
    
    // Operations
    m.insert(OpBooleanFuse, "并集");
    m.insert(OpBooleanCut, "差集");
    m.insert(OpBooleanCommon, "交集");
    m.insert(OpBooleanSection, "截面");
    m.insert(OpFillet, "圆角");
    m.insert(OpChamfer, "倒角");
    m.insert(OpOffset, "偏移");
    m.insert(OpDraft, "拔模");
    m.insert(OpShell, "抽壳");
    m.insert(OpThickness, "厚度");
    m.insert(OpMirror, "镜像");
    m.insert(OpRotate, "旋转");
    m.insert(OpTranslate, "平移");
    m.insert(OpScale, "缩放");
    
    // Units
    m.insert(UnitMillimeter, "毫米");
    m.insert(UnitCentimeter, "厘米");
    m.insert(UnitMeter, "米");
    m.insert(UnitInch, "英寸");
    m.insert(UnitFoot, "英尺");
    m.insert(UnitDegree, "度");
    m.insert(UnitRadian, "弧度");
    
    m
}

/// Traditional Chinese translations (繁體中文)
fn traditional_chinese_translations() -> HashMap<MessageKey, &'static str> {
    use MessageKey::*;
    
    let mut m = HashMap::new();
    
    // General Errors
    m.insert(ErrorUnknown, "未知錯誤");
    m.insert(ErrorInvalidInput, "無效的輸入");
    m.insert(ErrorInvalidShape, "無效的形狀");
    m.insert(ErrorNullPointer, "空指標");
    m.insert(ErrorIndexOutOfRange, "索引超出範圍");
    m.insert(ErrorOperationFailed, "操作失敗");
    m.insert(ErrorNotImplemented, "未實作");
    m.insert(ErrorInternalError, "內部錯誤");
    
    // File I/O Errors
    m.insert(ErrorFileNotFound, "檔案未找到: {}");
    m.insert(ErrorFileRead, "讀取檔案失敗: {}");
    m.insert(ErrorFileWrite, "寫入檔案失敗: {}");
    m.insert(ErrorFileFormat, "不支援的檔案格式: {}");
    m.insert(ErrorFileCorrupted, "檔案已損壞: {}");
    m.insert(ErrorFilePermission, "權限被拒絕: {}");
    
    // Geometry Errors
    m.insert(ErrorInvalidPoint, "無效的點");
    m.insert(ErrorInvalidVector, "無效的向量");
    m.insert(ErrorInvalidCurve, "無效的曲線");
    m.insert(ErrorInvalidSurface, "無效的曲面");
    m.insert(ErrorInvalidEdge, "無效的邊");
    m.insert(ErrorInvalidFace, "無效的面");
    m.insert(ErrorInvalidWire, "無效的線框");
    m.insert(ErrorInvalidShell, "無效的殼");
    m.insert(ErrorInvalidSolid, "無效的實體");
    m.insert(ErrorDegenerateGeometry, "退化幾何體");
    m.insert(ErrorSelfIntersection, "偵測到自相交");
    m.insert(ErrorNonManifold, "非流形幾何體");
    
    // Topology Errors
    m.insert(ErrorTopologyInvalid, "無效的拓撲");
    m.insert(ErrorTopologyBroken, "拓撲斷裂");
    m.insert(ErrorTopologyOrientation, "無效的拓撲方向");
    m.insert(ErrorTopologyConnectivity, "無效的拓撲連接");
    
    // Boolean Operation Errors
    m.insert(ErrorBooleanFuse, "布林聯集操作失敗");
    m.insert(ErrorBooleanCut, "布林差集操作失敗");
    m.insert(ErrorBooleanCommon, "布林交集操作失敗");
    m.insert(ErrorBooleanSection, "布林截面操作失敗");
    m.insert(ErrorBooleanOverlap, "形狀不重疊");
    
    // Mesh Errors
    m.insert(ErrorMeshGeneration, "網格生成失敗");
    m.insert(ErrorMeshInvalid, "無效的網格");
    m.insert(ErrorMeshDegenerate, "退化網格");
    m.insert(ErrorMeshSelfIntersection, "偵測到網格自相交");
    
    // Feature Errors
    m.insert(ErrorFilletFailed, "圓角操作失敗");
    m.insert(ErrorChamferFailed, "倒角操作失敗");
    m.insert(ErrorOffsetFailed, "偏移操作失敗");
    m.insert(ErrorDraftFailed, "拔模操作失敗");
    m.insert(ErrorShellFailed, "抽殼操作失敗");
    m.insert(ErrorThicknessFailed, "厚度操作失敗");
    
    // Constraint Errors
    m.insert(ErrorConstraintUnsatisfied, "約束未滿足");
    m.insert(ErrorConstraintOverConstrained, "過度約束");
    m.insert(ErrorConstraintUnderConstrained, "約束不足");
    m.insert(ErrorConstraintConflict, "約束衝突");
    
    // Warnings
    m.insert(WarningPrecisionLoss, "偵測到精度損失");
    m.insert(WarningApproximationUsed, "使用了近似值");
    m.insert(WarningDegenerateResult, "退化結果");
    m.insert(WarningLowQuality, "低品質結果");
    m.insert(WarningIncompleteOperation, "操作未完成");
    m.insert(WarningDeprecated, "已棄用: {}");
    
    // Info
    m.insert(InfoOperationStarted, "操作已開始");
    m.insert(InfoOperationCompleted, "操作已完成");
    m.insert(InfoOperationCancelled, "操作已取消");
    m.insert(InfoProgress, "進度: {}%");
    m.insert(InfoReady, "就緒");
    m.insert(InfoLoading, "載入中: {}");
    m.insert(InfoSaving, "儲存中: {}");
    m.insert(InfoExporting, "匯出中: {}");
    m.insert(InfoImporting, "匯入中: {}");
    
    // UI Labels
    m.insert(LabelFile, "檔案");
    m.insert(LabelEdit, "編輯");
    m.insert(LabelView, "檢視");
    m.insert(LabelTools, "工具");
    m.insert(LabelHelp, "說明");
    m.insert(LabelNew, "新增");
    m.insert(LabelOpen, "開啟");
    m.insert(LabelSave, "儲存");
    m.insert(LabelSaveAs, "另存新檔");
    m.insert(LabelExport, "匯出");
    m.insert(LabelImport, "匯入");
    m.insert(LabelClose, "關閉");
    m.insert(LabelExit, "結束");
    m.insert(LabelUndo, "復原");
    m.insert(LabelRedo, "重做");
    m.insert(LabelCut, "剪下");
    m.insert(LabelCopy, "複製");
    m.insert(LabelPaste, "貼上");
    m.insert(LabelDelete, "刪除");
    m.insert(LabelSelectAll, "全選");
    m.insert(LabelPreferences, "偏好設定");
    m.insert(LabelAbout, "關於");
    
    // Shape Types
    m.insert(ShapeVertex, "頂點");
    m.insert(ShapeEdge, "邊");
    m.insert(ShapeWire, "線框");
    m.insert(ShapeFace, "面");
    m.insert(ShapeShell, "殼");
    m.insert(ShapeSolid, "實體");
    m.insert(ShapeCompound, "複合體");
    m.insert(ShapeCompSolid, "複合實體");
    
    // Geometry Types
    m.insert(GeomPoint, "點");
    m.insert(GeomLine, "直線");
    m.insert(GeomCircle, "圓");
    m.insert(GeomEllipse, "橢圓");
    m.insert(GeomBezier, "貝茲");
    m.insert(GeomBSpline, "B樣條");
    m.insert(GeomNurbs, "NURBS");
    m.insert(GeomPlane, "平面");
    m.insert(GeomCylinder, "圓柱");
    m.insert(GeomSphere, "球");
    m.insert(GeomCone, "圓錐");
    m.insert(GeomTorus, "圓環");
    
    // Operations
    m.insert(OpBooleanFuse, "聯集");
    m.insert(OpBooleanCut, "差集");
    m.insert(OpBooleanCommon, "交集");
    m.insert(OpBooleanSection, "截面");
    m.insert(OpFillet, "圓角");
    m.insert(OpChamfer, "倒角");
    m.insert(OpOffset, "偏移");
    m.insert(OpDraft, "拔模");
    m.insert(OpShell, "抽殼");
    m.insert(OpThickness, "厚度");
    m.insert(OpMirror, "鏡像");
    m.insert(OpRotate, "旋轉");
    m.insert(OpTranslate, "平移");
    m.insert(OpScale, "縮放");
    
    // Units
    m.insert(UnitMillimeter, "公釐");
    m.insert(UnitCentimeter, "公分");
    m.insert(UnitMeter, "公尺");
    m.insert(UnitInch, "英吋");
    m.insert(UnitFoot, "英呎");
    m.insert(UnitDegree, "度");
    m.insert(UnitRadian, "弧度");
    
    m
}

/// French translations (Français)
fn french_translations() -> HashMap<MessageKey, &'static str> {
    use MessageKey::*;
    
    let mut m = HashMap::new();
    
    // General Errors
    m.insert(ErrorUnknown, "Erreur inconnue");
    m.insert(ErrorInvalidInput, "Entrée invalide");
    m.insert(ErrorInvalidShape, "Forme invalide");
    m.insert(ErrorNullPointer, "Pointeur nul");
    m.insert(ErrorIndexOutOfRange, "Index hors limites");
    m.insert(ErrorOperationFailed, "L'opération a échoué");
    m.insert(ErrorNotImplemented, "Non implémenté");
    m.insert(ErrorInternalError, "Erreur interne");
    
    // File I/O Errors
    m.insert(ErrorFileNotFound, "Fichier introuvable: {}");
    m.insert(ErrorFileRead, "Échec de lecture du fichier: {}");
    m.insert(ErrorFileWrite, "Échec d'écriture du fichier: {}");
    m.insert(ErrorFileFormat, "Format de fichier non supporté: {}");
    m.insert(ErrorFileCorrupted, "Fichier corrompu: {}");
    m.insert(ErrorFilePermission, "Permission refusée: {}");
    
    // Geometry Errors
    m.insert(ErrorInvalidPoint, "Point invalide");
    m.insert(ErrorInvalidVector, "Vecteur invalide");
    m.insert(ErrorInvalidCurve, "Courbe invalide");
    m.insert(ErrorInvalidSurface, "Surface invalide");
    m.insert(ErrorInvalidEdge, "Arête invalide");
    m.insert(ErrorInvalidFace, "Face invalide");
    m.insert(ErrorInvalidWire, "Fil invalide");
    m.insert(ErrorInvalidShell, "Coque invalide");
    m.insert(ErrorInvalidSolid, "Solide invalide");
    m.insert(ErrorDegenerateGeometry, "Géométrie dégénérée");
    m.insert(ErrorSelfIntersection, "Auto-intersection détectée");
    m.insert(ErrorNonManifold, "Géométrie non-manifold");
    
    // Topology Errors
    m.insert(ErrorTopologyInvalid, "Topologie invalide");
    m.insert(ErrorTopologyBroken, "Topologie cassée");
    m.insert(ErrorTopologyOrientation, "Orientation de topologie invalide");
    m.insert(ErrorTopologyConnectivity, "Connectivité de topologie invalide");
    
    // Boolean Operation Errors
    m.insert(ErrorBooleanFuse, "L'opération booléenne union a échoué");
    m.insert(ErrorBooleanCut, "L'opération booléenne soustraction a échoué");
    m.insert(ErrorBooleanCommon, "L'opération booléenne intersection a échoué");
    m.insert(ErrorBooleanSection, "L'opération booléenne section a échoué");
    m.insert(ErrorBooleanOverlap, "Les formes ne se chevauchent pas");
    
    // Mesh Errors
    m.insert(ErrorMeshGeneration, "La génération de maillage a échoué");
    m.insert(ErrorMeshInvalid, "Maillage invalide");
    m.insert(ErrorMeshDegenerate, "Maillage dégénéré");
    m.insert(ErrorMeshSelfIntersection, "Auto-intersection de maillage détectée");
    
    // Feature Errors
    m.insert(ErrorFilletFailed, "L'opération de congé a échoué");
    m.insert(ErrorChamferFailed, "L'opération de chanfrein a échoué");
    m.insert(ErrorOffsetFailed, "L'opération de décalage a échoué");
    m.insert(ErrorDraftFailed, "L'opération de dépouille a échoué");
    m.insert(ErrorShellFailed, "L'opération de coque a échoué");
    m.insert(ErrorThicknessFailed, "L'opération d'épaisseur a échoué");
    
    // Constraint Errors
    m.insert(ErrorConstraintUnsatisfied, "Contrainte non satisfaite");
    m.insert(ErrorConstraintOverConstrained, "Sur-contraint");
    m.insert(ErrorConstraintUnderConstrained, "Sous-contraint");
    m.insert(ErrorConstraintConflict, "Conflit de contraintes");
    
    // Warnings
    m.insert(WarningPrecisionLoss, "Perte de précision détectée");
    m.insert(WarningApproximationUsed, "Approximation utilisée");
    m.insert(WarningDegenerateResult, "Résultat dégénéré");
    m.insert(WarningLowQuality, "Résultat de faible qualité");
    m.insert(WarningIncompleteOperation, "Opération incomplète");
    m.insert(WarningDeprecated, "Obsolète: {}");
    
    // Info
    m.insert(InfoOperationStarted, "Opération démarrée");
    m.insert(InfoOperationCompleted, "Opération terminée");
    m.insert(InfoOperationCancelled, "Opération annulée");
    m.insert(InfoProgress, "Progression: {}%");
    m.insert(InfoReady, "Prêt");
    m.insert(InfoLoading, "Chargement: {}");
    m.insert(InfoSaving, "Enregistrement: {}");
    m.insert(InfoExporting, "Exportation: {}");
    m.insert(InfoImporting, "Importation: {}");
    
    // UI Labels
    m.insert(LabelFile, "Fichier");
    m.insert(LabelEdit, "Édition");
    m.insert(LabelView, "Affichage");
    m.insert(LabelTools, "Outils");
    m.insert(LabelHelp, "Aide");
    m.insert(LabelNew, "Nouveau");
    m.insert(LabelOpen, "Ouvrir");
    m.insert(LabelSave, "Enregistrer");
    m.insert(LabelSaveAs, "Enregistrer sous");
    m.insert(LabelExport, "Exporter");
    m.insert(LabelImport, "Importer");
    m.insert(LabelClose, "Fermer");
    m.insert(LabelExit, "Quitter");
    m.insert(LabelUndo, "Annuler");
    m.insert(LabelRedo, "Rétablir");
    m.insert(LabelCut, "Couper");
    m.insert(LabelCopy, "Copier");
    m.insert(LabelPaste, "Coller");
    m.insert(LabelDelete, "Supprimer");
    m.insert(LabelSelectAll, "Tout sélectionner");
    m.insert(LabelPreferences, "Préférences");
    m.insert(LabelAbout, "À propos");
    
    // Shape Types
    m.insert(ShapeVertex, "Sommet");
    m.insert(ShapeEdge, "Arête");
    m.insert(ShapeWire, "Fil");
    m.insert(ShapeFace, "Face");
    m.insert(ShapeShell, "Coque");
    m.insert(ShapeSolid, "Solide");
    m.insert(ShapeCompound, "Composé");
    m.insert(ShapeCompSolid, "Composé de solides");
    
    // Geometry Types
    m.insert(GeomPoint, "Point");
    m.insert(GeomLine, "Ligne");
    m.insert(GeomCircle, "Cercle");
    m.insert(GeomEllipse, "Ellipse");
    m.insert(GeomBezier, "Bézier");
    m.insert(GeomBSpline, "B-Spline");
    m.insert(GeomNurbs, "NURBS");
    m.insert(GeomPlane, "Plan");
    m.insert(GeomCylinder, "Cylindre");
    m.insert(GeomSphere, "Sphère");
    m.insert(GeomCone, "Cône");
    m.insert(GeomTorus, "Tore");
    
    // Operations
    m.insert(OpBooleanFuse, "Union");
    m.insert(OpBooleanCut, "Soustraction");
    m.insert(OpBooleanCommon, "Intersection");
    m.insert(OpBooleanSection, "Section");
    m.insert(OpFillet, "Congé");
    m.insert(OpChamfer, "Chanfrein");
    m.insert(OpOffset, "Décalage");
    m.insert(OpDraft, "Dépouille");
    m.insert(OpShell, "Coque");
    m.insert(OpThickness, "Épaisseur");
    m.insert(OpMirror, "Miroir");
    m.insert(OpRotate, "Rotation");
    m.insert(OpTranslate, "Translation");
    m.insert(OpScale, "Échelle");
    
    // Units
    m.insert(UnitMillimeter, "Millimètre");
    m.insert(UnitCentimeter, "Centimètre");
    m.insert(UnitMeter, "Mètre");
    m.insert(UnitInch, "Pouce");
    m.insert(UnitFoot, "Pied");
    m.insert(UnitDegree, "Degré");
    m.insert(UnitRadian, "Radian");
    
    m
}

/// German translations (Deutsch)
fn german_translations() -> HashMap<MessageKey, &'static str> {
    use MessageKey::*;
    
    let mut m = HashMap::new();
    
    // General Errors
    m.insert(ErrorUnknown, "Unbekannter Fehler");
    m.insert(ErrorInvalidInput, "Ungültige Eingabe");
    m.insert(ErrorInvalidShape, "Ungültige Form");
    m.insert(ErrorNullPointer, "Nullzeiger");
    m.insert(ErrorIndexOutOfRange, "Index außerhalb des Bereichs");
    m.insert(ErrorOperationFailed, "Operation fehlgeschlagen");
    m.insert(ErrorNotImplemented, "Nicht implementiert");
    m.insert(ErrorInternalError, "Interner Fehler");
    
    // File I/O Errors
    m.insert(ErrorFileNotFound, "Datei nicht gefunden: {}");
    m.insert(ErrorFileRead, "Datei konnte nicht gelesen werden: {}");
    m.insert(ErrorFileWrite, "Datei konnte nicht geschrieben werden: {}");
    m.insert(ErrorFileFormat, "Nicht unterstütztes Dateiformat: {}");
    m.insert(ErrorFileCorrupted, "Datei ist beschädigt: {}");
    m.insert(ErrorFilePermission, "Zugriff verweigert: {}");
    
    // Geometry Errors
    m.insert(ErrorInvalidPoint, "Ungültiger Punkt");
    m.insert(ErrorInvalidVector, "Ungültiger Vektor");
    m.insert(ErrorInvalidCurve, "Ungültige Kurve");
    m.insert(ErrorInvalidSurface, "Ungültige Oberfläche");
    m.insert(ErrorInvalidEdge, "Ungültige Kante");
    m.insert(ErrorInvalidFace, "Ungültige Fläche");
    m.insert(ErrorInvalidWire, "Ungültiger Draht");
    m.insert(ErrorInvalidShell, "Ungültige Hülle");
    m.insert(ErrorInvalidSolid, "Ungültiger Festkörper");
    m.insert(ErrorDegenerateGeometry, "Degenerierte Geometrie");
    m.insert(ErrorSelfIntersection, "Selbstüberschneidung erkannt");
    m.insert(ErrorNonManifold, "Nicht-manifold Geometrie");
    
    // Topology Errors
    m.insert(ErrorTopologyInvalid, "Ungültige Topologie");
    m.insert(ErrorTopologyBroken, "Beschädigte Topologie");
    m.insert(ErrorTopologyOrientation, "Ungültige Topologie-Orientierung");
    m.insert(ErrorTopologyConnectivity, "Ungültige Topologie-Konnektivität");
    
    // Boolean Operation Errors
    m.insert(ErrorBooleanFuse, "Boolesche Vereinigung fehlgeschlagen");
    m.insert(ErrorBooleanCut, "Boolesche Differenz fehlgeschlagen");
    m.insert(ErrorBooleanCommon, "Boolesche Schnittmenge fehlgeschlagen");
    m.insert(ErrorBooleanSection, "Boolescher Schnitt fehlgeschlagen");
    m.insert(ErrorBooleanOverlap, "Formen überlappen sich nicht");
    
    // Mesh Errors
    m.insert(ErrorMeshGeneration, "Mesh-Generierung fehlgeschlagen");
    m.insert(ErrorMeshInvalid, "Ungültiges Mesh");
    m.insert(ErrorMeshDegenerate, "Degeneriertes Mesh");
    m.insert(ErrorMeshSelfIntersection, "Mesh-Selbstüberschneidung erkannt");
    
    // Feature Errors
    m.insert(ErrorFilletFailed, "Verrundung fehlgeschlagen");
    m.insert(ErrorChamferFailed, "Fase fehlgeschlagen");
    m.insert(ErrorOffsetFailed, "Versatz fehlgeschlagen");
    m.insert(ErrorDraftFailed, "Schrägstellung fehlgeschlagen");
    m.insert(ErrorShellFailed, "Hüllenoperation fehlgeschlagen");
    m.insert(ErrorThicknessFailed, "Wandstärke fehlgeschlagen");
    
    // Constraint Errors
    m.insert(ErrorConstraintUnsatisfied, "Bedingung nicht erfüllt");
    m.insert(ErrorConstraintOverConstrained, "Überbestimmt");
    m.insert(ErrorConstraintUnderConstrained, "Unterbestimmt");
    m.insert(ErrorConstraintConflict, "Bedingungskonflikt");
    
    // Warnings
    m.insert(WarningPrecisionLoss, "Präzisionsverlust erkannt");
    m.insert(WarningApproximationUsed, "Annäherung verwendet");
    m.insert(WarningDegenerateResult, "Degeneriertes Ergebnis");
    m.insert(WarningLowQuality, "Ergebnis mit niedriger Qualität");
    m.insert(WarningIncompleteOperation, "Unvollständige Operation");
    m.insert(WarningDeprecated, "Veraltet: {}");
    
    // Info
    m.insert(InfoOperationStarted, "Operation gestartet");
    m.insert(InfoOperationCompleted, "Operation abgeschlossen");
    m.insert(InfoOperationCancelled, "Operation abgebrochen");
    m.insert(InfoProgress, "Fortschritt: {}%");
    m.insert(InfoReady, "Bereit");
    m.insert(InfoLoading, "Laden: {}");
    m.insert(InfoSaving, "Speichern: {}");
    m.insert(InfoExporting, "Exportieren: {}");
    m.insert(InfoImporting, "Importieren: {}");
    
    // UI Labels
    m.insert(LabelFile, "Datei");
    m.insert(LabelEdit, "Bearbeiten");
    m.insert(LabelView, "Ansicht");
    m.insert(LabelTools, "Werkzeuge");
    m.insert(LabelHelp, "Hilfe");
    m.insert(LabelNew, "Neu");
    m.insert(LabelOpen, "Öffnen");
    m.insert(LabelSave, "Speichern");
    m.insert(LabelSaveAs, "Speichern unter");
    m.insert(LabelExport, "Exportieren");
    m.insert(LabelImport, "Importieren");
    m.insert(LabelClose, "Schließen");
    m.insert(LabelExit, "Beenden");
    m.insert(LabelUndo, "Rückgängig");
    m.insert(LabelRedo, "Wiederholen");
    m.insert(LabelCut, "Ausschneiden");
    m.insert(LabelCopy, "Kopieren");
    m.insert(LabelPaste, "Einfügen");
    m.insert(LabelDelete, "Löschen");
    m.insert(LabelSelectAll, "Alles auswählen");
    m.insert(LabelPreferences, "Einstellungen");
    m.insert(LabelAbout, "Über");
    
    // Shape Types
    m.insert(ShapeVertex, "Scheitelpunkt");
    m.insert(ShapeEdge, "Kante");
    m.insert(ShapeWire, "Draht");
    m.insert(ShapeFace, "Fläche");
    m.insert(ShapeShell, "Hülle");
    m.insert(ShapeSolid, "Festkörper");
    m.insert(ShapeCompound, "Verbund");
    m.insert(ShapeCompSolid, "Verbundfestkörper");
    
    // Geometry Types
    m.insert(GeomPoint, "Punkt");
    m.insert(GeomLine, "Linie");
    m.insert(GeomCircle, "Kreis");
    m.insert(GeomEllipse, "Ellipse");
    m.insert(GeomBezier, "Bézier");
    m.insert(GeomBSpline, "B-Spline");
    m.insert(GeomNurbs, "NURBS");
    m.insert(GeomPlane, "Ebene");
    m.insert(GeomCylinder, "Zylinder");
    m.insert(GeomSphere, "Kugel");
    m.insert(GeomCone, "Kegel");
    m.insert(GeomTorus, "Torus");
    
    // Operations
    m.insert(OpBooleanFuse, "Vereinigung");
    m.insert(OpBooleanCut, "Differenz");
    m.insert(OpBooleanCommon, "Schnittmenge");
    m.insert(OpBooleanSection, "Schnitt");
    m.insert(OpFillet, "Verrundung");
    m.insert(OpChamfer, "Fase");
    m.insert(OpOffset, "Versatz");
    m.insert(OpDraft, "Schrägstellung");
    m.insert(OpShell, "Hülle");
    m.insert(OpThickness, "Wandstärke");
    m.insert(OpMirror, "Spiegeln");
    m.insert(OpRotate, "Drehen");
    m.insert(OpTranslate, "Verschieben");
    m.insert(OpScale, "Skalieren");
    
    // Units
    m.insert(UnitMillimeter, "Millimeter");
    m.insert(UnitCentimeter, "Zentimeter");
    m.insert(UnitMeter, "Meter");
    m.insert(UnitInch, "Zoll");
    m.insert(UnitFoot, "Fuß");
    m.insert(UnitDegree, "Grad");
    m.insert(UnitRadian, "Radiant");
    
    m
}

/// Russian translations (Русский)
fn russian_translations() -> HashMap<MessageKey, &'static str> {
    use MessageKey::*;
    
    let mut m = HashMap::new();
    
    // General Errors
    m.insert(ErrorUnknown, "Неизвестная ошибка");
    m.insert(ErrorInvalidInput, "Неверный ввод");
    m.insert(ErrorInvalidShape, "Недопустимая форма");
    m.insert(ErrorNullPointer, "Нулевой указатель");
    m.insert(ErrorIndexOutOfRange, "Индекс вне диапазона");
    m.insert(ErrorOperationFailed, "Операция не удалась");
    m.insert(ErrorNotImplemented, "Не реализовано");
    m.insert(ErrorInternalError, "Внутренняя ошибка");
    
    // File I/O Errors
    m.insert(ErrorFileNotFound, "Файл не найден: {}");
    m.insert(ErrorFileRead, "Ошибка чтения файла: {}");
    m.insert(ErrorFileWrite, "Ошибка записи файла: {}");
    m.insert(ErrorFileFormat, "Неподдерживаемый формат файла: {}");
    m.insert(ErrorFileCorrupted, "Файл повреждён: {}");
    m.insert(ErrorFilePermission, "Доступ запрещён: {}");
    
    // Geometry Errors
    m.insert(ErrorInvalidPoint, "Недопустимая точка");
    m.insert(ErrorInvalidVector, "Недопустимый вектор");
    m.insert(ErrorInvalidCurve, "Недопустимая кривая");
    m.insert(ErrorInvalidSurface, "Недопустимая поверхность");
    m.insert(ErrorInvalidEdge, "Недопустимое ребро");
    m.insert(ErrorInvalidFace, "Недопустимая грань");
    m.insert(ErrorInvalidWire, "Недопустимый каркас");
    m.insert(ErrorInvalidShell, "Недопустимая оболочка");
    m.insert(ErrorInvalidSolid, "Недопустимое тело");
    m.insert(ErrorDegenerateGeometry, "Вырожденная геометрия");
    m.insert(ErrorSelfIntersection, "Обнаружено самопересечение");
    m.insert(ErrorNonManifold, "Не-многообразная геометрия");
    
    // Topology Errors
    m.insert(ErrorTopologyInvalid, "Недопустимая топология");
    m.insert(ErrorTopologyBroken, "Нарушенная топология");
    m.insert(ErrorTopologyOrientation, "Недопустимая ориентация топологии");
    m.insert(ErrorTopologyConnectivity, "Недопустимая связность топологии");
    
    // Boolean Operation Errors
    m.insert(ErrorBooleanFuse, "Операция булева объединения не удалась");
    m.insert(ErrorBooleanCut, "Операция булева вычитания не удалась");
    m.insert(ErrorBooleanCommon, "Операция булева пересечения не удалась");
    m.insert(ErrorBooleanSection, "Операция булева сечения не удалась");
    m.insert(ErrorBooleanOverlap, "Формы не перекрываются");
    
    // Mesh Errors
    m.insert(ErrorMeshGeneration, "Ошибка генерации сетки");
    m.insert(ErrorMeshInvalid, "Недопустимая сетка");
    m.insert(ErrorMeshDegenerate, "Вырожденная сетка");
    m.insert(ErrorMeshSelfIntersection, "Обнаружено самопересечение сетки");
    
    // Feature Errors
    m.insert(ErrorFilletFailed, "Операция скругления не удалась");
    m.insert(ErrorChamferFailed, "Операция фаски не удалась");
    m.insert(ErrorOffsetFailed, "Операция смещения не удалась");
    m.insert(ErrorDraftFailed, "Операция уклона не удалась");
    m.insert(ErrorShellFailed, "Операция оболочки не удалась");
    m.insert(ErrorThicknessFailed, "Операция толщины не удалась");
    
    // Constraint Errors
    m.insert(ErrorConstraintUnsatisfied, "Ограничение не удовлетворено");
    m.insert(ErrorConstraintOverConstrained, "Избыточные ограничения");
    m.insert(ErrorConstraintUnderConstrained, "Недостаточно ограничений");
    m.insert(ErrorConstraintConflict, "Конфликт ограничений");
    
    // Warnings
    m.insert(WarningPrecisionLoss, "Обнаружена потеря точности");
    m.insert(WarningApproximationUsed, "Использована аппроксимация");
    m.insert(WarningDegenerateResult, "Вырожденный результат");
    m.insert(WarningLowQuality, "Результат низкого качества");
    m.insert(WarningIncompleteOperation, "Незавершённая операция");
    m.insert(WarningDeprecated, "Устарело: {}");
    
    // Info
    m.insert(InfoOperationStarted, "Операция начата");
    m.insert(InfoOperationCompleted, "Операция завершена");
    m.insert(InfoOperationCancelled, "Операция отменена");
    m.insert(InfoProgress, "Прогресс: {}%");
    m.insert(InfoReady, "Готово");
    m.insert(InfoLoading, "Загрузка: {}");
    m.insert(InfoSaving, "Сохранение: {}");
    m.insert(InfoExporting, "Экспорт: {}");
    m.insert(InfoImporting, "Импорт: {}");
    
    // UI Labels
    m.insert(LabelFile, "Файл");
    m.insert(LabelEdit, "Редактирование");
    m.insert(LabelView, "Вид");
    m.insert(LabelTools, "Инструменты");
    m.insert(LabelHelp, "Справка");
    m.insert(LabelNew, "Новый");
    m.insert(LabelOpen, "Открыть");
    m.insert(LabelSave, "Сохранить");
    m.insert(LabelSaveAs, "Сохранить как");
    m.insert(LabelExport, "Экспорт");
    m.insert(LabelImport, "Импорт");
    m.insert(LabelClose, "Закрыть");
    m.insert(LabelExit, "Выход");
    m.insert(LabelUndo, "Отменить");
    m.insert(LabelRedo, "Повторить");
    m.insert(LabelCut, "Вырезать");
    m.insert(LabelCopy, "Копировать");
    m.insert(LabelPaste, "Вставить");
    m.insert(LabelDelete, "Удалить");
    m.insert(LabelSelectAll, "Выделить всё");
    m.insert(LabelPreferences, "Настройки");
    m.insert(LabelAbout, "О программе");
    
    // Shape Types
    m.insert(ShapeVertex, "Вершина");
    m.insert(ShapeEdge, "Ребро");
    m.insert(ShapeWire, "Каркас");
    m.insert(ShapeFace, "Грань");
    m.insert(ShapeShell, "Оболочка");
    m.insert(ShapeSolid, "Тело");
    m.insert(ShapeCompound, "Композит");
    m.insert(ShapeCompSolid, "Композитное тело");
    
    // Geometry Types
    m.insert(GeomPoint, "Точка");
    m.insert(GeomLine, "Линия");
    m.insert(GeomCircle, "Окружность");
    m.insert(GeomEllipse, "Эллипс");
    m.insert(GeomBezier, "Безье");
    m.insert(GeomBSpline, "B-сплайн");
    m.insert(GeomNurbs, "NURBS");
    m.insert(GeomPlane, "Плоскость");
    m.insert(GeomCylinder, "Цилиндр");
    m.insert(GeomSphere, "Сфера");
    m.insert(GeomCone, "Конус");
    m.insert(GeomTorus, "Тор");
    
    // Operations
    m.insert(OpBooleanFuse, "Объединение");
    m.insert(OpBooleanCut, "Вычитание");
    m.insert(OpBooleanCommon, "Пересечение");
    m.insert(OpBooleanSection, "Сечение");
    m.insert(OpFillet, "Скругление");
    m.insert(OpChamfer, "Фаска");
    m.insert(OpOffset, "Смещение");
    m.insert(OpDraft, "Уклон");
    m.insert(OpShell, "Оболочка");
    m.insert(OpThickness, "Толщина");
    m.insert(OpMirror, "Зеркало");
    m.insert(OpRotate, "Поворот");
    m.insert(OpTranslate, "Перенос");
    m.insert(OpScale, "Масштаб");
    
    // Units
    m.insert(UnitMillimeter, "Миллиметр");
    m.insert(UnitCentimeter, "Сантиметр");
    m.insert(UnitMeter, "Метр");
    m.insert(UnitInch, "Дюйм");
    m.insert(UnitFoot, "Фут");
    m.insert(UnitDegree, "Градус");
    m.insert(UnitRadian, "Радиан");
    
    m
}
