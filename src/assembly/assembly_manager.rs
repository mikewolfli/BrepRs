use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::topology::topods_shape::TopoDsShape;
use crate::geometry::{Point, Transform};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ComponentId(pub u64);

impl ComponentId {
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}

impl Default for ComponentId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssemblyId(pub u64);

impl AssemblyId {
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}

impl Default for AssemblyId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Component {
    id: ComponentId,
    name: String,
    shape: Option<TopoDsShape>,
    transform: Transform,
    children: Vec<ComponentId>,
    parent: Option<ComponentId>,
    properties: HashMap<String, String>,
}

impl Component {
    pub fn new(name: String) -> Self {
        Self {
            id: ComponentId::new(),
            name,
            shape: None,
            transform: Transform::identity(),
            children: Vec::new(),
            parent: None,
            properties: HashMap::new(),
        }
    }

    pub fn with_shape(mut self, shape: TopoDsShape) -> Self {
        self.shape = Some(shape);
        self
    }

    pub fn id(&self) -> ComponentId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn shape(&self) -> Option<&TopoDsShape> {
        self.shape.as_ref()
    }

    pub fn set_shape(&mut self, shape: TopoDsShape) {
        self.shape = Some(shape);
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    pub fn children(&self) -> &[ComponentId] {
        &self.children
    }

    pub fn parent(&self) -> Option<ComponentId> {
        self.parent
    }

    pub fn add_child(&mut self, child_id: ComponentId) {
        if !self.children.contains(&child_id) {
            self.children.push(child_id);
        }
    }

    pub fn remove_child(&mut self, child_id: ComponentId) {
        self.children.retain(|&id| id != child_id);
    }

    pub fn set_property(&mut self, key: String, value: String) {
        self.properties.insert(key, value);
    }

    pub fn get_property(&self, key: &str) -> Option<&String> {
        self.properties.get(key)
    }

    pub fn properties(&self) -> &HashMap<String, String> {
        &self.properties
    }

    pub fn world_transform(&self, assembly: &Assembly) -> Transform {
        let mut result = self.transform.clone();
        let mut current_parent = self.parent;

        while let Some(parent_id) = current_parent {
            if let Some(parent) = assembly.get_component(parent_id) {
                result = parent.transform.multiply(&result);
                current_parent = parent.parent;
            } else {
                break;
            }
        }

        result
    }
}

#[derive(Debug, Clone)]
pub struct Assembly {
    id: AssemblyId,
    name: String,
    root_components: Vec<ComponentId>,
    components: HashMap<ComponentId, Component>,
    sub_assemblies: HashMap<AssemblyId, AssemblyId>,
}

impl Assembly {
    pub fn new(name: String) -> Self {
        Self {
            id: AssemblyId::new(),
            name,
            root_components: Vec::new(),
            components: HashMap::new(),
            sub_assemblies: HashMap::new(),
        }
    }

    pub fn id(&self) -> AssemblyId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn add_component(&mut self, component: Component) -> ComponentId {
        let id = component.id;
        self.components.insert(id, component);
        self.root_components.push(id);
        id
    }

    pub fn add_component_as_child(&mut self, parent_id: ComponentId, mut component: Component) -> Result<ComponentId, String> {
        if !self.components.contains_key(&parent_id) {
            return Err("Parent component not found".to_string());
        }

        let child_id = component.id;
        component.parent = Some(parent_id);
        self.components.insert(child_id, component);

        if let Some(parent) = self.components.get_mut(&parent_id) {
            parent.add_child(child_id);
        }

        Ok(child_id)
    }

    pub fn remove_component(&mut self, component_id: ComponentId) -> Result<Component, String> {
        if let Some(component) = self.components.remove(&component_id) {
            if let Some(parent_id) = component.parent {
                if let Some(parent) = self.components.get_mut(&parent_id) {
                    parent.remove_child(component_id);
                }
            } else {
                self.root_components.retain(|&id| id != component_id);
            }

            for child_id in component.children.clone() {
                self.remove_component(child_id)?;
            }

            Ok(component)
        } else {
            Err("Component not found".to_string())
        }
    }

    pub fn get_component(&self, component_id: ComponentId) -> Option<&Component> {
        self.components.get(&component_id)
    }

    pub fn get_component_mut(&mut self, component_id: ComponentId) -> Option<&mut Component> {
        self.components.get_mut(&component_id)
    }

    pub fn root_components(&self) -> &[ComponentId] {
        &self.root_components
    }

    pub fn components(&self) -> &HashMap<ComponentId, Component> {
        &self.components
    }

    pub fn component_count(&self) -> usize {
        self.components.len()
    }

    pub fn find_component_by_name(&self, name: &str) -> Option<ComponentId> {
        for (id, component) in &self.components {
            if component.name == name {
                return Some(*id);
            }
        }
        None
    }

    pub fn find_components_by_property(&self, key: &str, value: &str) -> Vec<ComponentId> {
        self.components
            .iter()
            .filter(|(_, c)| c.get_property(key).map(|v| v == value).unwrap_or(false))
            .map(|(id, _)| *id)
            .collect()
    }

    pub fn add_sub_assembly(&mut self, sub_assembly_id: AssemblyId) {
        self.sub_assemblies.insert(sub_assembly_id, sub_assembly_id);
    }

    pub fn sub_assemblies(&self) -> &HashMap<AssemblyId, AssemblyId> {
        &self.sub_assemblies
    }

    pub fn traverse<F>(&self, mut visitor: F)
    where
        F: FnMut(&Component, usize),
    {
        fn visit_component<F>(assembly: &Assembly, component_id: ComponentId, depth: usize, visitor: &mut F)
        where
            F: FnMut(&Component, usize),
        {
            if let Some(component) = assembly.get_component(component_id) {
                visitor(component, depth);
                for &child_id in component.children() {
                    visit_component(assembly, child_id, depth + 1, visitor);
                }
            }
        }

        for &root_id in &self.root_components {
            visit_component(self, root_id, 0, &mut visitor);
        }
    }

    pub fn traverse_mut<F>(&mut self, mut visitor: F)
    where
        F: FnMut(&mut Component, usize),
    {
        fn visit_component<F>(assembly: &mut Assembly, component_id: ComponentId, depth: usize, visitor: &mut F)
        where
            F: FnMut(&mut Component, usize),
        {
            let children: Vec<ComponentId> = assembly
                .get_component(component_id)
                .map(|c| c.children.clone())
                .unwrap_or_default();

            if let Some(component) = assembly.get_component_mut(component_id) {
                visitor(component, depth);
            }

            for child_id in children {
                visit_component(assembly, child_id, depth + 1, visitor);
            }
        }

        let root_ids: Vec<ComponentId> = self.root_components.clone();
        for root_id in root_ids {
            visit_component(self, root_id, 0, &mut visitor);
        }
    }

    pub fn get_all_shapes(&self) -> Vec<(ComponentId, TopoDsShape)> {
        let mut shapes = Vec::new();
        for (id, component) in &self.components {
            if let Some(shape) = &component.shape {
                shapes.push((*id, shape.clone()));
            }
        }
        shapes
    }

    pub fn get_bounding_box(&self) -> Option<(Point, Point)> {
        let mut min_point: Option<Point> = None;
        let mut max_point: Option<Point> = None;

        for component in self.components.values() {
            if let Some(shape) = &component.shape {
                let (shape_min, shape_max) = shape.bounding_box();
                let world_transform = component.world_transform(self);

                let corners = [
                    Point::new(shape_min.x, shape_min.y, shape_min.z),
                    Point::new(shape_max.x, shape_min.y, shape_min.z),
                    Point::new(shape_min.x, shape_max.y, shape_min.z),
                    Point::new(shape_max.x, shape_max.y, shape_min.z),
                    Point::new(shape_min.x, shape_min.y, shape_max.z),
                    Point::new(shape_max.x, shape_min.y, shape_max.z),
                    Point::new(shape_min.x, shape_max.y, shape_max.z),
                    Point::new(shape_max.x, shape_max.y, shape_max.z),
                ];

                for corner in corners {
                    let transformed = world_transform.transforms(&corner);

                    min_point = Some(match min_point {
                        None => transformed,
                        Some(min) => Point::new(
                            min.x.min(transformed.x),
                            min.y.min(transformed.y),
                            min.z.min(transformed.z),
                        ),
                    });

                    max_point = Some(match max_point {
                        None => transformed,
                        Some(max) => Point::new(
                            max.x.max(transformed.x),
                            max.y.max(transformed.y),
                            max.z.max(transformed.z),
                        ),
                    });
                }
            }
        }

        match (min_point, max_point) {
            (Some(min), Some(max)) => Some((min, max)),
            _ => None,
        }
    }
}

pub struct AssemblyManager {
    assemblies: HashMap<AssemblyId, Arc<Mutex<Assembly>>>,
    active_assembly: Option<AssemblyId>,
}

impl AssemblyManager {
    pub fn new() -> Self {
        Self {
            assemblies: HashMap::new(),
            active_assembly: None,
        }
    }

    pub fn create_assembly(&mut self, name: String) -> AssemblyId {
        let assembly = Assembly::new(name);
        let id = assembly.id;
        self.assemblies.insert(id, Arc::new(Mutex::new(assembly)));
        id
    }

    pub fn get_assembly(&self, id: AssemblyId) -> Option<Arc<Mutex<Assembly>>> {
        self.assemblies.get(&id).cloned()
    }

    pub fn remove_assembly(&mut self, id: AssemblyId) -> Option<Arc<Mutex<Assembly>>> {
        if self.active_assembly == Some(id) {
            self.active_assembly = None;
        }
        self.assemblies.remove(&id)
    }

    pub fn set_active_assembly(&mut self, id: AssemblyId) -> Result<(), String> {
        if self.assemblies.contains_key(&id) {
            self.active_assembly = Some(id);
            Ok(())
        } else {
            Err("Assembly not found".to_string())
        }
    }

    pub fn active_assembly(&self) -> Option<Arc<Mutex<Assembly>>> {
        self.active_assembly.and_then(|id| self.assemblies.get(&id).cloned())
    }

    pub fn assembly_count(&self) -> usize {
        self.assemblies.len()
    }

    pub fn assemblies(&self) -> &HashMap<AssemblyId, Arc<Mutex<Assembly>>> {
        &self.assemblies
    }

    pub fn merge_assemblies(&mut self, target_id: AssemblyId, source_id: AssemblyId) -> Result<(), String> {
        let source = self.assemblies.get(&source_id)
            .ok_or("Source assembly not found")?
            .lock()
            .map_err(|_| "Failed to lock source assembly")?
            .clone();

        {
            let target = self.assemblies.get_mut(&target_id)
                .ok_or("Target assembly not found")?;

            let mut target = target.lock()
                .map_err(|_| "Failed to lock target assembly")?;

            for (_, component) in source.components {
                let id = component.id;
                target.components.insert(id, component);
                target.root_components.push(id);
            }
        }

        self.remove_assembly(source_id);
        Ok(())
    }
}

impl Default for AssemblyManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct AssemblyTree {
    assembly: Assembly,
}

impl AssemblyTree {
    pub fn new(assembly: Assembly) -> Self {
        Self { assembly }
    }

    pub fn assembly(&self) -> &Assembly {
        &self.assembly
    }

    pub fn assembly_mut(&mut self) -> &mut Assembly {
        &mut self.assembly
    }

    pub fn get_path(&self, component_id: ComponentId) -> Vec<ComponentId> {
        let mut path = Vec::new();
        let mut current = self.assembly.get_component(component_id);

        while let Some(component) = current {
            path.insert(0, component.id);
            current = component.parent.and_then(|id| self.assembly.get_component(id));
        }

        path
    }

    pub fn get_depth(&self, component_id: ComponentId) -> Option<usize> {
        let path = self.get_path(component_id);
        if path.is_empty() {
            None
        } else {
            Some(path.len() - 1)
        }
    }

    pub fn get_siblings(&self, component_id: ComponentId) -> Vec<ComponentId> {
        if let Some(component) = self.assembly.get_component(component_id) {
            if let Some(parent_id) = component.parent {
                if let Some(parent) = self.assembly.get_component(parent_id) {
                    return parent.children.iter()
                        .filter(|&&id| id != component_id)
                        .copied()
                        .collect();
                }
            } else {
                return self.assembly.root_components.iter()
                    .filter(|&&id| id != component_id)
                    .copied()
                    .collect();
            }
        }
        Vec::new()
    }

    pub fn get_descendants(&self, component_id: ComponentId) -> Vec<ComponentId> {
        let mut descendants = Vec::new();
        self.collect_descendants(component_id, &mut descendants);
        descendants
    }

    fn collect_descendants(&self, component_id: ComponentId, descendants: &mut Vec<ComponentId>) {
        if let Some(component) = self.assembly.get_component(component_id) {
            for &child_id in component.children() {
                descendants.push(child_id);
                self.collect_descendants(child_id, descendants);
            }
        }
    }

    pub fn get_ancestors(&self, component_id: ComponentId) -> Vec<ComponentId> {
        let mut ancestors = Vec::new();
        let mut current = self.assembly.get_component(component_id);

        while let Some(component) = current {
            if let Some(parent_id) = component.parent {
                ancestors.push(parent_id);
                current = self.assembly.get_component(parent_id);
            } else {
                break;
            }
        }

        ancestors
    }

    pub fn is_ancestor(&self, potential_ancestor: ComponentId, component: ComponentId) -> bool {
        let ancestors = self.get_ancestors(component);
        ancestors.contains(&potential_ancestor)
    }

    pub fn is_descendant(&self, potential_descendant: ComponentId, component: ComponentId) -> bool {
        let descendants = self.get_descendants(component);
        descendants.contains(&potential_descendant)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_creation() {
        let component = Component::new("TestComponent".to_string());
        assert_eq!(component.name(), "TestComponent");
        assert!(component.shape().is_none());
        assert!(component.children().is_empty());
    }

    #[test]
    fn test_assembly_creation() {
        let assembly = Assembly::new("TestAssembly".to_string());
        assert_eq!(assembly.name(), "TestAssembly");
        assert!(assembly.root_components().is_empty());
        assert_eq!(assembly.component_count(), 0);
    }

    #[test]
    fn test_add_component() {
        let mut assembly = Assembly::new("TestAssembly".to_string());
        let component = Component::new("TestComponent".to_string());
        let id = assembly.add_component(component);

        assert_eq!(assembly.component_count(), 1);
        assert!(assembly.get_component(id).is_some());
    }

    #[test]
    fn test_add_child_component() {
        let mut assembly = Assembly::new("TestAssembly".to_string());
        let parent = Component::new("Parent".to_string());
        let parent_id = assembly.add_component(parent);

        let child = Component::new("Child".to_string());
        let child_id = assembly.add_component_as_child(parent_id, child).unwrap();

        assert_eq!(assembly.component_count(), 2);
        assert!(assembly.get_component(child_id).unwrap().parent().is_some());
        assert!(assembly.get_component(parent_id).unwrap().children().contains(&child_id));
    }

    #[test]
    fn test_assembly_manager() {
        let mut manager = AssemblyManager::new();
        let id = manager.create_assembly("TestAssembly".to_string());

        assert_eq!(manager.assembly_count(), 1);
        assert!(manager.get_assembly(id).is_some());
    }

    #[test]
    fn test_assembly_tree() {
        let mut assembly = Assembly::new("TestAssembly".to_string());
        let parent = Component::new("Parent".to_string());
        let parent_id = assembly.add_component(parent);

        let child = Component::new("Child".to_string());
        let child_id = assembly.add_component_as_child(parent_id, child).unwrap();

        let tree = AssemblyTree::new(assembly);

        let path = tree.get_path(child_id);
        assert_eq!(path.len(), 2);

        let depth = tree.get_depth(child_id);
        assert_eq!(depth, Some(1));
    }
}
