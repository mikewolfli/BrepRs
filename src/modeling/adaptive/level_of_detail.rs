use crate::foundation::types::StandardReal;
use crate::geometry::Point;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LodLevel(pub u32);

impl Default for LodLevel {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LodStrategy {
    DistanceBased,
    ScreenSizeBased,
    Hybrid,
    Custom,
}

#[derive(Debug, Clone)]
pub struct LodLevelData {
    pub level: LodLevel,
    pub distance_range: (StandardReal, StandardReal),
    pub screen_size_range: (StandardReal, StandardReal),
    pub triangle_count: usize,
    pub vertex_count: usize,
    pub mesh_data: Option<Vec<u8>>,
}

impl LodLevelData {
    pub fn new(level: LodLevel) -> Self {
        Self {
            level,
            distance_range: (0.0, StandardReal::MAX),
            screen_size_range: (0.0, 1.0),
            triangle_count: 0,
            vertex_count: 0,
            mesh_data: None,
        }
    }

    pub fn with_distance_range(mut self, min: StandardReal, max: StandardReal) -> Self {
        self.distance_range = (min, max);
        self
    }

    pub fn with_screen_size_range(mut self, min: StandardReal, max: StandardReal) -> Self {
        self.screen_size_range = (min, max);
        self
    }

    pub fn with_mesh_stats(mut self, triangles: usize, vertices: usize) -> Self {
        self.triangle_count = triangles;
        self.vertex_count = vertices;
        self
    }

    pub fn is_applicable_for_distance(&self, distance: StandardReal) -> bool {
        distance >= self.distance_range.0 && distance < self.distance_range.1
    }

    pub fn is_applicable_for_screen_size(&self, screen_size: StandardReal) -> bool {
        screen_size >= self.screen_size_range.0 && screen_size <= self.screen_size_range.1
    }
}

#[derive(Debug, Clone)]
pub struct LodParameters {
    pub strategy: LodStrategy,
    pub bias: StandardReal,
    pub transition_speed: StandardReal,
    pub max_lod_levels: u32,
    pub min_distance: StandardReal,
    pub max_distance: StandardReal,
    pub lod_distances: Vec<StandardReal>,
}

impl Default for LodParameters {
    fn default() -> Self {
        Self {
            strategy: LodStrategy::DistanceBased,
            bias: 1.0,
            transition_speed: 1.0,
            max_lod_levels: 5,
            min_distance: 0.1,
            max_distance: 1000.0,
            lod_distances: vec![10.0, 25.0, 50.0, 100.0, 200.0],
        }
    }
}

impl LodParameters {
    pub fn new(strategy: LodStrategy) -> Self {
        Self {
            strategy,
            ..Default::default()
        }
    }

    pub fn with_bias(mut self, bias: StandardReal) -> Self {
        self.bias = bias;
        self
    }

    pub fn with_distance_range(mut self, min: StandardReal, max: StandardReal) -> Self {
        self.min_distance = min;
        self.max_distance = max;
        self
    }

    pub fn with_lod_distances(mut self, distances: Vec<StandardReal>) -> Self {
        self.lod_distances = distances;
        self.max_lod_levels = self.lod_distances.len() as u32;
        self
    }
}

pub struct LodSelector {
    parameters: LodParameters,
    levels: HashMap<LodLevel, LodLevelData>,
    current_level: LodLevel,
    transition_progress: StandardReal,
}

impl LodSelector {
    pub fn new(parameters: LodParameters) -> Self {
        Self {
            parameters,
            levels: HashMap::new(),
            current_level: LodLevel(0),
            transition_progress: 0.0,
        }
    }

    pub fn add_level(&mut self, level: LodLevelData) {
        self.levels.insert(level.level, level);
    }

    pub fn select_lod(&mut self, camera_position: &Point, object_position: &Point, screen_size: StandardReal) -> LodLevel {
        let distance = ((object_position.x - camera_position.x).powi(2)
            + (object_position.y - camera_position.y).powi(2)
            + (object_position.z - camera_position.z).powi(2)).sqrt();

        let adjusted_distance = distance * self.parameters.bias;

        let new_level = match self.parameters.strategy {
            LodStrategy::DistanceBased => self.select_by_distance(adjusted_distance),
            LodStrategy::ScreenSizeBased => self.select_by_screen_size(screen_size),
            LodStrategy::Hybrid => self.select_hybrid(adjusted_distance, screen_size),
            LodStrategy::Custom => self.select_custom(adjusted_distance, screen_size),
        };

        if new_level != self.current_level {
            self.transition_progress = 0.0;
            self.current_level = new_level;
        }

        self.current_level
    }

    fn select_by_distance(&self, distance: StandardReal) -> LodLevel {
        for (level, data) in &self.levels {
            if data.is_applicable_for_distance(distance) {
                return *level;
            }
        }

        for (i, &lod_distance) in self.parameters.lod_distances.iter().enumerate() {
            if distance < lod_distance {
                return LodLevel(i as u32);
            }
        }

        LodLevel(self.parameters.max_lod_levels - 1)
    }

    fn select_by_screen_size(&self, screen_size: StandardReal) -> LodLevel {
        for (level, data) in &self.levels {
            if data.is_applicable_for_screen_size(screen_size) {
                return *level;
            }
        }

        if screen_size > 0.5 {
            LodLevel(0)
        } else if screen_size > 0.25 {
            LodLevel(1)
        } else if screen_size > 0.1 {
            LodLevel(2)
        } else if screen_size > 0.05 {
            LodLevel(3)
        } else {
            LodLevel(4)
        }
    }

    fn select_hybrid(&self, distance: StandardReal, screen_size: StandardReal) -> LodLevel {
        let distance_lod = self.select_by_distance(distance);
        let screen_lod = self.select_by_screen_size(screen_size);

        let distance_weight = 0.6;
        let screen_weight = 0.4;

        let weighted_level = distance_lod.0 as StandardReal * distance_weight
            + screen_lod.0 as StandardReal * screen_weight;

        LodLevel(weighted_level.round() as u32)
    }

    fn select_custom(&self, distance: StandardReal, screen_size: StandardReal) -> LodLevel {
        self.select_hybrid(distance, screen_size)
    }

    pub fn current_level(&self) -> LodLevel {
        self.current_level
    }

    pub fn transition_progress(&self) -> StandardReal {
        self.transition_progress
    }

    pub fn update_transition(&mut self, delta_time: StandardReal) {
        if self.transition_progress < 1.0 {
            self.transition_progress += delta_time * self.parameters.transition_speed;
            self.transition_progress = self.transition_progress.min(1.0);
        }
    }

    pub fn get_level_data(&self, level: LodLevel) -> Option<&LodLevelData> {
        self.levels.get(&level)
    }

    pub fn level_count(&self) -> usize {
        self.levels.len()
    }
}

pub struct LodManager {
    selectors: HashMap<String, LodSelector>,
    global_parameters: LodParameters,
}

impl LodManager {
    pub fn new() -> Self {
        Self {
            selectors: HashMap::new(),
            global_parameters: LodParameters::default(),
        }
    }

    pub fn with_parameters(mut self, parameters: LodParameters) -> Self {
        self.global_parameters = parameters;
        self
    }

    pub fn register_object(&mut self, id: &str, selector: LodSelector) {
        self.selectors.insert(id.to_string(), selector);
    }

    pub fn unregister_object(&mut self, id: &str) {
        self.selectors.remove(id);
    }

    pub fn update(&mut self, camera_position: &Point, objects: &[(String, Point, StandardReal)]) {
        for (id, position, screen_size) in objects {
            if let Some(selector) = self.selectors.get_mut(id) {
                selector.select_lod(camera_position, position, *screen_size);
            }
        }
    }

    pub fn get_lod(&self, id: &str) -> Option<LodLevel> {
        self.selectors.get(id).map(|s| s.current_level())
    }

    pub fn get_transition_progress(&self, id: &str) -> Option<StandardReal> {
        self.selectors.get(id).map(|s| s.transition_progress())
    }

    pub fn object_count(&self) -> usize {
        self.selectors.len()
    }

    pub fn set_global_bias(&mut self, bias: StandardReal) {
        self.global_parameters.bias = bias;
        for selector in self.selectors.values_mut() {
            selector.parameters.bias = bias;
        }
    }
}

impl Default for LodManager {
    fn default() -> Self {
        Self::new()
    }
}

pub struct LodStatistics {
    pub objects_at_lod: Vec<usize>,
    pub average_lod: StandardReal,
    pub max_lod_used: u32,
    pub min_lod_used: u32,
    pub total_triangles: usize,
    pub total_vertices: usize,
}

impl LodStatistics {
    pub fn new(max_levels: u32) -> Self {
        Self {
            objects_at_lod: vec![0; max_levels as usize],
            average_lod: 0.0,
            max_lod_used: 0,
            min_lod_used: max_levels - 1,
            total_triangles: 0,
            total_vertices: 0,
        }
    }

    pub fn update(&mut self, manager: &LodManager) {
        let mut total_objects = 0;
        let mut lod_sum = 0.0;

        for (_, selector) in &manager.selectors {
            let level = selector.current_level().0 as usize;
            if level < self.objects_at_lod.len() {
                self.objects_at_lod[level] += 1;
            }
            total_objects += 1;
            lod_sum += level as StandardReal;

            self.max_lod_used = self.max_lod_used.max(selector.current_level().0);
            self.min_lod_used = self.min_lod_used.min(selector.current_level().0);

            if let Some(data) = selector.get_level_data(selector.current_level()) {
                self.total_triangles += data.triangle_count;
                self.total_vertices += data.vertex_count;
            }
        }

        if total_objects > 0 {
            self.average_lod = lod_sum / total_objects as StandardReal;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lod_level_data() {
        let data = LodLevelData::new(LodLevel(0))
            .with_distance_range(0.0, 10.0)
            .with_screen_size_range(0.5, 1.0);

        assert!(data.is_applicable_for_distance(5.0));
        assert!(!data.is_applicable_for_distance(15.0));
        assert!(data.is_applicable_for_screen_size(0.75));
    }

    #[test]
    fn test_lod_parameters() {
        let params = LodParameters::new(LodStrategy::DistanceBased)
            .with_bias(1.5)
            .with_lod_distances(vec![10.0, 20.0, 40.0]);

        assert_eq!(params.strategy, LodStrategy::DistanceBased);
        assert_eq!(params.bias, 1.5);
        assert_eq!(params.lod_distances.len(), 3);
    }

    #[test]
    fn test_lod_selector() {
        let params = LodParameters::new(LodStrategy::DistanceBased)
            .with_lod_distances(vec![10.0, 25.0, 50.0, 100.0]);

        let mut selector = LodSelector::new(params);

        selector.add_level(LodLevelData::new(LodLevel(0)).with_distance_range(0.0, 10.0));
        selector.add_level(LodLevelData::new(LodLevel(1)).with_distance_range(10.0, 25.0));
        selector.add_level(LodLevelData::new(LodLevel(2)).with_distance_range(25.0, 50.0));

        let camera = Point::new(0.0, 0.0, 0.0);
        let object = Point::new(15.0, 0.0, 0.0);

        let level = selector.select_lod(&camera, &object, 0.5);
        assert_eq!(level, LodLevel(1));
    }

    #[test]
    fn test_lod_manager() {
        let mut manager = LodManager::new();

        let params = LodParameters::new(LodStrategy::DistanceBased);
        let selector = LodSelector::new(params);

        manager.register_object("test_object", selector);

        assert_eq!(manager.object_count(), 1);

        manager.unregister_object("test_object");
        assert_eq!(manager.object_count(), 0);
    }

    #[test]
    fn test_lod_statistics() {
        let mut stats = LodStatistics::new(5);
        let manager = LodManager::new();

        stats.update(&manager);

        assert_eq!(stats.total_triangles, 0);
        assert_eq!(stats.total_vertices, 0);
    }
}
