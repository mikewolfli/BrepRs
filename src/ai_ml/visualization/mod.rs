//! ML Visualization Module
//!
//! This module provides visualization tools for ML results, including feature recognition,
//! model performance, and training progress visualization.

use std::path::Path;
use std::io::Write;

use crate::geometry::Point;
use crate::mesh::mesh_data::Mesh3D;

/// Performance metrics for monitoring
#[derive(Clone)]
pub struct PerformanceMetrics {
    pub inference_time: f64,    // In milliseconds
    pub render_time: f64,       // In milliseconds
    pub memory_usage: f64,      // In MB
    pub mesh_complexity: usize, // Number of vertices
    pub fps: f64,               // Frames per second
}

/// ML Visualization Tool
pub struct MlVisualization {
    output_dir: String,
    performance_metrics: Vec<PerformanceMetrics>,
}

impl MlVisualization {
    pub fn new(output_dir: &str) -> Self {
        std::fs::create_dir_all(output_dir).unwrap_or(());
        Self {
            output_dir: output_dir.to_string(),
            performance_metrics: Vec::new(),
        }
    }

    /// Visualize feature recognition results
    pub fn visualize_features(&self, mesh: &Mesh3D, features: &[String]) -> Result<(), String> {
        // Create a visualization of the mesh with recognized features
        let output_path = Path::new(&self.output_dir).join("feature_visualization.obj");
        self.save_mesh_with_features(mesh, features, &output_path)
    }

    /// Visualize model training progress
    pub fn visualize_training_progress(
        &self,
        epochs: &[usize],
        accuracy: &[f64],
    ) -> Result<(), String> {
        // Create a plot of training progress
        let output_path = Path::new(&self.output_dir).join("training_progress.svg");
        self.save_training_plot(epochs, accuracy, &output_path)
    }

    /// Visualize model performance
    pub fn visualize_performance(
        &self,
        metrics: &std::collections::HashMap<String, f64>,
    ) -> Result<(), String> {
        // Create a performance report
        let output_path = Path::new(&self.output_dir).join("performance_report.txt");
        self.save_performance_report(metrics, &output_path)
    }

    /// Save mesh with features to OBJ file
    fn save_mesh_with_features(
        &self,
        mesh: &Mesh3D,
        features: &[String],
        path: &Path,
    ) -> Result<(), String> {
        use std::io::Write;
        let mut file = std::fs::File::create(path).map_err(|e| e.to_string())?;

        // Write vertices
        for vertex in &mesh.vertices {
            writeln!(
                file,
                "v {} {} {}",
                vertex.point.x, vertex.point.y, vertex.point.z
            )
            .map_err(|e| e.to_string())?;
        }

        // Write faces
        for face in &mesh.faces {
            write!(file, "f").map_err(|e| e.to_string())?;
            for &vertex_id in &face.vertices {
                write!(file, " {}", vertex_id + 1).map_err(|e| e.to_string())?;
            }
            writeln!(file).map_err(|e| e.to_string())?;
        }

        // Write features as comments
        writeln!(file, "# Recognized features:").map_err(|e| e.to_string())?;
        for feature in features {
            writeln!(file, "# - {}", feature).map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    /// Save training progress plot
    fn save_training_plot(
        &self,
        epochs: &[usize],
        accuracy: &[f64],
        path: &Path,
    ) -> Result<(), String> {
        use std::io::Write;
        let mut file = std::fs::File::create(path).map_err(|e| e.to_string())?;

        // Simple SVG plot
        let width = 800;
        let height = 600;
        let padding = 50;

        let max_epoch = *epochs.last().unwrap_or(&1);
        let max_accuracy = accuracy.iter().cloned().fold(0.0, f64::max);

        writeln!(
            file,
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#,
            width, height
        )
        .map_err(|e| e.to_string())?;

        // Draw axes
        writeln!(
            file,
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="black" stroke-width="2"/>"#,
            padding,
            height - padding,
            width - padding,
            height - padding
        )
        .map_err(|e| e.to_string())?;
        writeln!(
            file,
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="black" stroke-width="2"/>"#,
            padding,
            height - padding,
            padding,
            padding
        )
        .map_err(|e| e.to_string())?;

        // Draw data points
        for i in 0..epochs.len() {
            let x = padding as f64
                + (epochs[i] as f64 / max_epoch as f64) * (width - 2 * padding) as f64;
            let y = (height - padding) as f64
                - (accuracy[i] / max_accuracy) * (height - 2 * padding) as f64;

            if i > 0 {
                let prev_x = padding as f64
                    + (epochs[i - 1] as f64 / max_epoch as f64) * (width - 2 * padding) as f64;
                let prev_y = (height - padding) as f64
                    - (accuracy[i - 1] / max_accuracy) * (height - 2 * padding) as f64;
                writeln!(
                    file,
                    r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="blue" stroke-width="2"/>"#,
                    prev_x, prev_y, x, y
                )
                .map_err(|e| e.to_string())?;
            }

            writeln!(file, r#"<circle cx="{}" cy="{}" r="4" fill="red"/>"#, x, y)
                .map_err(|e| e.to_string())?;
        }

        // Add labels
        writeln!(
            file,
            r#"<text x="{}" y="{}" text-anchor="middle">Epoch</text>"#,
            width / 2,
            height - 10
        )
        .map_err(|e| e.to_string())?;
        writeln!(
            file,
            r#"<text x="{}" y="{}" text-anchor="middle" transform="rotate(-90, {}, {})">Accuracy</text>"#,
            20, height / 2, 20, height / 2
        ).map_err(|e| e.to_string())?;

        writeln!(file, r#"</svg>"#).map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Save performance report
    fn save_performance_report(
        &self,
        metrics: &std::collections::HashMap<String, f64>,
        path: &Path,
    ) -> Result<(), String> {
        use std::io::Write;
        let mut file = std::fs::File::create(path).map_err(|e| e.to_string())?;

        writeln!(file, "Model Performance Report").map_err(|e| e.to_string())?;
        writeln!(file, "========================").map_err(|e| e.to_string())?;
        writeln!(file).map_err(|e| e.to_string())?;

        for (metric, value) in metrics {
            writeln!(file, "{}: {:.4}", metric, value).map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    /// Visualize mesh comparison (e.g., original vs repaired)
    pub fn visualize_mesh_comparison(
        &self,
        original: &Mesh3D,
        repaired: &Mesh3D,
    ) -> Result<(), String> {
        // Save both meshes to separate files
        let original_path = Path::new(&self.output_dir).join("original_mesh.obj");
        let repaired_path = Path::new(&self.output_dir).join("repaired_mesh.obj");

        self.save_mesh(original, &original_path)?;
        self.save_mesh(repaired, &repaired_path)?;

        Ok(())
    }

    /// Save mesh to OBJ file
    fn save_mesh(&self, mesh: &Mesh3D, path: &Path) -> Result<(), String> {
        use std::io::Write;
        let mut file = std::fs::File::create(path).map_err(|e| e.to_string())?;

        // Write vertices
        for vertex in &mesh.vertices {
            writeln!(
                file,
                "v {} {} {}",
                vertex.point.x, vertex.point.y, vertex.point.z
            )
            .map_err(|e| e.to_string())?;
        }

        // Write faces
        for face in &mesh.faces {
            write!(file, "f").map_err(|e| e.to_string())?;
            for &vertex_id in &face.vertices {
                write!(file, " {}", vertex_id + 1).map_err(|e| e.to_string())?;
            }
            writeln!(file).map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    /// Add performance metrics for monitoring
    pub fn add_performance_metrics(&mut self, metrics: PerformanceMetrics) {
        self.performance_metrics.push(metrics);
    }

    /// Visualize performance metrics over time
    pub fn visualize_performance_metrics(&self) -> Result<(), String> {
        if self.performance_metrics.is_empty() {
            return Err("No performance metrics to visualize".to_string());
        }

        let output_path = Path::new(&self.output_dir).join("performance_metrics.svg");
        self.save_performance_metrics_plot(&output_path)
    }

    /// Save performance metrics plot
    fn save_performance_metrics_plot(&self, path: &Path) -> Result<(), String> {
        use std::io::Write;
        let mut file = std::fs::File::create(path).map_err(|e| e.to_string())?;

        // Simple SVG plot
        let width = 1000;
        let height = 800;
        let padding = 60;

        // Prepare data
        let steps: Vec<usize> = (0..self.performance_metrics.len()).collect();
        let inference_times: Vec<f64> = self
            .performance_metrics
            .iter()
            .map(|m| m.inference_time)
            .collect();
        let render_times: Vec<f64> = self
            .performance_metrics
            .iter()
            .map(|m| m.render_time)
            .collect();
        let fps_values: Vec<f64> = self.performance_metrics.iter().map(|m| m.fps).collect();

        let max_time = inference_times
            .iter()
            .chain(render_times.iter())
            .cloned()
            .fold(0.0, f64::max);
        let max_fps = fps_values.iter().cloned().fold(0.0, f64::max);

        writeln!(
            file,
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#,
            width, height
        )
        .map_err(|e| e.to_string())?;

        // Draw axes for time metrics
        writeln!(
            file,
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="black" stroke-width="2"/>"#,
            padding,
            height - padding - 200,
            width - padding,
            height - padding - 200
        )
        .map_err(|e| e.to_string())?;
        writeln!(
            file,
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="black" stroke-width="2"/>"#,
            padding,
            height - padding - 200,
            padding,
            padding
        )
        .map_err(|e| e.to_string())?;

        // Draw axes for FPS
        writeln!(
            file,
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="black" stroke-width="2"/>"#,
            padding,
            height - padding,
            width - padding,
            height - padding
        )
        .map_err(|e| e.to_string())?;
        writeln!(
            file,
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="black" stroke-width="2"/>"#,
            padding,
            height - padding,
            padding,
            height - padding - 150
        )
        .map_err(|e| e.to_string())?;

        // Draw inference time
        for i in 0..steps.len() {
            let x = padding as f64
                + (steps[i] as f64 / (steps.len() - 1) as f64) * (width - 2 * padding) as f64;
            let y = (height - padding - 200) as f64
                - (inference_times[i] / max_time) * (height - 2 * padding - 200) as f64;

            if i > 0 {
                let prev_x = padding as f64
                    + (steps[i - 1] as f64 / (steps.len() - 1) as f64)
                        * (width - 2 * padding) as f64;
                let prev_y = (height - padding - 200) as f64
                    - (inference_times[i - 1] / max_time) * (height - 2 * padding - 200) as f64;
                writeln!(
                    file,
                    r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="red" stroke-width="2"/>"#,
                    prev_x, prev_y, x, y
                )
                .map_err(|e| e.to_string())?;
            }

            writeln!(file, r#"<circle cx="{}" cy="{}" r="3" fill="red"/>"#, x, y)
                .map_err(|e| e.to_string())?;
        }

        // Draw render time
        for i in 0..steps.len() {
            let x = padding as f64
                + (steps[i] as f64 / (steps.len() - 1) as f64) * (width - 2 * padding) as f64;
            let y = (height - padding - 200) as f64
                - (render_times[i] / max_time) * (height - 2 * padding - 200) as f64;

            if i > 0 {
                let prev_x = padding as f64
                    + (steps[i - 1] as f64 / (steps.len() - 1) as f64)
                        * (width - 2 * padding) as f64;
                let prev_y = (height - padding - 200) as f64
                    - (render_times[i - 1] / max_time) * (height - 2 * padding - 200) as f64;
                writeln!(
                    file,
                    r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="blue" stroke-width="2"/>"#,
                    prev_x, prev_y, x, y
                )
                .map_err(|e| e.to_string())?;
            }

            writeln!(file, r#"<circle cx="{}" cy="{}" r="3" fill="blue"/>"#, x, y)
                .map_err(|e| e.to_string())?;
        }

        // Draw FPS
        for i in 0..steps.len() {
            let x = padding as f64
                + (steps[i] as f64 / (steps.len() - 1) as f64) * (width - 2 * padding) as f64;
            let y = (height - padding) as f64 - (fps_values[i] / max_fps) * 150.0;

            if i > 0 {
                let prev_x = padding as f64
                    + (steps[i - 1] as f64 / (steps.len() - 1) as f64)
                        * (width - 2 * padding) as f64;
                let prev_y = (height - padding) as f64 - (fps_values[i - 1] / max_fps) * 150.0;
                writeln!(
                    file,
                    r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="green" stroke-width="2"/>"#,
                    prev_x, prev_y, x, y
                )
                .map_err(|e| e.to_string())?;
            }

            writeln!(
                file,
                r#"<circle cx="{}" cy="{}" r="3" fill="green"/>"#,
                x, y
            )
            .map_err(|e| e.to_string())?;
        }

        // Add labels
        writeln!(
            file,
            r#"<text x="{}" y="{}" text-anchor="middle">Step</text>"#,
            width / 2,
            height - 10
        )
        .map_err(|e| e.to_string())?;
        writeln!(
            file,
            r#"<text x="{}" y="{}" text-anchor="middle" transform="rotate(-90, {}, {})">Time (ms)</text>"#,
            20, (height - 200) / 2, 20, (height - 200) / 2
        ).map_err(|e| e.to_string())?;
        writeln!(
            file,
            r#"<text x="{}" y="{}" text-anchor="middle" transform="rotate(-90, {}, {})">FPS</text>"#,
            20, height - 100, 20, height - 100
        ).map_err(|e| e.to_string())?;

        // Add legend
        writeln!(
            file,
            r#"<rect x="{}" y="{}" width="20" height="20" fill="red"/>"#,
            width - padding - 150,
            padding + 10
        )
        .map_err(|e| e.to_string())?;
        writeln!(
            file,
            r#"<text x="{}" y="{}">Inference Time</text>"#,
            width - padding - 120,
            padding + 25
        )
        .map_err(|e| e.to_string())?;
        writeln!(
            file,
            r#"<rect x="{}" y="{}" width="20" height="20" fill="blue"/>"#,
            width - padding - 150,
            padding + 40
        )
        .map_err(|e| e.to_string())?;
        writeln!(
            file,
            r#"<text x="{}" y="{}">Render Time</text>"#,
            width - padding - 120,
            padding + 55
        )
        .map_err(|e| e.to_string())?;
        writeln!(
            file,
            r#"<rect x="{}" y="{}" width="20" height="20" fill="green"/>"#,
            width - padding - 150,
            padding + 70
        )
        .map_err(|e| e.to_string())?;
        writeln!(
            file,
            r#"<text x="{}" y="{}">FPS</text>"#,
            width - padding - 120,
            padding + 85
        )
        .map_err(|e| e.to_string())?;

        writeln!(file, r#"</svg>"#).map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Visualize model structure
    pub fn visualize_model_structure(
        &self,
        model_name: &str,
        layers: &[(String, usize)],
    ) -> Result<(), String> {
        let output_path = Path::new(&self.output_dir).join(format!("{}_structure.svg", model_name));
        self.save_model_structure_plot(model_name, layers, &output_path)
    }

    /// Save model structure plot
    fn save_model_structure_plot(
        &self,
        model_name: &str,
        layers: &[(String, usize)],
        path: &Path,
    ) -> Result<(), String> {
        use std::io::Write;
        let mut file = std::fs::File::create(path).map_err(|e| e.to_string())?;

        // Simple SVG plot
        let width = 800;
        let height = 600;
        let padding = 50;

        let max_neurons = layers.iter().map(|(_, size)| *size).max().unwrap_or(100);
        let layer_height = (height - 2 * padding) as f64 / layers.len() as f64;

        writeln!(
            file,
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#,
            width, height
        )
        .map_err(|e| e.to_string())?;

        // Draw model name
        writeln!(
            file,
            r#"<text x="{}" y="{}" text-anchor="middle" font-size="20">{}</text>"#,
            width / 2,
            padding / 2,
            model_name
        )
        .map_err(|e| e.to_string())?;

        // Draw layers
        for (i, (name, size)) in layers.iter().enumerate() {
            let layer_x = padding as f64
                + (i as f64 / (layers.len() - 1) as f64) * (width - 2 * padding) as f64;
            let layer_y = padding as f64 + i as f64 * layer_height;
            let layer_width = (*size as f64 / max_neurons as f64) * 200.0;

            // Draw layer rectangle
            writeln!(
                file,
                r#"<rect x="{}" y="{}" width="{}" height="{}" fill="lightblue" stroke="black" stroke-width="2"/>"#,
                layer_x - layer_width / 2.0,
                layer_y,
                layer_width,
                layer_height - 10.0
            )
            .map_err(|e| e.to_string())?;

            // Draw layer name
            writeln!(
                file,
                r#"<text x="{}" y="{}" text-anchor="middle" font-size="12">{}</text>"#,
                layer_x,
                layer_y + layer_height / 2.0,
                name
            )
            .map_err(|e| e.to_string())?;

            // Draw layer size
            writeln!(
                file,
                r#"<text x="{}" y="{}" text-anchor="middle" font-size="10">{}</text>"#,
                layer_x,
                layer_y + layer_height / 2.0 + 15.0,
                size
            )
            .map_err(|e| e.to_string())?;

            // Draw connections to next layer
            if i < layers.len() - 1 {
                let next_layer_x = padding as f64
                    + ((i + 1) as f64 / (layers.len() - 1) as f64) * (width - 2 * padding) as f64;
                let next_layer_y = padding as f64 + (i + 1) as f64 * layer_height;

                // Draw a few connection lines
                for j in 0..5 {
                    let start_y = layer_y + (j as f64 / 4.0) * (layer_height - 10.0);
                    let end_y = next_layer_y + (j as f64 / 4.0) * (layer_height - 10.0);

                    writeln!(
                        file,
                        r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="gray" stroke-width="1" opacity="0.5"/>"#,
                        layer_x + layer_width / 2.0,
                        start_y,
                        next_layer_x - layer_width / 2.0,
                        end_y
                    )
                    .map_err(|e| e.to_string())?;
                }
            }
        }

        writeln!(file, r#"</svg>"#).map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Analyze and visualize render results
    pub fn analyze_render_results(
        &self,
        mesh: &Mesh3D,
        render_time: f64,
        fps: f64,
    ) -> Result<(), String> {
        let output_path = Path::new(&self.output_dir).join("render_analysis.txt");
        let mut file = std::fs::File::create(output_path).map_err(|e| e.to_string())?;

        writeln!(file, "Render Analysis Report").map_err(|e| e.to_string())?;
        writeln!(file, "======================").map_err(|e| e.to_string())?;
        writeln!(file).map_err(|e| e.to_string())?;

        // Mesh statistics
        writeln!(file, "Mesh Statistics:").map_err(|e| e.to_string())?;
        writeln!(file, "- Vertices: {}", mesh.vertices.len()).map_err(|e| e.to_string())?;
        writeln!(file, "- Faces: {}", mesh.faces.len()).map_err(|e| e.to_string())?;

        // Calculate average face size
        let mut total_area = 0.0;
        for face in &mesh.faces {
            if face.vertices.len() >= 3 {
                let v0 = &mesh.vertices[face.vertices[0]].point;
                let v1 = &mesh.vertices[face.vertices[1]].point;
                let v2 = &mesh.vertices[face.vertices[2]].point;

                let a =
                    ((v1.x - v0.x).powi(2) + (v1.y - v0.y).powi(2) + (v1.z - v0.z).powi(2)).sqrt();
                let b =
                    ((v2.x - v1.x).powi(2) + (v2.y - v1.y).powi(2) + (v2.z - v1.z).powi(2)).sqrt();
                let c =
                    ((v0.x - v2.x).powi(2) + (v0.y - v2.y).powi(2) + (v0.z - v2.z).powi(2)).sqrt();

                let s = (a + b + c) / 2.0;
                let area = (s * (s - a) * (s - b) * (s - c)).sqrt();
                total_area += area;
            }
        }
        let avg_face_area = if mesh.faces.len() > 0 {
            total_area / mesh.faces.len() as f64
        } else {
            0.0
        };
        writeln!(file, "- Average face area: {:.6}", avg_face_area).map_err(|e| e.to_string())?;
        writeln!(file).map_err(|e| e.to_string())?;

        // Render performance
        writeln!(file, "Render Performance:").map_err(|e| e.to_string())?;
        writeln!(file, "- Render time: {:.2} ms", render_time).map_err(|e| e.to_string())?;
        writeln!(file, "- FPS: {:.2}", fps).map_err(|e| e.to_string())?;

        // Performance analysis
        writeln!(file).map_err(|e| e.to_string())?;
        writeln!(file, "Performance Analysis:").map_err(|e| e.to_string())?;
        if fps >= 60.0 {
            writeln!(file, "- Excellent performance: Can run at high frame rates")
                .map_err(|e| e.to_string())?;
        } else if fps >= 30.0 {
            writeln!(file, "- Good performance: Smooth interactive experience")
                .map_err(|e| e.to_string())?;
        } else if fps >= 15.0 {
            writeln!(
                file,
                "- Acceptable performance: Usable but may have some lag"
            )
            .map_err(|e| e.to_string())?;
        } else {
            writeln!(file, "- Poor performance: May need optimization")
                .map_err(|e| e.to_string())?;
        }

        // Optimization suggestions
        writeln!(file).map_err(|e| e.to_string())?;
        writeln!(file, "Optimization Suggestions:").map_err(|e| e.to_string())?;
        if mesh.vertices.len() > 100000 {
            writeln!(
                file,
                "- Consider mesh simplification to reduce vertex count"
            )
            .map_err(|e| e.to_string())?;
        }
        if avg_face_area < 0.001 {
            writeln!(file, "- Consider reducing mesh detail in flat areas")
                .map_err(|e| e.to_string())?;
        }
        if fps < 30.0 {
            writeln!(file, "- Consider implementing LOD (Level of Detail) system")
                .map_err(|e| e.to_string())?;
            writeln!(file, "- Check if GPU acceleration is properly enabled")
                .map_err(|e| e.to_string())?;
        }

        Ok(())
    }
}

/// Feature Visualization
pub struct FeatureVisualization {
    mesh: Mesh3D,
    features: Vec<String>,
    feature_positions: Vec<Point>,
}

impl FeatureVisualization {
    pub fn new(mesh: Mesh3D, features: Vec<String>) -> Self {
        // Calculate feature positions (simplified: use mesh center)
        let center = Self::calculate_mesh_center(&mesh);
        let feature_positions = vec![center; features.len()];

        Self {
            mesh,
            features,
            feature_positions,
        }
    }

    /// Calculate mesh center
    fn calculate_mesh_center(mesh: &Mesh3D) -> Point {
        if mesh.vertices.is_empty() {
            return Point::origin();
        }

        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_z = 0.0;

        for vertex in &mesh.vertices {
            sum_x += vertex.point.x;
            sum_y += vertex.point.y;
            sum_z += vertex.point.z;
        }

        let count = mesh.vertices.len() as f64;
        Point::new(sum_x / count, sum_y / count, sum_z / count)
    }

    /// Render visualization
    pub fn render(&self, output_path: &Path) -> Result<(), String> {
        use std::io::Write;
        let mut file = std::fs::File::create(output_path).map_err(|e| e.to_string())?;

        // Write vertices
        for vertex in &self.mesh.vertices {
            writeln!(
                file,
                "v {} {} {}",
                vertex.point.x, vertex.point.y, vertex.point.z
            )
            .map_err(|e| e.to_string())?;
        }

        // Write faces
        for face in &self.mesh.faces {
            write!(file, "f").map_err(|e| e.to_string())?;
            for &vertex_id in &face.vertices {
                write!(file, " {}", vertex_id + 1).map_err(|e| e.to_string())?;
            }
            writeln!(file).map_err(|e| e.to_string())?;
        }

        // Write feature markers
        for (_i, position) in self.feature_positions.iter().enumerate() {
            writeln!(file, "v {} {} {}", position.x, position.y, position.z)
                .map_err(|e| e.to_string())?;
        }

        // Write feature labels
        writeln!(file, "# Features:").map_err(|e| e.to_string())?;
        for (i, feature) in self.features.iter().enumerate() {
            writeln!(file, "# {}: {}", i, feature).map_err(|e| e.to_string())?;
        }

        Ok(())
    }
}
