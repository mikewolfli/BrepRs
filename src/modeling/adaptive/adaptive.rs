use crate::foundation::types::StandardReal;
use crate::geometry::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdaptationStrategy {
    QualityBased,
    PerformanceBased,
    Hybrid,
    Predictive,
}

#[derive(Debug, Clone)]
pub struct AdaptationContext {
    pub available_memory: usize,
    pub cpu_usage: StandardReal,
    pub gpu_usage: StandardReal,
    pub target_fps: StandardReal,
    pub current_fps: StandardReal,
    pub model_complexity: StandardReal,
    pub view_distance: StandardReal,
    pub screen_size: (u32, u32),
}

impl AdaptationContext {
    pub fn new() -> Self {
        Self {
            available_memory: 1024 * 1024 * 1024,
            cpu_usage: 0.0,
            gpu_usage: 0.0,
            target_fps: 60.0,
            current_fps: 60.0,
            model_complexity: 1.0,
            view_distance: 1.0,
            screen_size: (1920, 1080),
        }
    }

    pub fn with_memory(mut self, memory: usize) -> Self {
        self.available_memory = memory;
        self
    }

    pub fn with_fps(mut self, target: StandardReal, current: StandardReal) -> Self {
        self.target_fps = target;
        self.current_fps = current;
        self
    }

    pub fn with_complexity(mut self, complexity: StandardReal) -> Self {
        self.model_complexity = complexity;
        self
    }

    pub fn with_view_distance(mut self, distance: StandardReal) -> Self {
        self.view_distance = distance;
        self
    }

    pub fn performance_score(&self) -> StandardReal {
        if self.target_fps <= 0.0 {
            return 0.0;
        }
        (self.current_fps / self.target_fps).min(2.0)
    }

    pub fn needs_adaptation(&self) -> bool {
        self.performance_score() < 0.9 || self.cpu_usage > 0.9 || self.gpu_usage > 0.9
    }
}

impl Default for AdaptationContext {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct AdaptiveParameters {
    pub mesh_detail: StandardReal,
    pub texture_resolution: u32,
    pub anti_aliasing_level: u32,
    pub shadow_quality: StandardReal,
    pub reflection_quality: StandardReal,
    pub lod_bias: StandardReal,
    pub tessellation_factor: StandardReal,
}

impl AdaptiveParameters {
    pub fn new() -> Self {
        Self {
            mesh_detail: 1.0,
            texture_resolution: 1024,
            anti_aliasing_level: 4,
            shadow_quality: 1.0,
            reflection_quality: 1.0,
            lod_bias: 1.0,
            tessellation_factor: 1.0,
        }
    }

    pub fn high_quality() -> Self {
        Self {
            mesh_detail: 1.0,
            texture_resolution: 4096,
            anti_aliasing_level: 8,
            shadow_quality: 1.0,
            reflection_quality: 1.0,
            lod_bias: 1.0,
            tessellation_factor: 1.0,
        }
    }

    pub fn balanced() -> Self {
        Self {
            mesh_detail: 0.75,
            texture_resolution: 2048,
            anti_aliasing_level: 4,
            shadow_quality: 0.75,
            reflection_quality: 0.75,
            lod_bias: 1.0,
            tessellation_factor: 0.75,
        }
    }

    pub fn performance() -> Self {
        Self {
            mesh_detail: 0.5,
            texture_resolution: 1024,
            anti_aliasing_level: 2,
            shadow_quality: 0.5,
            reflection_quality: 0.25,
            lod_bias: 1.5,
            tessellation_factor: 0.5,
        }
    }

    pub fn low_end() -> Self {
        Self {
            mesh_detail: 0.25,
            texture_resolution: 512,
            anti_aliasing_level: 0,
            shadow_quality: 0.0,
            reflection_quality: 0.0,
            lod_bias: 2.0,
            tessellation_factor: 0.25,
        }
    }
}

impl Default for AdaptiveParameters {
    fn default() -> Self {
        Self::new()
    }
}

pub struct AdaptiveController {
    strategy: AdaptationStrategy,
    current_params: AdaptiveParameters,
    history: Vec<AdaptationContext>,
    max_history: usize,
    adaptation_threshold: StandardReal,
    stability_window: usize,
}

impl AdaptiveController {
    pub fn new(strategy: AdaptationStrategy) -> Self {
        Self {
            strategy,
            current_params: AdaptiveParameters::new(),
            history: Vec::new(),
            max_history: 100,
            adaptation_threshold: 0.1,
            stability_window: 10,
        }
    }

    pub fn update(&mut self, context: &AdaptationContext) -> AdaptiveParameters {
        self.history.push(context.clone());
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }

        if !self.should_adapt(context) {
            return self.current_params.clone();
        }

        self.adapt(context)
    }

    fn should_adapt(&self, context: &AdaptationContext) -> bool {
        if self.history.len() < self.stability_window {
            return false;
        }

        let recent: Vec<_> = self.history.iter().rev().take(self.stability_window).collect();
        let avg_score: StandardReal = recent.iter().map(|c| c.performance_score()).sum::<StandardReal>()
            / recent.len() as StandardReal;

        (context.performance_score() - avg_score).abs() > self.adaptation_threshold
    }

    fn adapt(&mut self, context: &AdaptationContext) -> AdaptiveParameters {
        match self.strategy {
            AdaptationStrategy::QualityBased => self.adapt_quality_based(context),
            AdaptationStrategy::PerformanceBased => self.adapt_performance_based(context),
            AdaptationStrategy::Hybrid => self.adapt_hybrid(context),
            AdaptationStrategy::Predictive => self.adapt_predictive(context),
        }
    }

    fn adapt_quality_based(&mut self, context: &AdaptationContext) -> AdaptiveParameters {
        let score = context.performance_score();

        if score >= 1.0 {
            self.current_params = AdaptiveParameters::high_quality();
        } else if score >= 0.8 {
            self.current_params = AdaptiveParameters::balanced();
        } else if score >= 0.6 {
            self.current_params = AdaptiveParameters::performance();
        } else {
            self.current_params = AdaptiveParameters::low_end();
        }

        self.current_params.clone()
    }

    fn adapt_performance_based(&mut self, context: &AdaptationContext) -> AdaptiveParameters {
        let mut params = self.current_params.clone();

        if context.cpu_usage > 0.9 {
            params.mesh_detail *= 0.8;
            params.tessellation_factor *= 0.8;
        }

        if context.gpu_usage > 0.9 {
            params.texture_resolution = (params.texture_resolution as StandardReal * 0.75) as u32;
            params.shadow_quality *= 0.8;
            params.reflection_quality *= 0.8;
        }

        if context.current_fps < context.target_fps * 0.5 {
            params.lod_bias *= 1.5;
            params.anti_aliasing_level = params.anti_aliasing_level.saturating_sub(2);
        }

        self.current_params = params;
        self.current_params.clone()
    }

    fn adapt_hybrid(&mut self, context: &AdaptationContext) -> AdaptiveParameters {
        let score = context.performance_score();
        let mut params = self.current_params.clone();

        if score >= 0.9 {
            params.mesh_detail = 1.0;
            params.texture_resolution = 2048;
        } else if score >= 0.7 {
            params.mesh_detail = 0.75;
            params.texture_resolution = 1024;
        } else {
            params.mesh_detail = 0.5;
            params.texture_resolution = 512;
        }

        if context.cpu_usage > 0.8 {
            params.mesh_detail *= 0.9;
        }

        if context.gpu_usage > 0.8 {
            params.shadow_quality *= 0.9;
            params.reflection_quality *= 0.9;
        }

        params.mesh_detail = params.mesh_detail.clamp(0.1, 1.0);
        params.texture_resolution = params.texture_resolution.clamp(256, 4096);

        self.current_params = params;
        self.current_params.clone()
    }

    fn adapt_predictive(&mut self, context: &AdaptationContext) -> AdaptiveParameters {
        if self.history.len() < 5 {
            return self.adapt_hybrid(context);
        }

        let trend = self.predict_performance_trend();
        let mut params = self.current_params.clone();

        if trend < -0.1 {
            params.mesh_detail *= 0.9;
            params.texture_resolution = (params.texture_resolution as StandardReal * 0.9) as u32;
            params.lod_bias *= 1.1;
        } else if trend > 0.1 {
            params.mesh_detail = (params.mesh_detail * 1.05).min(1.0);
            params.texture_resolution = ((params.texture_resolution as StandardReal * 1.05) as u32).min(4096);
            params.lod_bias = (params.lod_bias * 0.95).max(0.5);
        }

        if context.performance_score() < 0.7 {
            params = AdaptiveParameters::performance();
        }

        self.current_params = params;
        self.current_params.clone()
    }

    fn predict_performance_trend(&self) -> StandardReal {
        if self.history.len() < 5 {
            return 0.0;
        }

        let recent: Vec<_> = self.history.iter().rev().take(5).collect();
        let scores: Vec<StandardReal> = recent.iter().map(|c| c.performance_score()).collect();

        let n = scores.len();
        let sum_x: StandardReal = (0..n).map(|i| i as StandardReal).sum();
        let sum_y: StandardReal = scores.iter().sum();
        let sum_xy: StandardReal = scores
            .iter()
            .enumerate()
            .map(|(i, y)| i as StandardReal * y)
            .sum();
        let sum_xx: StandardReal = (0..n).map(|i| (i * i) as StandardReal).sum();

        let denominator = n as StandardReal * sum_xx - sum_x * sum_x;
        if denominator.abs() < 1e-10 {
            return 0.0;
        }

        (n as StandardReal * sum_xy - sum_x * sum_y) / denominator
    }

    pub fn current_params(&self) -> &AdaptiveParameters {
        &self.current_params
    }

    pub fn set_strategy(&mut self, strategy: AdaptationStrategy) {
        self.strategy = strategy;
    }

    pub fn strategy(&self) -> AdaptationStrategy {
        self.strategy
    }
}

impl Default for AdaptiveController {
    fn default() -> Self {
        Self::new(AdaptationStrategy::Hybrid)
    }
}

pub struct AdaptiveTessellator {
    base_tolerance: StandardReal,
    max_triangles: usize,
    min_triangles: usize,
    curvature_weight: StandardReal,
    distance_weight: StandardReal,
}

impl AdaptiveTessellator {
    pub fn new() -> Self {
        Self {
            base_tolerance: 0.01,
            max_triangles: 1_000_000,
            min_triangles: 100,
            curvature_weight: 0.5,
            distance_weight: 0.5,
        }
    }

    pub fn with_tolerance(mut self, tolerance: StandardReal) -> Self {
        self.base_tolerance = tolerance;
        self
    }

    pub fn with_triangle_limits(mut self, min: usize, max: usize) -> Self {
        self.min_triangles = min;
        self.max_triangles = max;
        self
    }

    pub fn compute_adaptive_tolerance(&self, point: &Point, camera: &Point, curvature: StandardReal) -> StandardReal {
        let distance = ((point.x - camera.x).powi(2)
            + (point.y - camera.y).powi(2)
            + (point.z - camera.z).powi(2)).sqrt();

        let distance_factor = 1.0 + distance * self.distance_weight;
        let curvature_factor = 1.0 / (1.0 + curvature * self.curvature_weight);

        self.base_tolerance * distance_factor * curvature_factor
    }

    pub fn estimate_triangle_count(&self, surface_area: StandardReal, avg_tolerance: StandardReal) -> usize {
        if avg_tolerance <= 0.0 {
            return self.min_triangles;
        }

        let estimated = (surface_area / (avg_tolerance * avg_tolerance)) as usize;
        estimated.clamp(self.min_triangles, self.max_triangles)
    }

    pub fn should_subdivide(&self, current_error: StandardReal, tolerance: StandardReal) -> bool {
        current_error > tolerance
    }
}

impl Default for AdaptiveTessellator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptation_context() {
        let context = AdaptationContext::new()
            .with_fps(60.0, 30.0)
            .with_complexity(0.5);

        assert!(context.needs_adaptation());
        assert!(context.performance_score() < 1.0);
    }

    #[test]
    fn test_adaptive_parameters() {
        let high = AdaptiveParameters::high_quality();
        assert_eq!(high.texture_resolution, 4096);

        let low = AdaptiveParameters::low_end();
        assert_eq!(low.texture_resolution, 512);
    }

    #[test]
    fn test_adaptive_controller() {
        let mut controller = AdaptiveController::new(AdaptationStrategy::Hybrid);
        
        let context = AdaptationContext::new().with_fps(60.0, 60.0);
        for _ in 0..15 {
            controller.update(&context);
        }

        let params = controller.current_params();
        assert!(params.mesh_detail > 0.0);
    }

    #[test]
    fn test_adaptive_tessellator() {
        let tessellator = AdaptiveTessellator::new()
            .with_tolerance(0.001)
            .with_triangle_limits(100, 10000);

        let point = Point::new(0.0, 0.0, 0.0);
        let camera = Point::new(10.0, 0.0, 0.0);

        let tolerance = tessellator.compute_adaptive_tolerance(&point, &camera, 0.1);
        assert!(tolerance > 0.0);

        let count = tessellator.estimate_triangle_count(100.0, 0.01);
        assert!(count >= 100 && count <= 10000);
    }
}
