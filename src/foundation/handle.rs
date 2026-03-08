use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

pub struct Handle<T: ?Sized> {
    inner: Option<Arc<T>>,
}

impl<T: ?Sized> Handle<T> {
    pub fn new(obj: Arc<T>) -> Self {
        Self { inner: Some(obj) }
    }

    pub fn null() -> Self {
        Self { inner: None }
    }

    pub fn is_null(&self) -> bool {
        self.inner.is_none()
    }

    pub fn is_null_handle(&self) -> bool {
        self.is_null()
    }

    pub fn get(&self) -> Option<&Arc<T>> {
        self.inner.as_ref()
    }

    pub fn as_ptr(&self) -> *const () {
        match &self.inner {
            Some(arc) => Arc::as_ptr(arc) as *const (),
            None => std::ptr::null(),
        }
    }

    pub fn ref_count(&self) -> usize {
        match &self.inner {
            Some(arc) => Arc::strong_count(arc),
            None => 0,
        }
    }
}

impl<T: ?Sized> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: ?Sized> Default for Handle<T> {
    fn default() -> Self {
        Self::null()
    }
}

impl<T: ?Sized> fmt::Debug for Handle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.inner {
            Some(_) => write!(f, "Handle(...)"),
            None => write!(f, "Handle(NULL)"),
        }
    }
}

impl<T: ?Sized> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        match (&self.inner, &other.inner) {
            (Some(a), Some(b)) => Arc::ptr_eq(a, b),
            (None, None) => true,
            _ => false,
        }
    }
}

impl<T: ?Sized> Eq for Handle<T> {}

impl<T: ?Sized> Hash for Handle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self.inner {
            Some(arc) => {
                let ptr = Arc::as_ptr(arc) as *const () as usize;
                ptr.hash(state);
            }
            None => {
                0usize.hash(state);
            }
        }
    }
}

impl<T: ?Sized> std::ops::Deref for Handle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_creation() {
        let obj = Arc::new(42);
        let handle = Handle::new(obj);
        assert!(!handle.is_null());
    }

    #[test]
    fn test_handle_null() {
        let handle: Handle<i32> = Handle::null();
        assert!(handle.is_null());
    }

    #[test]
    fn test_handle_clone() {
        let obj = Arc::new(42);
        let handle1 = Handle::new(obj);
        let handle2 = handle1.clone();
        assert_eq!(handle1.ref_count(), 2);
        assert_eq!(handle2.ref_count(), 2);
    }

    #[test]
    fn test_handle_equality() {
        let obj1 = Arc::new(42);
        let obj2 = Arc::new(42);
        let handle1 = Handle::new(obj1);
        let handle2 = Handle::new(obj2);
        let handle3 = handle1.clone();

        assert_ne!(handle1, handle2);
        assert_eq!(handle1, handle3);
    }

    #[test]
    fn test_handle_hash() {
        use std::collections::HashSet;

        let obj1 = Arc::new(42);
        let obj2 = Arc::new(42);
        let handle1 = Handle::new(obj1);
        let handle2 = Handle::new(obj2);
        let handle3 = handle1.clone();

        let mut set = HashSet::new();
        set.insert(handle1.clone());
        set.insert(handle2);
        set.insert(handle3);

        assert_eq!(set.len(), 2);
    }
}
