//! Parallel geometry algorithms
//!
//! This module provides parallel implementations of geometry algorithms.

use rayon::prelude::*;

use crate::foundation::handle::Handle;
use crate::geometry::Point;
use crate::topology::topods_shape::TopoDsShape;

/// Parallel geometry processing result
#[derive(Debug, Clone)]
pub struct ParallelGeometryResult {
    /// Processed shapes
    pub shapes: Vec<Handle<TopoDsShape>>,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
}

impl ParallelGeometryResult {
    /// Create a new result
    pub fn new(shapes: Vec<Handle<TopoDsShape>>, processing_time_ms: u64) -> Self {
        Self {
            shapes,
            processing_time_ms,
        }
    }
}

/// Parallel distance calculation
pub fn parallel_distance_calculations(points: &[Point], reference: &Point) -> Vec<f64> {
    points.par_iter().map(|p| p.distance(reference)).collect()
}

/// Parallel bounding box calculation
pub fn parallel_bounding_box(shapes: &[Handle<TopoDsShape>]) -> Option<(Point, Point)> {
    if shapes.is_empty() {
        return None;
    }

    // Calculate individual bounding boxes in parallel
    let bboxes: Vec<Option<(Point, Point)>> = shapes
        .par_iter()
        .map(|shape| {
            shape.as_ref().map(|s| s.bounding_box())
        })
        .collect();

    // Combine all bounding boxes
    let mut min = Point::new(f64::MAX, f64::MAX, f64::MAX);
    let mut max = Point::new(f64::MIN, f64::MIN, f64::MIN);

    for bbox in bboxes {
        if let Some((bbox_min, bbox_max)) = bbox {
            min.x = min.x.min(bbox_min.x);
            min.y = min.y.min(bbox_min.y);
            min.z = min.z.min(bbox_min.z);
            max.x = max.x.max(bbox_max.x);
            max.y = max.y.max(bbox_max.y);
            max.z = max.z.max(bbox_max.z);
        }
    }

    Some((min, max))
}
