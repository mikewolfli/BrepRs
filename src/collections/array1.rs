use crate::foundation::exception::{Standard_Failure, Standard_Result};

pub struct NCollection_Array1<T> {
    data: Vec<T>,
    lower: usize,
    upper: usize,
}

impl<T> NCollection_Array1<T> {
    pub fn new(lower: usize, upper: usize) -> Self {
        if lower > upper {
            panic!("{}", Standard_Failure::range_error("Lower bound must be <= upper bound"));
        }
        let length = upper - lower + 1;
        Self {
            data: Vec::with_capacity(length),
            lower,
            upper,
        }
    }

    pub fn with_capacity(lower: usize, upper: usize, capacity: usize) -> Self {
        if lower > upper {
            panic!("{}", Standard_Failure::range_error("Lower bound must be <= upper bound"));
        }
        Self {
            data: Vec::with_capacity(capacity),
            lower,
            upper,
        }
    }

    pub fn from_vec(lower: usize, vec: Vec<T>) -> Self {
        let upper = lower + vec.len() - 1;
        Self {
            data: vec,
            lower,
            upper,
        }
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

    pub fn resize(&mut self, lower: usize, upper: usize)
    where
        T: Clone,
    {
        if lower > upper {
            panic!("{}", Standard_Failure::range_error("Lower bound must be <= upper bound"));
        }
        self.lower = lower;
        self.upper = upper;
        let new_length = upper - lower + 1;
        self.data.resize(new_length, self.data.first().cloned().unwrap_or_else(|| panic!("Cannot resize empty array")));
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

    pub fn set(&mut self, index: usize, value: T) -> Standard_Result<()> {
        if index < self.lower || index > self.upper {
            Err(Standard_Failure::range_error("Index out of bounds"))
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

    pub fn insert(&mut self, index: usize, value: T) -> Standard_Result<()> {
        if index < self.lower || index > self.upper + 1 {
            Err(Standard_Failure::range_error("Index out of bounds"))
        } else {
            self.data.insert(index - self.lower, value);
            self.upper += 1;
            Ok(())
        }
    }

    pub fn remove(&mut self, index: usize) -> Standard_Result<T> {
        if index < self.lower || index > self.upper {
            Err(Standard_Failure::range_error("Index out of bounds"))
        } else {
            self.upper -= 1;
            Ok(self.data.remove(index - self.lower))
        }
    }

    pub fn swap(&mut self, index1: usize, index2: usize) -> Standard_Result<()> {
        if index1 < self.lower || index1 > self.upper || index2 < self.lower || index2 > self.upper {
            Err(Standard_Failure::range_error("Index out of bounds"))
        } else {
            self.data.swap(index1 - self.lower, index2 - self.lower);
            Ok(())
        }
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
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

impl<T> Default for NCollection_Array1<T> {
    fn default() -> Self {
        Self::new(1, 0)
    }
}

impl<T> Clone for NCollection_Array1<T>
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

impl<T> std::ops::Index<usize> for NCollection_Array1<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
            .unwrap_or_else(|| panic!("{}", Standard_Failure::range_error("Index out of bounds")))
    }
}

impl<T> std::ops::IndexMut<usize> for NCollection_Array1<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index)
            .unwrap_or_else(|| panic!("{}", Standard_Failure::range_error("Index out of bounds")))
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for NCollection_Array1<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NCollection_Array1")
            .field("lower", &self.lower)
            .field("upper", &self.upper)
            .field("data", &self.data)
            .finish()
    }
}

impl<'a, T> IntoIterator for &'a NCollection_Array1<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<T> IntoIterator for NCollection_Array1<T> {
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
        let array: NCollection_Array1<i32> = NCollection_Array1::new(1, 5);
        assert_eq!(array.lower(), 1);
        assert_eq!(array.upper(), 5);
        assert_eq!(array.length(), 0);
        assert!(array.is_empty());
    }

    #[test]
    fn test_array_push() {
        let mut array = NCollection_Array1::new(1, 5);
        array.push(10);
        array.push(20);
        array.push(30);
        assert_eq!(array.length(), 3);
        assert_eq!(array[1], 10);
        assert_eq!(array[2], 20);
        assert_eq!(array[3], 30);
    }

    #[test]
    fn test_array_pop() {
        let mut array = NCollection_Array1::new(1, 5);
        array.push(10);
        array.push(20);
        array.push(30);
        assert_eq!(array.pop(), Some(30));
        assert_eq!(array.length(), 2);
    }

    #[test]
    fn test_array_get() {
        let mut array = NCollection_Array1::new(1, 5);
        array.push(10);
        array.push(20);
        assert_eq!(array.get(1), Some(&10));
        assert_eq!(array.get(2), Some(&20));
        assert_eq!(array.get(3), None);
    }

    #[test]
    fn test_array_set() {
        let mut array = NCollection_Array1::new(1, 5);
        array.push(10);
        array.push(20);
        assert!(array.set(1, 100).is_ok());
        assert_eq!(array[1], 100);
    }

    #[test]
    fn test_array_insert() {
        let mut array = NCollection_Array1::new(1, 5);
        array.push(10);
        array.push(30);
        assert!(array.insert(2, 20).is_ok());
        assert_eq!(array[1], 10);
        assert_eq!(array[2], 20);
        assert_eq!(array[3], 30);
    }

    #[test]
    fn test_array_remove() {
        let mut array = NCollection_Array1::new(1, 5);
        array.push(10);
        array.push(20);
        array.push(30);
        assert_eq!(array.remove(2).unwrap(), 20);
        assert_eq!(array.length(), 2);
        assert_eq!(array[1], 10);
        assert_eq!(array[2], 30);
    }

    #[test]
    fn test_array_swap() {
        let mut array = NCollection_Array1::new(1, 5);
        array.push(10);
        array.push(20);
        assert!(array.swap(1, 2).is_ok());
        assert_eq!(array[1], 20);
        assert_eq!(array[2], 10);
    }

    #[test]
    fn test_array_clear() {
        let mut array = NCollection_Array1::new(1, 5);
        array.push(10);
        array.push(20);
        array.clear();
        assert!(array.is_empty());
    }

    #[test]
    fn test_array_from_vec() {
        let vec = vec![10, 20, 30];
        let array = NCollection_Array1::from_vec(1, vec);
        assert_eq!(array.lower(), 1);
        assert_eq!(array.upper(), 3);
        assert_eq!(array.length(), 3);
        assert_eq!(array[1], 10);
        assert_eq!(array[2], 20);
        assert_eq!(array[3], 30);
    }

    #[test]
    fn test_array_to_vec() {
        let mut array = NCollection_Array1::new(1, 5);
        array.push(10);
        array.push(20);
        array.push(30);
        let vec = array.to_vec();
        assert_eq!(vec, vec![10, 20, 30]);
    }

    #[test]
    fn test_array_clone() {
        let mut array1 = NCollection_Array1::new(1, 5);
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
        let mut array = NCollection_Array1::new(1, 5);
        array.push(10);
        array.push(20);
        array.push(30);
        let values: Vec<i32> = array.into_iter().collect();
        assert_eq!(values, vec![10, 20, 30]);
    }

    #[test]
    fn test_array_bounds() {
        let array: NCollection_Array1<i32> = NCollection_Array1::new(1, 5);
        assert_eq!(array.lower(), 1);
        assert_eq!(array.upper(), 5);
    }
}
