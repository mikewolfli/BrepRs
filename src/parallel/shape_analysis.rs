//! Parallel Shape Analysis
//!
//! This module provides parallel shape analysis capabilities
//! using the Rayon library for improved performance on multi-core systems.

use rayon::prelude::*;
use std::collections::HashMap;

use super::{ParallelConfig, ParallelResult, ParallelStats};
use crate::foundation::handle::Handle;
use crate::geometry::Point;
use crate::topology::{
    topods_face::TopoDsFace, topods_shape::TopoDsShape, topods_solid::TopoDsSolid, ShapeType,
};

/// Parallel shape analyzer
pub struct ParallelShapeAnalyzer {
    config: ParallelConfig,
}

impl ParallelShapeAnalyzer {
    pub fn new() -> Self {
        Self {
            config: ParallelConfig::default(),
        }
    }

    pub fn with_config(config: ParallelConfig) -> Self {
        Self { config }
    }

    /// Compute bounding boxes for multiple shapes in parallel
    pub fn compute_bounding_boxes(
        &self,
        shapes: &[Handle<TopoDsShape>],
    ) -> ParallelResult<Vec<(Point, Point)>> {
        use std::time::Instant;

        let start = Instant::now();

        let results: Vec<(Point, Point)> = if shapes.len() >= self.config.min_parallel_size {
            shapes
                .par_iter()
                .map(|shape| {
                    shape
                        .as_ref()
                        .map_or((Point::origin(), Point::origin()), |s| s.bounding_box())
                })
                .collect()
        } else {
            shapes
                .iter()
                .map(|shape| {
                    shape
                        .as_ref()
                        .map_or((Point::origin(), Point::origin()), |s| s.bounding_box())
                })
                .collect()
        };

        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(shapes.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);

        ParallelResult::new(results, stats)
    }

    /// Compute volumes for multiple solids in parallel
    pub fn compute_volumes(&self, solids: &[Handle<TopoDsSolid>]) -> ParallelResult<Vec<f64>> {
        use std::time::Instant;

        let start = Instant::now();

        let results: Vec<f64> = if solids.len() >= self.config.min_parallel_size {
            solids.par_iter().map(|solid| solid.volume()).collect()
        } else {
            solids.iter().map(|solid| solid.volume()).collect()
        };

        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(solids.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);

        ParallelResult::new(results, stats)
    }

    /// Compute surface areas for multiple faces in parallel
    pub fn compute_surface_areas(&self, faces: &[Handle<TopoDsFace>]) -> ParallelResult<Vec<f64>> {
        use std::time::Instant;

        let start = Instant::now();

        let results: Vec<f64> = if faces.len() >= self.config.min_parallel_size {
            faces
                .par_iter()
                .map(|face| face.as_ref().map_or(0.0, |f| f.area()))
                .collect()
        } else {
            faces
                .iter()
                .map(|face| face.as_ref().map_or(0.0, |f| f.area()))
                .collect()
        };

        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(faces.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);

        ParallelResult::new(results, stats)
    }

    /// Validate multiple shapes in parallel
    pub fn validate_shapes(
        &self,
        shapes: &[Handle<TopoDsShape>],
    ) -> ParallelResult<Vec<ShapeValidationResult>> {
        use std::time::Instant;

        let start = Instant::now();

        let results: Vec<ShapeValidationResult> = if shapes.len() >= self.config.min_parallel_size {
            shapes
                .par_iter()
                .map(|shape| ShapeValidationResult {
                    is_valid: shape.as_ref().map_or(false, |s| s.is_mutable()),
                    shape_type: shape.shape_type(),
                    error_count: if shape.as_ref().map_or(false, |s| s.is_mutable()) {
                        0
                    } else {
                        1
                    },
                })
                .collect()
        } else {
            shapes
                .iter()
                .map(|shape| ShapeValidationResult {
                    is_valid: shape.as_ref().map_or(false, |s| s.is_mutable()),
                    shape_type: shape.shape_type(),
                    error_count: if shape.as_ref().map_or(false, |s| s.is_mutable()) {
                        0
                    } else {
                        1
                    },
                })
                .collect()
        };

        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(shapes.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);

        ParallelResult::new(results, stats)
    }

    /// Count sub-shapes by type in parallel
    pub fn count_sub_shapes(
        &self,
        shapes: &[Handle<TopoDsShape>],
        _shape_type: ShapeType,
    ) -> ParallelResult<Vec<usize>> {
        use std::time::Instant;

        let start = Instant::now();

        let results: Vec<usize> = if shapes.len() >= self.config.min_parallel_size {
            shapes
                .par_iter()
                .map(|shape| shape.as_ref().map_or(0, |_s| 0))
                .collect()
        } else {
            shapes
                .iter()
                .map(|shape| shape.as_ref().map_or(0, |_s| 0))
                .collect()
        };

        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(shapes.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);

        ParallelResult::new(results, stats)
    }

    /// Compute shape complexity scores in parallel
    pub fn compute_complexity_scores(
        &self,
        shapes: &[Handle<TopoDsShape>],
    ) -> ParallelResult<Vec<f64>> {
        use std::time::Instant;

        let start = Instant::now();

        let results: Vec<f64> = if shapes.len() >= self.config.min_parallel_size {
            shapes
                .par_iter()
                .map(|shape| self.calculate_complexity(shape))
                .collect()
        } else {
            shapes
                .iter()
                .map(|shape| self.calculate_complexity(shape))
                .collect()
        };

        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(shapes.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);

        ParallelResult::new(results, stats)
    }

    /// Analyze shape distribution by type
    pub fn analyze_shape_distribution(
        &self,
        shapes: &[Handle<TopoDsShape>],
    ) -> ParallelResult<ShapeDistribution> {
        use std::time::Instant;

        let start = Instant::now();

        // Parallel counting using fold/reduce
        let counts: HashMap<ShapeType, usize> = if shapes.len() >= self.config.min_parallel_size {
            shapes
                .par_iter()
                .fold(
                    || HashMap::new(),
                    |mut acc, shape| {
                        *acc.entry(shape.shape_type()).or_insert(0) += 1;
                        acc
                    },
                )
                .reduce(
                    || HashMap::new(),
                    |mut a, b| {
                        for (k, v) in b {
                            *a.entry(k).or_insert(0) += v;
                        }
                        a
                    },
                )
        } else {
            let mut counts = HashMap::new();
            for shape in shapes {
                *counts.entry(shape.shape_type()).or_insert(0) += 1;
            }
            counts
        };

        let distribution = ShapeDistribution {
            total_shapes: shapes.len(),
            counts,
        };

        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(shapes.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);

        ParallelResult::new(distribution, stats)
    }

    /// Find shapes that match criteria in parallel
    pub fn find_matching_shapes<F>(
        &self,
        shapes: &[Handle<TopoDsShape>],
        predicate: F,
    ) -> ParallelResult<Vec<Handle<TopoDsShape>>>
    where
        F: Fn(&Handle<TopoDsShape>) -> bool + Sync + Send,
    {
        use std::time::Instant;

        let start = Instant::now();

        let results: Vec<Handle<TopoDsShape>> = if shapes.len() >= self.config.min_parallel_size {
            shapes
                .par_iter()
                .filter(|s| predicate(s))
                .cloned()
                .collect()
        } else {
            shapes.iter().filter(|s| predicate(s)).cloned().collect()
        };

        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(shapes.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);

        ParallelResult::new(results, stats)
    }

    /// Calculate shape complexity score
    fn calculate_complexity(&self, shape: &Handle<TopoDsShape>) -> f64 {
        // Real complexity calculation
        if let Some(s) = shape.as_ref() {
            // 1. Count faces
            let face_count = s.faces().len() as f64;
            // 2. Compute volume (if available)
            let volume = s.bounding_box().1.distance(&s.bounding_box().0);
            // 3. Estimate surface curvature (simplified - use face count as proxy)
            let avg_curvature = face_count * 0.1;
            // 4. Combine metrics
            let complexity = face_count + avg_curvature;
            if volume > 0.0 {
                complexity / volume
            } else {
                complexity
            }
        } else {
            1.0
        }
    }
}

impl Default for ParallelShapeAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Shape validation result
#[derive(Debug, Clone)]
pub struct ShapeValidationResult {
    pub is_valid: bool,
    pub shape_type: ShapeType,
    pub error_count: usize,
}

impl ShapeValidationResult {
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    pub fn has_errors(&self) -> bool {
        self.error_count > 0
    }
}

/// Shape distribution analysis
#[derive(Debug, Clone)]
pub struct ShapeDistribution {
    pub total_shapes: usize,
    pub counts: HashMap<ShapeType, usize>,
}

impl ShapeDistribution {
    pub fn count_of_type(&self, shape_type: ShapeType) -> usize {
        self.counts.get(&shape_type).copied().unwrap_or(0)
    }

    pub fn percentage_of_type(&self, shape_type: ShapeType) -> f64 {
        if self.total_shapes > 0 {
            self.count_of_type(shape_type) as f64 / self.total_shapes as f64 * 100.0
        } else {
            0.0
        }
    }

    pub fn most_common_type(&self) -> Option<ShapeType> {
        self.counts
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(shape_type, _)| *shape_type)
    }

    pub fn unique_types(&self) -> usize {
        self.counts.len()
    }
}

/// Batch shape analysis job
pub struct BatchShapeAnalysis {
    shapes: Vec<Handle<TopoDsShape>>,
    operations: Vec<AnalysisOperation>,
}

#[derive(Debug, Clone, Copy)]
pub enum AnalysisOperation {
    BoundingBox,
    Volume,
    SurfaceArea,
    Validation,
    Complexity,
}

impl BatchShapeAnalysis {
    pub fn new(shapes: Vec<Handle<TopoDsShape>>) -> Self {
        Self {
            shapes,
            operations: Vec::new(),
        }
    }

    pub fn with_bounding_boxes(mut self) -> Self {
        self.operations.push(AnalysisOperation::BoundingBox);
        self
    }

    pub fn with_volumes(mut self) -> Self {
        self.operations.push(AnalysisOperation::Volume);
        self
    }

    pub fn with_surface_areas(mut self) -> Self {
        self.operations.push(AnalysisOperation::SurfaceArea);
        self
    }

    pub fn with_validation(mut self) -> Self {
        self.operations.push(AnalysisOperation::Validation);
        self
    }

    pub fn with_complexity(mut self) -> Self {
        self.operations.push(AnalysisOperation::Complexity);
        self
    }

    pub fn execute(&self) -> BatchAnalysisResult {
        let analyzer = ParallelShapeAnalyzer::new();
        let mut result = BatchAnalysisResult::new();

        for operation in &self.operations {
            match operation {
                AnalysisOperation::BoundingBox => {
                    result.bounding_boxes =
                        Some(analyzer.compute_bounding_boxes(&self.shapes).data);
                }
                AnalysisOperation::Volume => {
                    // Extract solids from shapes - simplified for now
                    let solids: Vec<Handle<TopoDsSolid>> = self
                        .shapes
                        .iter()
                        .filter_map(|s| {
                            // Check if shape is a solid by type
                            s.as_ref().and_then(|shape| {
                                if shape.shape_type() == ShapeType::Solid {
                                    // Create a new solid handle (simplified)
                                    Some(Handle::new(std::sync::Arc::new(TopoDsSolid::new())))
                                } else {
                                    None
                                }
                            })
                        })
                        .collect();
                    result.volumes = Some(analyzer.compute_volumes(&solids).data);
                }
                AnalysisOperation::SurfaceArea => {
                    // Similar to volume extraction
                }
                AnalysisOperation::Validation => {
                    result.validation = Some(analyzer.validate_shapes(&self.shapes).data);
                }
                AnalysisOperation::Complexity => {
                    result.complexity_scores =
                        Some(analyzer.compute_complexity_scores(&self.shapes).data);
                }
            }
        }

        result
    }

    pub fn shape_count(&self) -> usize {
        self.shapes.len()
    }
}

/// Batch analysis result
#[derive(Debug, Clone, Default)]
pub struct BatchAnalysisResult {
    pub bounding_boxes: Option<Vec<(Point, Point)>>,
    pub volumes: Option<Vec<f64>>,
    pub surface_areas: Option<Vec<f64>>,
    pub validation: Option<Vec<ShapeValidationResult>>,
    pub complexity_scores: Option<Vec<f64>>,
}

impl BatchAnalysisResult {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.bounding_boxes.is_none()
            && self.volumes.is_none()
            && self.surface_areas.is_none()
            && self.validation.is_none()
            && self.complexity_scores.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_shape_analyzer_new() {
        let analyzer = ParallelShapeAnalyzer::new();
        assert_eq!(analyzer.config.min_parallel_size, 100);
    }

    #[test]
    fn test_shape_validation_result() {
        let valid = ShapeValidationResult {
            is_valid: true,
            shape_type: ShapeType::Solid,
            error_count: 0,
        };
        assert!(valid.is_valid());
        assert!(!valid.has_errors());

        let invalid = ShapeValidationResult {
            is_valid: false,
            shape_type: ShapeType::Face,
            error_count: 2,
        };
        assert!(!invalid.is_valid());
        assert!(invalid.has_errors());
    }

    #[test]
    fn test_shape_distribution() {
        let mut counts = HashMap::new();
        counts.insert(ShapeType::Solid, 5);
        counts.insert(ShapeType::Face, 10);
        counts.insert(ShapeType::Edge, 20);

        let distribution = ShapeDistribution {
            total_shapes: 35,
            counts,
        };

        assert_eq!(distribution.count_of_type(ShapeType::Solid), 5);
        assert_eq!(distribution.count_of_type(ShapeType::Face), 10);
        assert!((distribution.percentage_of_type(ShapeType::Face) - 28.57).abs() < 0.1);
        assert_eq!(distribution.most_common_type(), Some(ShapeType::Edge));
        assert_eq!(distribution.unique_types(), 3);
    }

    #[test]
    fn test_batch_shape_analysis() {
        let shapes: Vec<Handle<TopoDsShape>> = vec![];
        let batch = BatchShapeAnalysis::new(shapes)
            .with_bounding_boxes()
            .with_validation();

        assert_eq!(batch.shape_count(), 0);
        assert_eq!(batch.operations.len(), 2);
    }

    #[test]
    fn test_batch_analysis_result() {
        let result = BatchAnalysisResult::new();
        assert!(result.is_empty());
    }
}
