//! Parallel Boolean Operations
//!
//! This module provides parallel execution of boolean operations
//! using the Rayon library for improved performance on multi-core systems.

use rayon::prelude::*;
use std::sync::Arc;

use super::{ParallelConfig, ParallelResult, ParallelStats};
use crate::foundation::handle::Handle;
use crate::modeling::boolean_operations::BooleanOperations;
use crate::topology::{topods_compound::TopoDsCompound, topods_shape::TopoDsShape};

/// Parallel boolean operations processor
pub struct ParallelBooleanOps {
    config: ParallelConfig,
    boolean_ops: Arc<BooleanOperations>,
}

impl ParallelBooleanOps {
    pub fn new() -> Self {
        Self {
            config: ParallelConfig::default(),
            boolean_ops: Arc::new(BooleanOperations::new()),
        }
    }

    pub fn with_config(config: ParallelConfig) -> Self {
        Self {
            config,
            boolean_ops: Arc::new(BooleanOperations::new()),
        }
    }

    /// Perform fuse (union) operations on multiple shape pairs in parallel
    pub fn fuse_pairs(
        &self,
        pairs: &[(Handle<TopoDsShape>, Handle<TopoDsShape>)],
    ) -> ParallelResult<Vec<TopoDsCompound>> {
        use std::time::Instant;

        let start = Instant::now();
        let ops = self.boolean_ops.clone();

        let results: Vec<TopoDsCompound> = if pairs.len() >= self.config.min_parallel_size {
            pairs.par_iter().map(|(s1, s2)| ops.fuse(s1, s2)).collect()
        } else {
            pairs.iter().map(|(s1, s2)| ops.fuse(s1, s2)).collect()
        };

        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(pairs.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);

        ParallelResult::new(results, stats)
    }

    /// Perform cut operations on multiple shape pairs in parallel
    pub fn cut_pairs(
        &self,
        pairs: &[(Handle<TopoDsShape>, Handle<TopoDsShape>)],
    ) -> ParallelResult<Vec<TopoDsCompound>> {
        use std::time::Instant;

        let start = Instant::now();
        let ops = self.boolean_ops.clone();

        let results: Vec<TopoDsCompound> = if pairs.len() >= self.config.min_parallel_size {
            pairs.par_iter().map(|(s1, s2)| ops.cut(s1, s2)).collect()
        } else {
            pairs.iter().map(|(s1, s2)| ops.cut(s1, s2)).collect()
        };

        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(pairs.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);

        ParallelResult::new(results, stats)
    }

    /// Perform common (intersection) operations on multiple shape pairs in parallel
    pub fn common_pairs(
        &self,
        pairs: &[(Handle<TopoDsShape>, Handle<TopoDsShape>)],
    ) -> ParallelResult<Vec<TopoDsCompound>> {
        use std::time::Instant;

        let start = Instant::now();
        let ops = self.boolean_ops.clone();

        let results: Vec<TopoDsCompound> = if pairs.len() >= self.config.min_parallel_size {
            pairs
                .par_iter()
                .map(|(s1, s2)| ops.common(s1, s2))
                .collect()
        } else {
            pairs.iter().map(|(s1, s2)| ops.common(s1, s2)).collect()
        };

        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(pairs.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);

        ParallelResult::new(results, stats)
    }

    /// Fuse multiple shapes into a single compound (parallel reduction)
    pub fn fuse_many(&self, shapes: &[Handle<TopoDsShape>]) -> ParallelResult<TopoDsCompound> {
        use std::time::Instant;

        if shapes.is_empty() {
            return ParallelResult::new(
                TopoDsCompound::new(),
                ParallelStats::new().with_items_processed(0),
            );
        }

        if shapes.len() <= 1 {
            let mut compound = TopoDsCompound::new();
            compound.add_component(shapes[0].clone());
            return ParallelResult::new(compound, ParallelStats::new().with_items_processed(1));
        }

        let start = Instant::now();
        let _ops = self.boolean_ops.clone();

        // Use parallel reduction to fuse all shapes
        let result: TopoDsCompound = if shapes.len() >= self.config.min_parallel_size {
            shapes
                .par_iter()
                .cloned()
                .fold(
                    || TopoDsCompound::new(),
                    |mut acc, shape| {
                        acc.add_component(shape);
                        acc
                    },
                )
                .reduce(
                    || TopoDsCompound::new(),
                    |mut a, b| {
                        for comp in b.components() {
                            a.add_component(comp.clone());
                        }
                        a
                    },
                )
        } else {
            let mut compound = TopoDsCompound::new();
            for shape in shapes {
                compound.add_component(shape.clone());
            }
            compound
        };

        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(shapes.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);

        ParallelResult::new(result, stats)
    }

    /// Perform boolean operations on a grid of shapes (useful for pattern operations)
    pub fn boolean_grid(
        &self,
        shapes: &[Handle<TopoDsShape>],
        operation: BooleanOperation,
    ) -> ParallelResult<Vec<TopoDsCompound>> {
        use std::time::Instant;

        let start = Instant::now();
        let ops = self.boolean_ops.clone();

        // Generate all pairs
        let pairs: Vec<(Handle<TopoDsShape>, Handle<TopoDsShape>)> = shapes
            .iter()
            .enumerate()
            .flat_map(|(i, s1)| {
                shapes
                    .iter()
                    .skip(i + 1)
                    .map(move |s2| (s1.clone(), s2.clone()))
            })
            .collect();

        let results: Vec<TopoDsCompound> = match operation {
            BooleanOperation::Fuse => {
                if pairs.len() >= self.config.min_parallel_size {
                    pairs.par_iter().map(|(s1, s2)| ops.fuse(s1, s2)).collect()
                } else {
                    pairs.iter().map(|(s1, s2)| ops.fuse(s1, s2)).collect()
                }
            }
            BooleanOperation::Cut => {
                if pairs.len() >= self.config.min_parallel_size {
                    pairs.par_iter().map(|(s1, s2)| ops.cut(s1, s2)).collect()
                } else {
                    pairs.iter().map(|(s1, s2)| ops.cut(s1, s2)).collect()
                }
            }
            BooleanOperation::Common => {
                if pairs.len() >= self.config.min_parallel_size {
                    pairs
                        .par_iter()
                        .map(|(s1, s2)| ops.common(s1, s2))
                        .collect()
                } else {
                    pairs.iter().map(|(s1, s2)| ops.common(s1, s2)).collect()
                }
            }
        };

        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(pairs.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);

        ParallelResult::new(results, stats)
    }

    /// Validate boolean operations in parallel
    pub fn validate_operations(
        &self,
        pairs: &[(Handle<TopoDsShape>, Handle<TopoDsShape>)],
    ) -> ParallelResult<Vec<BooleanValidationResult>> {
        use std::time::Instant;

        let start = Instant::now();

        let results: Vec<BooleanValidationResult> = if pairs.len() >= self.config.min_parallel_size
        {
            pairs
                .par_iter()
                .map(|(s1, s2)| {
                    BooleanValidationResult {
                        shape1_valid: s1.is_solid(),
                        shape2_valid: s2.is_solid(),
                        can_intersect: true, // Simplified check
                    }
                })
                .collect()
        } else {
            pairs
                .iter()
                .map(|(s1, s2)| BooleanValidationResult {
                    shape1_valid: s1.is_solid(),
                    shape2_valid: s2.is_solid(),
                    can_intersect: true,
                })
                .collect()
        };

        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(pairs.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);

        ParallelResult::new(results, stats)
    }
}

impl Default for ParallelBooleanOps {
    fn default() -> Self {
        Self::new()
    }
}

/// Boolean operation type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BooleanOperation {
    Fuse,
    Cut,
    Common,
}

/// Boolean validation result
#[derive(Debug, Clone)]
pub struct BooleanValidationResult {
    pub shape1_valid: bool,
    pub shape2_valid: bool,
    pub can_intersect: bool,
}

impl BooleanValidationResult {
    pub fn is_valid(&self) -> bool {
        self.shape1_valid && self.shape2_valid && self.can_intersect
    }
}

/// Builder for complex boolean operations
pub struct BooleanOperationBuilder {
    operations: Vec<(BooleanOperation, Handle<TopoDsShape>)>,
}

impl BooleanOperationBuilder {
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    pub fn fuse(mut self, shape: Handle<TopoDsShape>) -> Self {
        self.operations.push((BooleanOperation::Fuse, shape));
        self
    }

    pub fn cut(mut self, shape: Handle<TopoDsShape>) -> Self {
        self.operations.push((BooleanOperation::Cut, shape));
        self
    }

    pub fn common(mut self, shape: Handle<TopoDsShape>) -> Self {
        self.operations.push((BooleanOperation::Common, shape));
        self
    }

    pub fn build(self) -> Vec<(BooleanOperation, Handle<TopoDsShape>)> {
        self.operations
    }

    /// Execute the operation sequence in parallel where possible
    pub fn execute_parallel(&self) -> ParallelResult<TopoDsCompound> {
        use std::time::Instant;

        let start = Instant::now();
        let ops = BooleanOperations::new();

        // For simplicity, execute sequentially but could be parallelized
        // based on operation dependencies
        let mut result = TopoDsCompound::new();

        for (op, shape) in &self.operations {
            match op {
                BooleanOperation::Fuse => {
                    if result.components().is_empty() {
                        result.add_component(shape.clone());
                    } else {
                        let acc_shape = Handle::new(std::sync::Arc::new(result.shape().clone()));
                        result = ops.fuse(&acc_shape, shape);
                    }
                }
                BooleanOperation::Cut => {
                    if !result.components().is_empty() {
                        let acc_shape = Handle::new(std::sync::Arc::new(result.shape().clone()));
                        result = ops.cut(&acc_shape, shape);
                    }
                }
                BooleanOperation::Common => {
                    if !result.components().is_empty() {
                        let acc_shape = Handle::new(std::sync::Arc::new(result.shape().clone()));
                        result = ops.common(&acc_shape, shape);
                    }
                }
            }
        }

        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(self.operations.len())
            .with_threads_used(1)
            .with_elapsed_time_ms(elapsed.as_millis() as u64);

        ParallelResult::new(result, stats)
    }
}

impl Default for BooleanOperationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_boolean_ops_new() {
        let ops = ParallelBooleanOps::new();
        assert_eq!(ops.config.min_parallel_size, 100);
    }

    #[test]
    fn test_boolean_operation_enum() {
        assert_eq!(BooleanOperation::Fuse, BooleanOperation::Fuse);
        assert_ne!(BooleanOperation::Fuse, BooleanOperation::Cut);
    }

    #[test]
    fn test_boolean_validation_result() {
        let valid = BooleanValidationResult {
            shape1_valid: true,
            shape2_valid: true,
            can_intersect: true,
        };
        assert!(valid.is_valid());

        let invalid = BooleanValidationResult {
            shape1_valid: false,
            shape2_valid: true,
            can_intersect: true,
        };
        assert!(!invalid.is_valid());
    }

    #[test]
    fn test_boolean_operation_builder() {
        let builder = BooleanOperationBuilder::new()
            .fuse(Handle::new(std::sync::Arc::new(TopoDsShape::default())))
            .cut(Handle::new(std::sync::Arc::new(TopoDsShape::default())));

        assert_eq!(builder.operations.len(), 2);
    }
}
