//! Unit tests for BrepRs foundation types

use breprs::foundation::*;

#[test]
fn test_types() {
    // Test Standard_Integer
    let int: Standard_Integer = 42;
    assert_eq!(int, 42);
    
    // Test Standard_Real
    let real: Standard_Real = 3.14159;
    assert!((real - 3.14159).abs() < 1e-6);
    
    // Test Standard_Boolean
    let boolean: Standard_Boolean = true;
    assert_eq!(boolean, true);
    
    // Test Standard_Character
    let char: Standard_Character = 'A';
    assert_eq!(char, 'A');
    
    // Test Standard_String
    let string: Standard_String = "Hello, BrepRs!".to_string();
    assert_eq!(string, "Hello, BrepRs!");
}

#[test]
fn test_handle() {
    // Test Handle creation and dereferencing
    let value = 42;
    let handle = Handle::new(value);
    assert_eq!(*handle, 42);
    
    // Test reference counting
    let handle2 = handle.clone();
    assert_eq!(*handle2, 42);
}

#[test]
fn test_memory() {
    // Test memory allocation
    let memory = Memory::new();
    assert_eq!(memory.allocated(), 0);
    
    // Test memory tracking (debug builds only)
    #[cfg(debug_assertions)]
    {
        let allocated = memory.allocated();
        let _ptr = Box::new(42);
        // Note: This is just a placeholder test
    }
}