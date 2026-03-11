use crate::foundation::exception::{Failure, Result};

pub struct Array1<T> {
    data: Vec<T>,
    lower: usize,
    upper: usize,
}

impl<T> Array1<T> {
    pub fn new(lower: usize, upper: usize) -> Result<Self> {
        if lower > upper {
            return Err(Failure::range_error("Lower bound must be <= upper bound"));
        }
        let length = upper - lower + 1;
        Ok(Self {
            data: Vec::with_capacity(length),
            lower,
            upper,
        })
    }

    pub fn with_capacity(lower: usize, upper: usize, capacity: usize) -> Result<Self> {
        if lower > upper {
            return Err(Failure::range_error("Lower bound must be <= upper bound"));
        }
        Ok(Self {
            data: Vec::with_capacity(capacity),
            lower,
            upper,
        })
    }

    pub fn from_vec(lower: usize, vec: Vec<T>) -> Result<Self> {
        if vec.is_empty() {
            return Err(Failure::range_error("Cannot create Array1 from empty Vec"));
        }
        let upper = lower + vec.len() - 1;
        Ok(Self {
            data: vec,
            lower,
            upper,
        })
    }

    pub fn lower(&self) -> usize {
        self.lower
    }

    pub fn upper(&self) -> usize {
        self.upper
    }

    pub fn length(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    pub fn resize(&mut self, lower: usize, upper: usize) -> Result<()>
    where
        T: Clone,
    {
        if lower > upper {
            return Err(Failure::range_error("Lower bound must be <= upper bound"));
        }
        self.lower = lower;
        self.upper = upper;
        let new_length = upper - lower + 1;
        let default = self
            .data
            .first()
            .cloned()
            .ok_or_else(|| Failure::range_error("Cannot resize empty array"))?;
        self.data.resize(new_length, default);
        Ok(())
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

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.lower || index > self.upper {
            None
        } else {
            self.data.get(index - self.lower)
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.lower || index > self.upper {
            None
        } else {
            self.data.get_mut(index - self.lower)
        }
    }

    pub fn set(&mut self, index: usize, value: T) -> Result<()> {
        if index < self.lower || index > self.upper {
            Err(Failure::range_error("Index out of bounds"))
        } else {
            self.data[index - self.lower] = value;
            Ok(())
        }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
        self.upper += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            None
        } else {
            self.upper -= 1;
            self.data.pop()
        }
    }

    pub fn insert(&mut self, index: usize, value: T) -> Result<()> {
        if index < self.lower || index > self.upper + 1 {
            Err(Failure::range_error("Index out of bounds"))
        } else {
            self.data.insert(index - self.lower, value);
            self.upper += 1;
            Ok(())
        }
    }

    pub fn remove(&mut self, index: usize) -> Result<T> {
        if index < self.lower || index > self.upper {
            Err(Failure::range_error("Index out of bounds"))
        } else {
            self.upper -= 1;
            Ok(self.data.remove(index - self.lower))
        }
    }

    pub fn swap(&mut self, index1: usize, index2: usize) -> Result<()> {
        if index1 < self.lower || index1 > self.upper || index2 < self.lower || index2 > self.upper
        {
            Err(Failure::range_error("Index out of bounds"))
        } else {
            self.data.swap(index1 - self.lower, index2 - self.lower);
            Ok(())
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.data.iter_mut()
    }

    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }

    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.data.clone()
    }

    pub fn into_vec(self) -> Vec<T> {
        self.data
    }
}

impl<T> Default for Array1<T> {
    fn default() -> Self {
        // Default returns an empty array with lower > upper
        Self {
            data: Vec::new(),
            lower: 1,
            upper: 0,
        }
    }
}

/// Performs a deep clone of all elements in the array.
/// This may be expensive for large arrays.
impl<T> Clone for Array1<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            lower: self.lower,
            upper: self.upper,
        }
    }
}

impl<T> Array1<T> {
    pub fn get_checked(&self, index: usize) -> Result<&T> {
        self.get(index)
            .ok_or_else(|| Failure::range_error("Index out of bounds"))
    }
    pub fn get_checked_mut(&mut self, index: usize) -> Result<&mut T> {
        self.get_mut(index)
            .ok_or_else(|| Failure::range_error("Index out of bounds"))
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Array1<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Array1")
            .field("lower", &self.lower)
            .field("upper", &self.upper)
            .field("data", &self.data)
            .finish()
    }
}

impl<'a, T> IntoIterator for &'a Array1<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<T> IntoIterator for Array1<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_creation() {
        let array: Array1<i32> = Array1::new(1, 5).unwrap();
        assert_eq!(array.lower(), 1);
        assert_eq!(array.upper(), 5);
        assert_eq!(array.length(), 0);
        assert!(array.is_empty());
    }

    #[test]
    fn test_array_push() {
        let mut array = Array1::new(1, 5).unwrap();
        array.push(10);
        array.push(20);
        array.push(30);
        assert_eq!(array.length(), 3);
        assert_eq!(*array.get_checked(1).unwrap(), 10);
        assert_eq!(*array.get_checked(2).unwrap(), 20);
        assert_eq!(*array.get_checked(3).unwrap(), 30);
    }

    #[test]
    fn test_array_pop() {
        let mut array = Array1::new(1, 5).unwrap();
        array.push(10);
        array.push(20);
        array.push(30);
        assert_eq!(array.pop(), Some(30));
        assert_eq!(array.length(), 2);
    }

    #[test]
    fn test_array_get() {
        let mut array = Array1::new(1, 5).unwrap();
        array.push(10);
        array.push(20);
        assert_eq!(array.get(1), Some(&10));
        assert_eq!(array.get(2), Some(&20));
        assert_eq!(array.get(3), None);
    }

    #[test]
    fn test_array_set() {
        let mut array = Array1::new(1, 5).unwrap();
        array.push(10);
        array.push(20);
        assert!(array.set(1, 100).is_ok());
        assert_eq!(*array.get_checked(1).unwrap(), 100);
    }

    #[test]
    fn test_array_insert() {
        let mut array = Array1::new(1, 5).unwrap();
        array.push(10);
        array.push(30);
        assert!(array.insert(2, 20).is_ok());
        assert_eq!(*array.get_checked(1).unwrap(), 10);
        assert_eq!(*array.get_checked(2).unwrap(), 20);
        assert_eq!(*array.get_checked(3).unwrap(), 30);
    }

    #[test]
    fn test_array_remove() {
        let mut array = Array1::new(1, 5).unwrap();
        array.push(10);
        array.push(20);
        array.push(30);
        assert_eq!(array.remove(2).unwrap(), 20);
        assert_eq!(array.length(), 2);
        assert_eq!(*array.get_checked(1).unwrap(), 10);
        assert_eq!(*array.get_checked(2).unwrap(), 30);
    }

    #[test]
    fn test_array_swap() {
        let mut array = Array1::new(1, 5).unwrap();
        array.push(10);
        array.push(20);
        assert!(array.swap(1, 2).is_ok());
        assert_eq!(*array.get_checked(1).unwrap(), 20);
        assert_eq!(*array.get_checked(2).unwrap(), 10);
    }

    #[test]
    fn test_array_clear() {
        let mut array = Array1::new(1, 5).unwrap();
        array.push(10);
        array.push(20);
        array.clear();
        assert!(array.is_empty());
    }

    #[test]
    fn test_array_from_vec() {
        let vec = vec![10, 20, 30];
        let array = Array1::from_vec(1, vec).unwrap();
        assert_eq!(array.lower(), 1);
        assert_eq!(array.upper(), 3);
        assert_eq!(array.length(), 3);
        assert_eq!(*array.get_checked(1).unwrap(), 10);
        assert_eq!(*array.get_checked(2).unwrap(), 20);
        assert_eq!(*array.get_checked(3).unwrap(), 30);
    }

    #[test]
    fn test_array_to_vec() {
        let mut array = Array1::new(1, 5).unwrap();
        array.push(10);
        array.push(20);
        array.push(30);
        let vec = array.to_vec();
        assert_eq!(vec, vec![10, 20, 30]);
    }

    #[test]
    fn test_array_clone() {
        let mut array1 = Array1::new(1, 5).unwrap();
        array1.push(10);
        array1.push(20);
        array1.push(30);
        let array2 = array1.clone();
        assert_eq!(array2.lower(), array1.lower());
        assert_eq!(array2.upper(), array1.upper());
        assert_eq!(array2.to_vec(), vec![10, 20, 30]);
    }

    #[test]
    fn test_array_into_iter() {
        let mut array = Array1::new(1, 5).unwrap();
        array.push(10);
        array.push(20);
        array.push(30);
        let values: Vec<i32> = array.into_iter().collect();
        assert_eq!(values, vec![10, 20, 30]);
    }

    #[test]
    fn test_array_bounds() {
        let array: Array1<i32> = Array1::new(1, 5).unwrap();
        assert_eq!(array.lower(), 1);
        assert_eq!(array.upper(), 5);
    }
}
