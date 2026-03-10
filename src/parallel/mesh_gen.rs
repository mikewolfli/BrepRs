//! Parallel Mesh Generation
//!
//! This module provides parallel mesh generation capabilities
//! using the Rayon library for improved performance on multi-core systems.

use rayon::prelude::*;
use std::sync::Arc;

use super::{ParallelConfig, ParallelResult, ParallelStats};
use crate::foundation::handle::Handle;
use crate::mesh::{Mesh, Mesh2D, MeshGenerator, MeshQuality, MeshingAlgorithm};
use crate::topology::{
    topods_face::TopoDsFace, topods_shape::TopoDsShape, topods_solid::TopoDsSolid,
};

/// Parallel mesh generator
pub struct ParallelMeshGenerator {
    config: ParallelConfig,
    generator: Arc<MeshGenerator>,
}

impl ParallelMeshGenerator {
    pub fn new() -> Self {
        Self {
            config: ParallelConfig::default(),
            generator: Arc::new(MeshGenerator::new()),
        }
    }

    pub fn with_config(config: ParallelConfig) -> Self {
        Self {
            config,
            generator: Arc::new(MeshGenerator::new()),
        }
    }

    /// Generate meshes for multiple shapes in parallel
    pub fn generate_meshes(
        &self,
        shapes: &[Handle<TopoDsShape>],
        linear_deflection: f64,
        angular_deflection: f64,
    ) -> ParallelResult<Vec<Mesh>> {
        use std::time::Instant;

        let start = Instant::now();
        let gen = self.generator.clone();

        let results: Vec<Mesh> = if shapes.len() >= self.config.min_parallel_size {
            shapes
                .par_iter()
                .map(|shape| gen.generate(shape, linear_deflection, angular_deflection))
                .collect()
        } else {
            shapes
                .iter()
                .map(|shape| gen.generate(shape, linear_deflection, angular_deflection))
                .collect()
        };

        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(shapes.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);

        ParallelResult::new(results, stats)
    }

    /// Generate meshes for multiple faces in parallel
    pub fn generate_face_meshes(
        &self,
        faces: &[Handle<TopoDsFace>],
        linear_deflection: f64,
        angular_deflection: f64,
    ) -> ParallelResult<Vec<Mesh2D>> {
        use std::time::Instant;

        let start = Instant::now();
        let gen = self.generator.clone();

        let results: Vec<Mesh2D> = if faces.len() >= self.config.min_parallel_size {
            faces
                .par_iter()
                .map(|face| gen.generate_face(face, linear_deflection, angular_deflection))
                .collect()
        } else {
            faces
                .iter()
                .map(|face| gen.generate_face(face, linear_deflection, angular_deflection))
                .collect()
        };

        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(faces.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);

        ParallelResult::new(results, stats)
    }

    /// Generate tetrahedral meshes for multiple solids in parallel
    pub fn generate_tetrahedral_meshes(
        &self,
        solids: &[Handle<TopoDsSolid>],
        max_edge_length: f64,
    ) -> ParallelResult<Vec<crate::mesh::TetMesh>> {
        use std::time::Instant;

        let start = Instant::now();
        let gen = self.generator.clone();

        let results: Vec<crate::mesh::TetMesh> = if solids.len() >= self.config.min_parallel_size {
            solids
                .par_iter()
                .map(|solid| gen.generate_tetrahedral(solid, max_edge_length))
                .collect()
        } else {
            solids
                .iter()
                .map(|solid| gen.generate_tetrahedral(solid, max_edge_length))
                .collect()
        };

        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(solids.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);

        ParallelResult::new(results, stats)
    }

    /// Evaluate mesh quality for multiple meshes in parallel
    pub fn evaluate_mesh_quality(&self, meshes: &[Mesh]) -> ParallelResult<Vec<MeshQuality>> {
        use std::time::Instant;

        let start = Instant::now();
        let gen = self.generator.clone();

        let results: Vec<MeshQuality> = if meshes.len() >= self.config.min_parallel_size {
            meshes
                .par_iter()
                .map(|mesh| gen.evaluate_quality(mesh))
                .collect()
        } else {
            meshes
                .iter()
                .map(|mesh| gen.evaluate_quality(mesh))
                .collect()
        };

        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(meshes.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);

        ParallelResult::new(results, stats)
    }

    /// Optimize meshes in parallel
    pub fn optimize_meshes(
        &self,
        meshes: &mut [Mesh],
        iterations: usize,
    ) -> ParallelResult<Vec<MeshQuality>> {
        use std::time::Instant;

        let start = Instant::now();
        let gen = self.generator.clone();

        // Note: In-place parallel mutation requires careful handling
        // This is a simplified version
        let results: Vec<MeshQuality> = if meshes.len() >= self.config.min_parallel_size {
            meshes
                .par_iter_mut()
                .map(|mesh| {
                    (*gen).optimize(mesh, iterations);
                    gen.evaluate_quality(mesh)
                })
                .collect()
        } else {
            meshes
                .iter_mut()
                .map(|mesh| {
                    (*gen).optimize(mesh, iterations);
                    gen.evaluate_quality(mesh)
                })
                .collect()
        };

        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(meshes.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);

        ParallelResult::new(results, stats)
    }

    /// Generate meshes with different quality parameters in parallel
    pub fn generate_meshes_varied_quality(
        &self,
        shape: &Handle<TopoDsShape>,
        quality_params: &[(f64, f64)], // (linear_deflection, angular_deflection) pairs
    ) -> ParallelResult<Vec<Mesh>> {
        use std::time::Instant;

        let start = Instant::now();
        let gen = self.generator.clone();
        let shape = shape.clone();

        let results: Vec<Mesh> = if quality_params.len() >= self.config.min_parallel_size {
            quality_params
                .par_iter()
                .map(|(linear, angular)| gen.generate(&shape, *linear, *angular))
                .collect()
        } else {
            quality_params
                .iter()
                .map(|(linear, angular)| gen.generate(&shape, *linear, *angular))
                .collect()
        };

        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(quality_params.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);

        ParallelResult::new(results, stats)
    }

    /// Compute mesh statistics in parallel
    pub fn compute_mesh_statistics(&self, meshes: &[Mesh]) -> ParallelResult<MeshStatistics> {
        use std::time::Instant;

        let start = Instant::now();

        let total_vertices: usize = if meshes.len() >= self.config.min_parallel_size {
            meshes
                .par_iter()
                .map(|m: &crate::mesh::mesh_data::Mesh2D| m.vertex_count())
                .sum()
        } else {
            meshes
                .iter()
                .map(|m: &crate::mesh::mesh_data::Mesh2D| m.vertex_count())
                .sum()
        };

        let total_triangles: usize = if meshes.len() >= self.config.min_parallel_size {
            meshes
                .par_iter()
                .map(|m: &crate::mesh::mesh_data::Mesh2D| m.triangle_count())
                .sum()
        } else {
            meshes
                .iter()
                .map(|m: &crate::mesh::mesh_data::Mesh2D| m.triangle_count())
                .sum()
        };

        let avg_quality: f64 = if meshes.len() >= self.config.min_parallel_size {
            let sum: f64 = meshes
                .par_iter()
                .map(|m| self.generator.evaluate_quality(m).overall_quality())
                .sum();
            sum / meshes.len() as f64
        } else {
            let sum: f64 = meshes
                .iter()
                .map(|m| self.generator.evaluate_quality(m).overall_quality())
                .sum();
            sum / meshes.len() as f64
        };

        let stats = MeshStatistics {
            total_vertices,
            total_triangles,
            mesh_count: meshes.len(),
            average_quality: avg_quality,
        };

        let elapsed = start.elapsed();
        let parallel_stats = ParallelStats::new()
            .with_items_processed(meshes.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);

        ParallelResult::new(stats, parallel_stats)
    }
}

impl Default for ParallelMeshGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Mesh statistics
#[derive(Debug, Clone)]
pub struct MeshStatistics {
    pub total_vertices: usize,
    pub total_triangles: usize,
    pub mesh_count: usize,
    pub average_quality: f64,
}

impl MeshStatistics {
    pub fn average_vertices_per_mesh(&self) -> f64 {
        if self.mesh_count > 0 {
            self.total_vertices as f64 / self.mesh_count as f64
        } else {
            0.0
        }
    }

    pub fn average_triangles_per_mesh(&self) -> f64 {
        if self.mesh_count > 0 {
            self.total_triangles as f64 / self.mesh_count as f64
        } else {
            0.0
        }
    }
}

/// Batch mesh generation job
pub struct BatchMeshJob {
    shapes: Vec<Handle<TopoDsShape>>,
    linear_deflection: f64,
    angular_deflection: f64,
    algorithm: MeshingAlgorithm,
}

impl BatchMeshJob {
    pub fn new(
        shapes: Vec<Handle<TopoDsShape>>,
        linear_deflection: f64,
        angular_deflection: f64,
    ) -> Self {
        Self {
            shapes,
            linear_deflection,
            angular_deflection,
            algorithm: MeshingAlgorithm::Delaunay,
        }
    }

    pub fn with_algorithm(mut self, algorithm: MeshingAlgorithm) -> Self {
        self.algorithm = algorithm;
        self
    }

    pub fn execute(&self) -> ParallelResult<Vec<Mesh>> {
        let generator = ParallelMeshGenerator::new();
        generator.generate_meshes(
            &self.shapes,
            self.linear_deflection,
            self.angular_deflection,
        )
    }

    pub fn shape_count(&self) -> usize {
        self.shapes.len()
    }
}

/// Adaptive mesh generation that adjusts quality based on shape complexity
pub struct AdaptiveMeshGenerator {
    base_generator: ParallelMeshGenerator,
    quality_threshold: f64,
}

impl AdaptiveMeshGenerator {
    pub fn new() -> Self {
        Self {
            base_generator: ParallelMeshGenerator::new(),
            quality_threshold: 0.8,
        }
    }

    pub fn with_quality_threshold(mut self, threshold: f64) -> Self {
        self.quality_threshold = threshold;
        self
    }

    /// Generate meshes with adaptive quality based on shape complexity
    pub fn generate_adaptive(&self, shapes: &[Handle<TopoDsShape>]) -> ParallelResult<Vec<Mesh>> {
        use std::time::Instant;

        let start = Instant::now();
        let gen = self.base_generator.generator.clone();

        // First pass: estimate complexity in parallel
        let complexities: Vec<f64> = if shapes.len() >= self.base_generator.config.min_parallel_size
        {
            shapes
                .par_iter()
                .map(|shape| self.estimate_complexity(shape))
                .collect()
        } else {
            shapes
                .iter()
                .map(|shape| self.estimate_complexity(shape))
                .collect()
        };

        // Second pass: generate meshes with adaptive parameters
        let results: Vec<Mesh> = shapes
            .iter()
            .zip(complexities.iter())
            .map(|(shape, complexity)| {
                let (linear, angular) = self.compute_parameters(*complexity);
                gen.generate(shape, linear, angular)
            })
            .collect();

        let elapsed = start.elapsed();
        let stats = ParallelStats::new()
            .with_items_processed(shapes.len())
            .with_threads_used(rayon::current_num_threads())
            .with_elapsed_time_ms(elapsed.as_millis() as u64);

        ParallelResult::new(results, stats)
    }

    fn estimate_complexity(&self, _shape: &Handle<TopoDsShape>) -> f64 {
        // Simplified complexity estimation
        // In a real implementation, this would analyze the shape's geometric complexity
        1.0
    }

    fn compute_parameters(&self, complexity: f64) -> (f64, f64) {
        // Adjust parameters based on complexity
        let linear = 0.1 / complexity;
        let angular = 0.5 / complexity;
        (linear, angular)
    }
}

impl Default for AdaptiveMeshGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_mesh_generator_new() {
        let gen = ParallelMeshGenerator::new();
        assert_eq!(gen.config.min_parallel_size, 100);
    }

    #[test]
    fn test_mesh_statistics() {
        let stats = MeshStatistics {
            total_vertices: 1000,
            total_triangles: 500,
            mesh_count: 10,
            average_quality: 0.85,
        };

        assert_eq!(stats.average_vertices_per_mesh(), 100.0);
        assert_eq!(stats.average_triangles_per_mesh(), 50.0);
    }

    #[test]
    fn test_batch_mesh_job() {
        let shapes: Vec<Handle<TopoDsShape>> = vec![];
        let job = BatchMeshJob::new(shapes, 0.1, 0.5);
        assert_eq!(job.shape_count(), 0);
    }

    #[test]
    fn test_adaptive_mesh_generator() {
        let gen = AdaptiveMeshGenerator::new().with_quality_threshold(0.9);
        assert!((gen.quality_threshold - 0.9).abs() < 1e-10);
    }
}
