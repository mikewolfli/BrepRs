use crate::plugin::{PluginError, PluginManager};

#[test]
fn test_plugin_manager_basic() {
    // Create a plugin manager
    let mut manager = PluginManager::new();
    
    // Test initialization
    let result = manager.initialize();
    assert!(result.is_ok());
    assert!(manager.is_initialized());
    
    // Test shutdown
    let result = manager.shutdown();
    assert!(result.is_ok());
    assert!(!manager.is_initialized());
}

#[test]
fn test_plugin_manager_with_paths() {
    // Create a plugin manager
    let mut manager = PluginManager::new();
    
    // Add a plugin path (non-existent directory)
    manager.add_plugin_path("non_existent_directory");
    
    // Test initialization
    let result = manager.initialize();
    assert!(result.is_ok());
    assert!(manager.is_initialized());
    
    // Test shutdown
    let result = manager.shutdown();
    assert!(result.is_ok());
    assert!(!manager.is_initialized());
}

#[test]
fn test_plugin_error_display() {
    // Test error display
    let error = PluginError::LoadError("Test load error".to_string());
    assert_eq!(format!("{}", error), "Failed to load plugin: Test load error");
    
    let error = PluginError::NotFound("TestPlugin".to_string());
    assert_eq!(format!("{}", error), "Plugin not found: TestPlugin");
    
    let error = PluginError::InvalidFormat("Invalid format".to_string());
    assert_eq!(format!("{}", error), "Invalid plugin format: Invalid format");
    
    let error = PluginError::InitializationError("Initialization failed".to_string());
    assert_eq!(format!("{}", error), "Plugin initialization failed: Initialization failed");
}
