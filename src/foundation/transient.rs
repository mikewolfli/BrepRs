use std::any::{Any, TypeId};
use std::sync::atomic::{AtomicUsize, Ordering};

/// 基础对象特征，用于支持动态类型和引用计数。
///
/// 所有可被Handle管理的对象需实现该trait。
pub trait Transient: Any + Send + Sync {
    /// 获取类型ID
    fn type_id(&self) -> TypeId {
        Any::type_id(self)
    }

    /// 获取动态类型ID
    fn dynamic_type(&self) -> TypeId {
        Any::type_id(self)
    }

    /// 判断类型是否匹配
    fn is_kind(&self, other: TypeId) -> bool {
        Any::type_id(self) == other
    }

    /// 增加引用计数
    fn increment_ref_count(&self);
    /// 减少引用计数
    fn decrement_ref_count(&self);
    /// 获取当前引用计数
    fn ref_count(&self) -> usize;
}

/// 构建Transient对象的trait。
pub trait TransientBuilder {
    /// 构建对象
    fn build(self) -> Box<dyn Transient>;
}

/// Transient对象基础实现，包含引用计数。
pub struct TransientBase {
    ref_count: AtomicUsize,
}

impl TransientBase {
    /// 创建新对象，引用计数为1
    pub fn new() -> Self {
        Self {
            ref_count: AtomicUsize::new(1),
        }
    }

    /// 增加引用计数
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
        #[allow(dead_code)]
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
