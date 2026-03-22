use std::collections::HashMap;
use crate::foundation::types::StandardReal;
use crate::geometry::{Point, Vector, Plane};
use crate::topology::topods_shape::TopoDsShape;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FeatureId(pub u64);

impl FeatureId {
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}

impl Default for FeatureId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatureType {
    BaseFeature,
    Sketch,
    Extrude,
    Revolve,
    Sweep,
    Loft,
    Fillet,
    Chamfer,
    Shell,
    Draft,
    Hole,
    Mirror,
    Pattern,
    Boolean,
    DeleteFace,
    ReplaceFace,
    MoveFace,
    Scale,
    Transform,
    Custom,
}

#[derive(Debug, Clone)]
pub struct FeatureParameter {
    pub name: String,
    pub value: String,
    pub param_type: ParameterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterType {
    Real,
    Integer,
    Boolean,
    String,
    Point,
    Vector,
    Plane,
    Shape,
    Reference,
}

#[derive(Debug, Clone)]
pub struct Feature {
    pub id: FeatureId,
    pub feature_type: FeatureType,
    pub name: String,
    pub parameters: HashMap<String, FeatureParameter>,
    pub parent_features: Vec<FeatureId>,
    pub child_features: Vec<FeatureId>,
    pub suppressed: bool,
    pub result_shape: Option<TopoDsShape>,
}

impl Feature {
    pub fn new(feature_type: FeatureType, name: String) -> Self {
        Self {
            id: FeatureId::new(),
            feature_type,
            name,
            parameters: HashMap::new(),
            parent_features: Vec::new(),
            child_features: Vec::new(),
            suppressed: false,
            result_shape: None,
        }
    }

    pub fn with_parameter(mut self, name: &str, value: &str, param_type: ParameterType) -> Self {
        self.parameters.insert(
            name.to_string(),
            FeatureParameter {
                name: name.to_string(),
                value: value.to_string(),
                param_type,
            },
        );
        self
    }

    pub fn add_parent(&mut self, parent_id: FeatureId) {
        if !self.parent_features.contains(&parent_id) {
            self.parent_features.push(parent_id);
        }
    }

    pub fn add_child(&mut self, child_id: FeatureId) {
        if !self.child_features.contains(&child_id) {
            self.child_features.push(child_id);
        }
    }

    pub fn suppress(&mut self) {
        self.suppressed = true;
    }

    pub fn unsuppress(&mut self) {
        self.suppressed = false;
    }

    pub fn set_result_shape(&mut self, shape: TopoDsShape) {
        self.result_shape = Some(shape);
    }

    pub fn get_parameter(&self, name: &str) -> Option<&FeatureParameter> {
        self.parameters.get(name)
    }

    pub fn update_parameter(&mut self, name: &str, value: &str) -> Result<(), String> {
        if let Some(param) = self.parameters.get_mut(name) {
            param.value = value.to_string();
            Ok(())
        } else {
            Err(format!("Parameter '{}' not found", name))
        }
    }
}

pub struct FeatureHistory {
    features: HashMap<FeatureId, Feature>,
    root_features: Vec<FeatureId>,
    current_feature: Option<FeatureId>,
    dirty_features: Vec<FeatureId>,
}

impl FeatureHistory {
    pub fn new() -> Self {
        Self {
            features: HashMap::new(),
            root_features: Vec::new(),
            current_feature: None,
            dirty_features: Vec::new(),
        }
    }

    pub fn add_feature(&mut self, mut feature: Feature) -> FeatureId {
        let id = feature.id;

        if let Some(current) = self.current_feature {
            feature.add_parent(current);
            if let Some(parent) = self.features.get_mut(&current) {
                parent.add_child(id);
            }
        } else {
            self.root_features.push(id);
        }

        self.features.insert(id, feature);
        self.current_feature = Some(id);
        self.dirty_features.push(id);

        id
    }

    pub fn insert_feature_after(&mut self, feature: Feature, after_id: FeatureId) -> Result<FeatureId, String> {
        if !self.features.contains_key(&after_id) {
            return Err("Reference feature not found".to_string());
        }

        let id = feature.id;
        let mut new_feature = feature;

        new_feature.parent_features = vec![after_id];

        if let Some(after_feature) = self.features.get_mut(&after_id) {
            new_feature.child_features = after_feature.child_features.clone();
            after_feature.child_features = vec![id];

            for &child_id in &new_feature.child_features {
                if let Some(child) = self.features.get_mut(&child_id) {
                    child.parent_features.retain(|&p| p != after_id);
                    child.parent_features.push(id);
                    self.dirty_features.push(child_id);
                }
            }
        }

        self.features.insert(id, new_feature);
        self.dirty_features.push(id);

        Ok(id)
    }

    pub fn remove_feature(&mut self, feature_id: FeatureId) -> Result<Feature, String> {
        let feature = self.features.remove(&feature_id)
            .ok_or("Feature not found")?;

        for &parent_id in &feature.parent_features {
            if let Some(parent) = self.features.get_mut(&parent_id) {
                parent.child_features.retain(|&c| c != feature_id);
            }
        }

        for &child_id in &feature.child_features {
            if let Some(child) = self.features.get_mut(&child_id) {
                child.parent_features.retain(|&p| p != feature_id);
                if child.parent_features.is_empty() {
                    self.root_features.push(child_id);
                }
                self.dirty_features.push(child_id);
            }
        }

        if self.current_feature == Some(feature_id) {
            self.current_feature = feature.parent_features.first().copied();
        }

        self.root_features.retain(|&r| r != feature_id);

        Ok(feature)
    }

    pub fn get_feature(&self, id: FeatureId) -> Option<&Feature> {
        self.features.get(&id)
    }

    pub fn get_feature_mut(&mut self, id: FeatureId) -> Option<&mut Feature> {
        self.features.get_mut(&id)
    }

    pub fn features(&self) -> &HashMap<FeatureId, Feature> {
        &self.features
    }

    pub fn root_features(&self) -> &[FeatureId] {
        &self.root_features
    }

    pub fn current_feature(&self) -> Option<FeatureId> {
        self.current_feature
    }

    pub fn set_current_feature(&mut self, id: FeatureId) -> Result<(), String> {
        if self.features.contains_key(&id) {
            self.current_feature = Some(id);
            Ok(())
        } else {
            Err("Feature not found".to_string())
        }
    }

    pub fn update_feature_parameter(&mut self, feature_id: FeatureId, param_name: &str, value: &str) -> Result<(), String> {
        if let Some(feature) = self.features.get_mut(&feature_id) {
            feature.update_parameter(param_name, value)?;
            self.mark_dependent_features_dirty(feature_id);
            Ok(())
        } else {
            Err("Feature not found".to_string())
        }
    }

    fn mark_dependent_features_dirty(&mut self, feature_id: FeatureId) {
        let child_features: Vec<FeatureId> = self.features
            .get(&feature_id)
            .map(|f| f.child_features.clone())
            .unwrap_or_default();

        for child_id in child_features {
            if !self.dirty_features.contains(&child_id) {
                self.dirty_features.push(child_id);
                self.mark_dependent_features_dirty(child_id);
            }
        }
    }

    pub fn get_dirty_features(&self) -> &[FeatureId] {
        &self.dirty_features
    }

    pub fn clear_dirty_flags(&mut self) {
        self.dirty_features.clear();
    }

    pub fn regenerate(&mut self) -> Result<Vec<FeatureId>, String> {
        let dirty: Vec<FeatureId> = self.dirty_features.clone();
        self.clear_dirty_flags();
        Ok(dirty)
    }

    pub fn get_feature_tree(&self) -> Vec<(FeatureId, usize)> {
        let mut result = Vec::new();

        for &root_id in &self.root_features {
            self.traverse_feature_tree(root_id, 0, &mut result);
        }

        result
    }

    fn traverse_feature_tree(&self, feature_id: FeatureId, depth: usize, result: &mut Vec<(FeatureId, usize)>) {
        if let Some(feature) = self.features.get(&feature_id) {
            result.push((feature_id, depth));
            for &child_id in &feature.child_features {
                self.traverse_feature_tree(child_id, depth + 1, result);
            }
        }
    }

    pub fn get_final_shape(&self) -> Option<TopoDsShape> {
        let mut final_shape: Option<TopoDsShape> = None;

        for &root_id in &self.root_features {
            if let Some(shape) = self.compute_feature_shape(root_id) {
                final_shape = Some(shape);
            }
        }

        final_shape
    }

    fn compute_feature_shape(&self, feature_id: FeatureId) -> Option<TopoDsShape> {
        self.features.get(&feature_id).and_then(|f| f.result_shape.clone())
    }

    pub fn find_features_by_type(&self, feature_type: FeatureType) -> Vec<FeatureId> {
        self.features
            .iter()
            .filter(|(_, f)| f.feature_type == feature_type && !f.suppressed)
            .map(|(id, _)| *id)
            .collect()
    }

    pub fn find_features_by_name(&self, name_pattern: &str) -> Vec<FeatureId> {
        self.features
            .iter()
            .filter(|(_, f)| f.name.contains(name_pattern))
            .map(|(id, _)| *id)
            .collect()
    }

    pub fn suppress_feature(&mut self, feature_id: FeatureId) -> Result<(), String> {
        if let Some(feature) = self.features.get_mut(&feature_id) {
            feature.suppress();
            self.mark_dependent_features_dirty(feature_id);
            Ok(())
        } else {
            Err("Feature not found".to_string())
        }
    }

    pub fn unsuppress_feature(&mut self, feature_id: FeatureId) -> Result<(), String> {
        if let Some(feature) = self.features.get_mut(&feature_id) {
            feature.unsuppress();
            self.mark_dependent_features_dirty(feature_id);
            Ok(())
        } else {
            Err("Feature not found".to_string())
        }
    }

    pub fn roll_back_to(&mut self, feature_id: FeatureId) -> Result<(), String> {
        if !self.features.contains_key(&feature_id) {
            return Err("Feature not found".to_string());
        }

        self.current_feature = Some(feature_id);

        let features_to_remove: Vec<FeatureId> = self.features
            .values()
            .filter(|f| {
                f.parent_features.contains(&feature_id) &&
                !self.is_ancestor(feature_id, f.id)
            })
            .map(|f| f.id)
            .collect();

        for id in features_to_remove {
            let _ = self.remove_feature(id);
        }

        Ok(())
    }

    fn is_ancestor(&self, potential_ancestor: FeatureId, feature_id: FeatureId) -> bool {
        if let Some(feature) = self.features.get(&feature_id) {
            for &parent_id in &feature.parent_features {
                if parent_id == potential_ancestor {
                    return true;
                }
                if self.is_ancestor(potential_ancestor, parent_id) {
                    return true;
                }
            }
        }
        false
    }

    pub fn reorder_feature(&mut self, feature_id: FeatureId, new_parent_id: FeatureId) -> Result<(), String> {
        if feature_id == new_parent_id {
            return Err("Cannot move feature before itself".to_string());
        }

        if self.is_ancestor(feature_id, new_parent_id) {
            return Err("Cannot create circular dependency".to_string());
        }

        let mut feature = self.features.remove(&feature_id)
            .ok_or("Feature not found")?;

        for &old_parent_id in &feature.parent_features {
            if let Some(old_parent) = self.features.get_mut(&old_parent_id) {
                old_parent.child_features.retain(|&c| c != feature_id);
            }
        }

        feature.parent_features = vec![new_parent_id];

        if let Some(new_parent) = self.features.get_mut(&new_parent_id) {
            new_parent.child_features.push(feature_id);
        }

        self.features.insert(feature_id, feature);
        self.mark_dependent_features_dirty(feature_id);

        Ok(())
    }

    pub fn export_history(&self) -> FeatureHistoryData {
        FeatureHistoryData {
            features: self.features.values().cloned().collect(),
            root_features: self.root_features.clone(),
        }
    }

    pub fn import_history(&mut self, data: FeatureHistoryData) {
        self.features.clear();
        self.root_features = data.root_features;

        for feature in data.features {
            self.features.insert(feature.id, feature);
        }

        self.current_feature = self.root_features.first().copied();
        self.dirty_features = self.features.keys().copied().collect();
    }
}

impl Default for FeatureHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct FeatureHistoryData {
    pub features: Vec<Feature>,
    pub root_features: Vec<FeatureId>,
}

pub struct FeatureBuilder {
    feature_type: FeatureType,
    name: String,
    parameters: HashMap<String, FeatureParameter>,
}

impl FeatureBuilder {
    pub fn new(feature_type: FeatureType, name: &str) -> Self {
        Self {
            feature_type,
            name: name.to_string(),
            parameters: HashMap::new(),
        }
    }

    pub fn parameter(mut self, name: &str, value: &str, param_type: ParameterType) -> Self {
        self.parameters.insert(
            name.to_string(),
            FeatureParameter {
                name: name.to_string(),
                value: value.to_string(),
                param_type,
            },
        );
        self
    }

    pub fn real_parameter(self, name: &str, value: StandardReal) -> Self {
        self.parameter(name, &value.to_string(), ParameterType::Real)
    }

    pub fn integer_parameter(self, name: &str, value: i32) -> Self {
        self.parameter(name, &value.to_string(), ParameterType::Integer)
    }

    pub fn boolean_parameter(self, name: &str, value: bool) -> Self {
        self.parameter(name, &value.to_string(), ParameterType::Boolean)
    }

    pub fn string_parameter(self, name: &str, value: &str) -> Self {
        self.parameter(name, value, ParameterType::String)
    }

    pub fn point_parameter(self, name: &str, value: &Point) -> Self {
        self.parameter(name, &format!("{},{},{}", value.x, value.y, value.z), ParameterType::Point)
    }

    pub fn vector_parameter(self, name: &str, value: &Vector) -> Self {
        self.parameter(name, &format!("{},{},{}", value.x, value.y, value.z), ParameterType::Vector)
    }

    pub fn plane_parameter(self, name: &str, value: &Plane) -> Self {
        let loc = value.location();
        let dir = value.direction();
        self.parameter(name, &format!("{},{},{},{},{},{}", loc.x, loc.y, loc.z, dir.x, dir.y, dir.z), ParameterType::Plane)
    }

    pub fn build(self) -> Feature {
        let mut feature = Feature::new(self.feature_type, self.name);
        feature.parameters = self.parameters;
        feature
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_creation() {
        let feature = Feature::new(FeatureType::Extrude, "Extrude1".to_string())
            .with_parameter("distance", "10.0", ParameterType::Real);

        assert_eq!(feature.feature_type, FeatureType::Extrude);
        assert_eq!(feature.name, "Extrude1");
        assert!(feature.get_parameter("distance").is_some());
    }

    #[test]
    fn test_feature_history() {
        let mut history = FeatureHistory::new();
        let feature = Feature::new(FeatureType::BaseFeature, "Base".to_string());
        let id = history.add_feature(feature);

        assert_eq!(history.features().len(), 1);
        assert!(history.get_feature(id).is_some());
    }

    #[test]
    fn test_feature_dependencies() {
        let mut history = FeatureHistory::new();

        let base = Feature::new(FeatureType::BaseFeature, "Base".to_string());
        let base_id = history.add_feature(base);

        let extrude = Feature::new(FeatureType::Extrude, "Extrude1".to_string());
        let extrude_id = history.add_feature(extrude);

        let base_feature = history.get_feature(base_id).unwrap();
        assert!(base_feature.child_features.contains(&extrude_id));

        let extrude_feature = history.get_feature(extrude_id).unwrap();
        assert!(extrude_feature.parent_features.contains(&base_id));
    }

    #[test]
    fn test_feature_builder() {
        let feature = FeatureBuilder::new(FeatureType::Extrude, "Extrude1")
            .real_parameter("distance", 10.0)
            .boolean_parameter("symmetric", false)
            .string_parameter("direction", "normal")
            .build();

        assert_eq!(feature.feature_type, FeatureType::Extrude);
        assert_eq!(feature.get_parameter("distance").unwrap().value, "10");
    }

    #[test]
    fn test_find_features() {
        let mut history = FeatureHistory::new();

        let extrude1 = Feature::new(FeatureType::Extrude, "Extrude1".to_string());
        history.add_feature(extrude1);

        let extrude2 = Feature::new(FeatureType::Extrude, "Extrude2".to_string());
        history.add_feature(extrude2);

        let fillet = Feature::new(FeatureType::Fillet, "Fillet1".to_string());
        history.add_feature(fillet);

        let extrudes = history.find_features_by_type(FeatureType::Extrude);
        assert_eq!(extrudes.len(), 2);

        let named = history.find_features_by_name("Extrude");
        assert_eq!(named.len(), 2);
    }

    #[test]
    fn test_feature_tree() {
        let mut history = FeatureHistory::new();

        let base = Feature::new(FeatureType::BaseFeature, "Base".to_string());
        let _base_id = history.add_feature(base);

        let extrude = Feature::new(FeatureType::Extrude, "Extrude1".to_string());
        let _extrude_id = history.add_feature(extrude);

        let fillet = Feature::new(FeatureType::Fillet, "Fillet1".to_string());
        let _fillet_id = history.add_feature(fillet);

        let tree = history.get_feature_tree();
        assert_eq!(tree.len(), 3);
        assert_eq!(tree[0].1, 0);
        assert_eq!(tree[1].1, 1);
        assert_eq!(tree[2].1, 2);
    }
}
