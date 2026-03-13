use std::collections::HashSet;
use std::hash::Hash;

/// 集合类型，基于HashSet实现。
///
/// 用于存储唯一元素，支持容量管理和清空操作。
pub struct Map<T> {
    data: HashSet<T>,
}

impl<T: Hash + Eq> Map<T> {
    /// 创建空集合
    pub fn new() -> Self {
        Self {
            data: HashSet::new(),
        }
    }

    /// 按容量创建集合
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: HashSet::with_capacity(capacity),
        }
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// 元素数量
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// 元素数量（等价于size）
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// 当前容量
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// 清空集合
    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }

    pub fn insert(&mut self, value: T) -> bool {
        self.data.insert(value)
    }

    pub fn remove(&mut self, value: &T) -> bool {
        self.data.remove(value)
    }

    pub fn contains(&self, value: &T) -> bool {
        self.data.contains(value)
    }

    pub fn get(&self, value: &T) -> Option<&T> {
        self.data.get(value)
    }

    pub fn take(&mut self, value: &T) -> Option<T> {
        self.data.take(value)
    }

    pub fn replace(&mut self, value: T) -> Option<T> {
        self.data.replace(value)
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.data.retain(f);
    }

    pub fn iter(&self) -> std::collections::hash_set::Iter<'_, T> {
        self.data.iter()
    }

    pub fn union<'a>(&'a self, other: &'a Self) -> Union<'a, T> {
        Union {
            iter1: self.data.iter(),
            iter2: other.data.iter(),
        }
    }

    pub fn difference<'a>(&'a self, other: &'a Self) -> Difference<'a, T> {
        Difference {
            iter: self.data.iter(),
            other: &other.data,
        }
    }

    pub fn intersection<'a>(&'a self, other: &'a Self) -> Intersection<'a, T> {
        Intersection {
            iter: self.data.iter(),
            other: &other.data,
        }
    }

    pub fn symmetric_difference<'a>(&'a self, other: &'a Self) -> SymmetricDifference<'a, T> {
        SymmetricDifference {
            iter: self.data.iter().chain(other.data.iter()),
            set1: &self.data,
            set2: &other.data,
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.data.is_subset(&other.data)
    }

    pub fn is_superset(&self, other: &Self) -> bool {
        self.data.is_superset(&other.data)
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.data.is_disjoint(&other.data)
    }
}

impl<T: Hash + Eq> Default for Map<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Performs a deep clone of all elements in the map.
/// This may be expensive for large sets.
impl<T: Hash + Eq + Clone> Clone for Map<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl<T: Hash + Eq + std::fmt::Debug> std::fmt::Debug for Map<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.data.iter()).finish()
    }
}

pub struct Union<'a, T> {
    iter1: std::collections::hash_set::Iter<'a, T>,
    iter2: std::collections::hash_set::Iter<'a, T>,
}

impl<'a, T: Hash + Eq> Iterator for Union<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter1.next().or_else(|| self.iter2.next())
    }
}

pub struct Difference<'a, T> {
    iter: std::collections::hash_set::Iter<'a, T>,
    other: &'a HashSet<T>, // O(1) lookup, efficient
}

impl<'a, T: Hash + Eq> Iterator for Difference<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // Efficient: HashSet lookup is O(1)
        while let Some(item) = self.iter.next() {
            if !self.other.contains(item) {
                return Some(item);
            }
        }
        None
    }
}

pub struct Intersection<'a, T> {
    iter: std::collections::hash_set::Iter<'a, T>,
    other: &'a HashSet<T>,
}

impl<'a, T: Hash + Eq> Iterator for Intersection<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // Efficient: HashSet lookup is O(1)
        while let Some(item) = self.iter.next() {
            if self.other.contains(item) {
                return Some(item);
            }
        }
        None
    }
}

pub struct SymmetricDifference<'a, T> {
    iter: std::iter::Chain<
        std::collections::hash_set::Iter<'a, T>,
        std::collections::hash_set::Iter<'a, T>,
    >,
    set1: &'a HashSet<T>,
    set2: &'a HashSet<T>,
}

impl<'a, T: Hash + Eq> Iterator for SymmetricDifference<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // Efficient: HashSet lookup is O(1)
        while let Some(item) = self.iter.next() {
            let in_set1 = self.set1.contains(item);
            let in_set2 = self.set2.contains(item);
            if in_set1 ^ in_set2 {
                return Some(item);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_creation() {
        let map: Map<i32> = Map::new();
        assert!(map.is_empty());
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_map_insert() {
        let mut map = Map::new();
        assert!(map.insert(1));
        assert!(map.insert(2));
        assert!(!map.insert(1));
        assert_eq!(map.size(), 2);
    }

    #[test]
    fn test_map_contains() {
        let mut map = Map::new();
        map.insert(1);
        map.insert(2);
        assert!(map.contains(&1));
        assert!(map.contains(&2));
        assert!(!map.contains(&3));
    }

    #[test]
    fn test_map_remove() {
        let mut map = Map::new();
        map.insert(1);
        map.insert(2);
        assert!(map.remove(&1));
        assert!(!map.contains(&1));
        assert_eq!(map.size(), 1);
    }

    #[test]
    fn test_map_clear() {
        let mut map = Map::new();
        map.insert(1);
        map.insert(2);
        map.insert(3);
        map.clear();
        assert!(map.is_empty());
    }

    #[test]
    fn test_map_iter() {
        let mut map = Map::new();
        map.insert(1);
        map.insert(2);
        map.insert(3);
        let values: Vec<&i32> = map.iter().collect();
        assert_eq!(values.len(), 3);
    }

    #[test]
    fn test_map_clone() {
        let mut map1 = Map::new();
        map1.insert(1);
        map1.insert(2);
        map1.insert(3);
        let map2 = map1.clone();
        assert_eq!(map2.size(), 3);
        assert!(map2.contains(&1));
        assert!(map2.contains(&2));
        assert!(map2.contains(&3));
    }

    #[test]
    fn test_map_is_subset() {
        let mut map1 = Map::new();
        map1.insert(1);
        map1.insert(2);

        let mut map2 = Map::new();
        map2.insert(1);
        map2.insert(2);
        map2.insert(3);

        assert!(map1.is_subset(&map2));
        assert!(!map2.is_subset(&map1));
    }

    #[test]
    fn test_map_is_superset() {
        let mut map1 = Map::new();
        map1.insert(1);
        map1.insert(2);

        let mut map2 = Map::new();
        map2.insert(1);
        map2.insert(2);
        map2.insert(3);

        assert!(!map1.is_superset(&map2));
        assert!(map2.is_superset(&map1));
    }

    #[test]
    fn test_map_is_disjoint() {
        let mut map1 = Map::new();
        map1.insert(1);
        map1.insert(2);

        let mut map2 = Map::new();
        map2.insert(3);
        map2.insert(4);

        assert!(map1.is_disjoint(&map2));
    }
}
