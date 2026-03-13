/// Downcast inner Arc to a specific type if possible
// Moved to impl block below
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

/// Smart pointer wrapper for CAD objects, providing nullability and thread-safe reference counting.
///
/// This struct wraps an `Arc<T>` to provide a nullable smart pointer that is compatible
/// with CAD kernel patterns. It is thread-safe and can be shared across threads.
///
/// # Thread Safety
/// - Handle<T> is thread-safe as it uses `Arc<T>` internally
/// - Multiple handles can safely reference the same object across threads
/// - Mutation through `as_mut()` is only possible if there's a single strong reference
///
/// # Usage Patterns
/// - Use `Handle::new()` to create a new handle from an `Arc<T>`
/// - Use `Handle::null()` to create a null handle
/// - Use `is_null()` to check if a handle is null
/// - Use `as_ref()` for safe access to the inner value
/// - Use `as_mut()` for mutable access (only possible if no other references exist)
pub struct Handle<T: ?Sized> {
    inner: Option<Arc<T>>,
}

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "serde")]
impl<T: ?Sized + Serialize> Serialize for Handle<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.inner {
            Some(arc) => {
                // Serialize the inner value, not the Arc itself
                arc.as_ref().serialize(serializer)
            }
            None => {
                // Serialize a null value
                serializer.serialize_none()
            }
        }
    }
}

#[cfg(feature = "serde")]
impl<'de, T: ?Sized + Deserialize<'de>> Deserialize<'de> for Handle<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize as Option<T>
        let value: Option<T> = Option::deserialize(deserializer)?;
        match value {
            Some(v) => {
                // Create a new Arc from the deserialized value
                Ok(Handle::new(Arc::new(v)))
            }
            None => {
                // Create a null handle
                Ok(Handle::null())
            }
        }
    }
}

impl<T: ?Sized> Handle<T> {
    #[inline]
    pub fn new(obj: Arc<T>) -> Self {
        Self { inner: Some(obj) }
    }

    #[inline]
    pub fn null() -> Self {
        Self { inner: None }
    }

    #[inline]
    pub fn is_null(&self) -> bool {
        self.inner.is_none()
    }

    #[inline]
    pub fn is_null_handle(&self) -> bool {
        self.is_null()
    }

    #[inline]
    pub fn get(&self) -> Option<&Arc<T>> {
        self.inner.as_ref()
    }

    #[inline]
    pub fn as_ptr(&self) -> *const () {
        match &self.inner {
            Some(arc) => Arc::as_ptr(arc) as *const (),
            None => std::ptr::null(),
        }
    }

    #[inline]
    pub fn ref_count(&self) -> usize {
        match &self.inner {
            Some(arc) => Arc::strong_count(arc),
            None => 0,
        }
    }

    #[inline]
    pub fn as_ref(&self) -> Option<&T> {
        self.inner.as_deref()
    }

    #[inline]
    pub fn as_mut(&mut self) -> Option<&mut T> {
        match &mut self.inner {
            Some(arc) => Arc::get_mut(arc),
            None => None,
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
        self.inner
            .as_ref()
            .expect("Attempted to dereference null handle")
    }
}

impl<T: ?Sized> std::ops::DerefMut for Handle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Arc<T> does not support mutable borrow unless there is only one strong reference.
        // Use as_mut() for safe mutable access, otherwise panic.
        self.as_mut()
            .expect("Cannot borrow mutably: multiple references exist or handle is null")
    }
}

// Specific implementation for Handle<TopoDsShape> to provide faces method
impl Handle<crate::topology::topods_shape::TopoDsShape> {
    /// Get faces of the shape
    pub fn faces(&self) -> Vec<crate::topology::topods_face::TopoDsFace> {
        if let Some(shape) = self.as_ref() {
            shape.faces()
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_handle_creation() {
        let obj = Arc::new(42);
        let handle = Handle::new(obj);
        assert!(!handle.is_null());
        assert_eq!(handle.ref_count(), 1);
    }

    #[test]
    fn test_handle_null() {
        let handle: Handle<i32> = Handle::null();
        assert!(handle.is_null());
        assert!(handle.is_null_handle());
        assert_eq!(handle.ref_count(), 0);
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

    #[test]
    fn test_handle_as_ref() {
        let obj = Arc::new(42);
        let handle = Handle::new(obj);
        assert_eq!(handle.as_ref(), Some(&42));

        let null_handle: Handle<i32> = Handle::null();
        assert_eq!(null_handle.as_ref(), None);
    }

    #[test]
    fn test_handle_as_mut() {
        let obj = Arc::new(42);
        let mut handle = Handle::new(obj);

        // Can get mutable reference when only one reference exists
        if let Some(val) = handle.as_mut() {
            *val = 100;
        }
        assert_eq!(handle.as_ref(), Some(&100));

        // Cannot get mutable reference when multiple references exist
        let handle2 = handle.clone();
        assert!(handle.as_mut().is_none());
        assert_eq!(handle2.ref_count(), 2);
    }

    #[test]
    fn test_handle_ref_count_decrement() {
        let obj = Arc::new(42);
        let handle1 = Handle::new(obj);
        {
            let handle2 = handle1.clone();
            assert_eq!(handle1.ref_count(), 2);
            assert_eq!(handle2.ref_count(), 2);
        }
        // After handle2 goes out of scope
        assert_eq!(handle1.ref_count(), 1);
    }

    #[test]
    fn test_handle_null_equality() {
        let null1: Handle<i32> = Handle::null();
        let null2: Handle<i32> = Handle::null();
        assert_eq!(null1, null2);
    }

    #[test]
    fn test_handle_default() {
        let handle: Handle<i32> = Handle::default();
        assert!(handle.is_null());
    }

    #[test]
    fn test_handle_get() {
        let obj = Arc::new(42);
        let handle = Handle::new(obj);
        assert!(handle.get().is_some());
        assert_eq!(handle.get().unwrap().as_ref(), &42);

        let null_handle: Handle<i32> = Handle::null();
        assert!(null_handle.get().is_none());
    }

    #[test]
    fn test_handle_deref() {
        let obj = Arc::new(42);
        let handle = Handle::new(obj);
        assert_eq!(*handle, 42);
    }

    #[test]
    #[should_panic(expected = "Attempted to dereference null handle")]
    fn test_handle_deref_null() {
        let handle: Handle<i32> = Handle::null();
        // This should panic
        let _ = *handle;
    }

    #[test]
    fn test_handle_as_ptr() {
        let obj = Arc::new(42);
        let handle = Handle::new(obj);
        assert!(!handle.as_ptr().is_null());

        let null_handle: Handle<i32> = Handle::null();
        assert!(null_handle.as_ptr().is_null());
    }

    #[test]
    fn test_handle_thread_safety() {
        use std::thread;

        let obj = Arc::new(AtomicUsize::new(0));
        let handle = Handle::new(obj);

        let mut handles = vec![];
        for i in 0..10 {
            let _ = i;
            let h = handle.clone();
            handles.push(thread::spawn(move || {
                h.as_ref().unwrap().fetch_add(1, Ordering::SeqCst);
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        assert_eq!(handle.as_ref().unwrap().load(Ordering::SeqCst), 10);
    }

    #[test]
    fn test_handle_debug() {
        let obj = Arc::new(42);
        let handle = Handle::new(obj);
        let debug_str = format!("{:?}", handle);
        assert!(debug_str.contains("Handle"));
        assert!(!debug_str.contains("NULL"));

        let null_handle: Handle<i32> = Handle::null();
        let null_debug = format!("{:?}", null_handle);
        assert!(null_debug.contains("NULL"));
    }
}
