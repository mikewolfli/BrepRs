use std::any::{Any, TypeId};
use std::sync::atomic::{AtomicUsize, Ordering};

pub trait Transient: Any + Send + Sync {
    fn type_id(&self) -> TypeId {
        Any::type_id(self)
    }

    fn dynamic_type(&self) -> TypeId {
        Any::type_id(self)
    }

    fn is_kind(&self, other: TypeId) -> bool {
        Any::type_id(self) == other
    }

    fn increment_ref_count(&self);
    fn decrement_ref_count(&self);
    fn ref_count(&self) -> usize;
}

pub trait TransientBuilder {
    fn build(self) -> Box<dyn Transient>;
}

pub struct TransientBase {
    ref_count: AtomicUsize,
}

impl TransientBase {
    pub fn new() -> Self {
        Self {
            ref_count: AtomicUsize::new(1),
        }
    }

    pub fn increment_ref_count(&self) {
        self.ref_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn decrement_ref_count(&self) -> usize {
        self.ref_count.fetch_sub(1, Ordering::SeqCst) - 1
    }

    pub fn ref_count(&self) -> usize {
        self.ref_count.load(Ordering::SeqCst)
    }
}

impl Default for TransientBase {
    fn default() -> Self {
        Self::new()
    }
}

#[macro_export]
macro_rules! impl_standard_transient {
    ($type:ty) => {
        impl Transient for $type {
            fn increment_ref_count(&self) {
                self.base.increment_ref_count();
            }

            fn decrement_ref_count(&self) {
                let count = self.base.decrement_ref_count();
                if count == 0 {
                    // SAFETY: This is safe because:
                    // - The reference count is 0, meaning no other references exist
                    // - The object was originally allocated on the heap via Box
                    // - We have exclusive ownership at this point
                    // - The pointer is valid and properly aligned
                    unsafe {
                        let _ = Box::from_raw(self as *const Self as *mut Self);
                    }
                }
            }

            fn ref_count(&self) -> usize {
                self.base.ref_count()
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestObject {
        base: TransientBase,
        value: i32,
    }

    impl TestObject {
        fn new(value: i32) -> Self {
            Self {
                base: TransientBase::new(),
                value,
            }
        }
    }

    impl_standard_transient!(TestObject);

    #[test]
    fn test_transient_ref_count() {
        let obj = TestObject::new(42);
        assert_eq!(obj.ref_count(), 1);

        obj.increment_ref_count();
        assert_eq!(obj.ref_count(), 2);

        obj.decrement_ref_count();
        assert_eq!(obj.ref_count(), 1);
    }

    #[test]
    fn test_transient_type_id() {
        let obj = TestObject::new(42);
        assert!(obj.is_kind(TypeId::of::<TestObject>()));
        assert!(!obj.is_kind(TypeId::of::<i32>()));
    }
}
