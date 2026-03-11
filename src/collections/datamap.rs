use std::collections::HashMap;
use std::hash::Hash;

pub struct DataMap<K, V> {
    data: HashMap<K, V>,
}

impl<K: Hash + Eq, V> DataMap<K, V> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: HashMap::with_capacity(capacity),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.data.insert(key, value)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.data.get_mut(key)
    }

    pub fn take(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }

    pub fn entry(&mut self, key: K) -> std::collections::hash_map::Entry<'_, K, V> {
        self.data.entry(key)
    }

    pub fn keys(&self) -> std::collections::hash_map::Keys<'_, K, V> {
        self.data.keys()
    }

    pub fn values(&self) -> std::collections::hash_map::Values<'_, K, V> {
        self.data.values()
    }

    pub fn values_mut(&mut self) -> std::collections::hash_map::ValuesMut<'_, K, V> {
        self.data.values_mut()
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, K, V> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<'_, K, V> {
        self.data.iter_mut()
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        self.data.retain(f);
    }

    pub fn drain(&mut self) -> std::collections::hash_map::Drain<'_, K, V> {
        self.data.drain()
    }
}

impl<K: Hash + Eq, V> Default for DataMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

/// Performs a deep clone of all keys and values in the map.
/// This may be expensive for large maps.
impl<K: Hash + Eq + Clone, V: Clone> Clone for DataMap<K, V> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl<K: Hash + Eq + std::fmt::Debug, V: std::fmt::Debug> std::fmt::Debug for DataMap<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.data.iter()).finish()
    }
}

impl<K: Hash + Eq, V> DataMap<K, V> {
    pub fn get_checked(&self, key: &K) -> crate::foundation::exception::Result<&V> {
        self.data.get(key).ok_or_else(|| {
            crate::foundation::exception::Failure::range_error("key not found in DataMap")
        })
    }
    pub fn get_checked_mut(&mut self, key: &K) -> crate::foundation::exception::Result<&mut V> {
        self.data.get_mut(key).ok_or_else(|| {
            crate::foundation::exception::Failure::range_error("key not found in DataMap")
        })
    }
}

use std::borrow::Borrow;
use std::ops::{Index, IndexMut};

impl<K: Hash + Eq, V> Index<K> for DataMap<K, V> {
    type Output = V;

    fn index(&self, key: K) -> &Self::Output {
        self.data.get(&key).expect("Key not found in DataMap")
    }
}

impl<K: Hash + Eq, V> IndexMut<K> for DataMap<K, V> {
    fn index_mut(&mut self, key: K) -> &mut Self::Output {
        self.data.get_mut(&key).expect("Key not found in DataMap")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datamap_creation() {
        let map: DataMap<i32, i32> = DataMap::new();
        assert!(map.is_empty());
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_datamap_insert() {
        let mut map = DataMap::new();
        assert!(map.insert(1, 10).is_none());
        assert!(map.insert(2, 20).is_none());
        assert_eq!(map.insert(1, 100), Some(10));
        assert_eq!(map.size(), 2);
    }

    #[test]
    fn test_datamap_contains_key() {
        let mut map = DataMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        assert!(map.contains_key(&1));
        assert!(map.contains_key(&2));
        assert!(!map.contains_key(&3));
    }

    #[test]
    fn test_datamap_get() {
        let mut map = DataMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        assert_eq!(map.get(&1), Some(&10));
        assert_eq!(map.get(&2), Some(&20));
        assert_eq!(map.get(&3), None);
    }

    #[test]
    fn test_datamap_get_mut() {
        let mut map = DataMap::new();
        map.insert(1, 10);
        if let Some(value) = map.get_mut(&1) {
            *value = 100;
        }
        assert_eq!(map.get(&1), Some(&100));
    }

    #[test]
    fn test_datamap_remove() {
        let mut map = DataMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        assert_eq!(map.remove(&1), Some(10));
        assert!(!map.contains_key(&1));
        assert_eq!(map.size(), 1);
    }

    #[test]
    fn test_datamap_clear() {
        let mut map = DataMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        map.insert(3, 30);
        map.clear();
        assert!(map.is_empty());
    }

    #[test]
    fn test_datamap_iter() {
        let mut map = DataMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        map.insert(3, 30);
        let count = map.iter().count();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_datamap_keys() {
        let mut map = DataMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        map.insert(3, 30);
        let keys: Vec<&i32> = map.keys().collect();
        assert_eq!(keys.len(), 3);
    }

    #[test]
    fn test_datamap_values() {
        let mut map = DataMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        map.insert(3, 30);
        let values: Vec<&i32> = map.values().collect();
        assert_eq!(values.len(), 3);
    }

    #[test]
    fn test_datamap_clone() {
        let mut map1 = DataMap::new();
        map1.insert(1, 10);
        map1.insert(2, 20);
        map1.insert(3, 30);
        let map2 = map1.clone();
        assert_eq!(map2.size(), 3);
        assert_eq!(map2.get(&1), Some(&10));
        assert_eq!(map2.get(&2), Some(&20));
        assert_eq!(map2.get(&3), Some(&30));
    }

    #[test]
    fn test_datamap_index() {
        let mut map = DataMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        assert_eq!(map[1], 10);
        assert_eq!(map[2], 20);
    }

    #[test]
    fn test_datamap_index_mut() {
        let mut map = DataMap::new();
        map.insert(1, 10);
        map[1] = 100;
        assert_eq!(map[1], 100);
    }

    #[test]
    fn test_datamap_retain() {
        let mut map = DataMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        map.insert(3, 30);
        map.retain(|k, _| *k > 1);
        assert_eq!(map.size(), 2);
        assert!(!map.contains_key(&1));
    }
}
