//! Plugin system for BrepRs
//! 
//! This module provides a plugin system that allows extending BrepRs functionality
//! through dynamically loaded plugins.

use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;
use std::sync::Arc;

/// Plugin trait that all plugins must implement
pub trait Plugin {
    /// Get the plugin name
    fn name(&self) -> &str;
    
    /// Get the plugin version
    fn version(&self) -> &str;
    
    /// Get the plugin description
    fn description(&self) -> &str;
    
    /// Initialize the plugin
    fn initialize(&mut self) -> Result<(), PluginError>;
    
    /// Shutdown the plugin
    fn shutdown(&mut self) -> Result<(), PluginError>;
}

/// Plugin error type
#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Loading error: {0}")]
    LoadingError(String),
    
    #[error("Initialization error: {0}")]
    InitializationError(String),
    
    #[error("Unsupported plugin format")]
    UnsupportedFormat,
    
    #[error("Plugin not found: {0}")]
    PluginNotFound(String),
}

/// Plugin manager for managing plugins
pub struct PluginManager {
    /// Loaded plugins
    plugins: HashMap<String, Arc<dyn Plugin + Send + Sync>>,
    /// Plugin search paths
    search_paths: Vec<String>,
}

impl PluginManager {
    /// Create a new plugin manager
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            search_paths: Vec::new(),
        }
    }
    
    /// Add a plugin search path
    pub fn add_search_path(&mut self, path: &str) {
        self.search_paths.push(path.to_string());
    }
    
    /// Get the plugin search paths
    pub fn search_paths(&self) -> &Vec<String> {
        &self.search_paths
    }
    
    /// Load a plugin from a file
    pub fn load_plugin(&mut self, path: &str) -> Result<(), PluginError> {
        // Check if the file exists
        let path = Path::new(path);
        if !path.exists() {
            return Err(PluginError::PluginNotFound(path.to_string_lossy().to_string()));
        }
        
        // Check if the file is a dynamic library
        if path.extension() != Some(OsStr::new("dll")) && 
           path.extension() != Some(OsStr::new("so")) && 
           path.extension() != Some(OsStr::new("dylib")) {
            return Err(PluginError::UnsupportedFormat);
        }
        
        // Load the plugin
        // This is a placeholder implementation
        // In a real implementation, we would use dlopen to load the dynamic library
        // and get the plugin instance
        
        // For now, we'll just return a placeholder error
        Err(PluginError::LoadingError("Plugin loading not implemented".to_string()))
    }
    
    /// Load all plugins from the search paths
    pub fn load_all_plugins(&mut self) -> Result<(), PluginError> {
        for path in &self.search_paths {
            let path = Path::new(path);
            if !path.exists() || !path.is_dir() {
                continue;
            }
            
            // Iterate through all files in the directory
            for entry in std::fs::read_dir(path).map_err(PluginError::IoError)? {
                let entry = entry.map_err(PluginError::IoError)?;
                let path = entry.path();
                
                // Check if the file is a dynamic library
                if path.extension() == Some(OsStr::new("dll")) || 
                   path.extension() == Some(OsStr::new("so")) || 
                   path.extension() == Some(OsStr::new("dylib")) {
                    // Load the plugin
                    let _ = self.load_plugin(path.to_str().unwrap());
                }
            }
        }
        
        Ok(())
    }
    
    /// Get a plugin by name
    pub fn get_plugin(&self, name: &str) -> Option<Arc<dyn Plugin + Send + Sync>> {
        self.plugins.get(name).cloned()
    }
    
    /// Get all loaded plugins
    pub fn plugins(&self) -> &HashMap<String, Arc<dyn Plugin + Send + Sync>> {
        &self.plugins
    }
    
    /// Unload a plugin
    pub fn unload_plugin(&mut self, name: &str) -> Result<(), PluginError> {
        if let Some(plugin) = self.plugins.remove(name) {
            // Shutdown the plugin
            let mut plugin = Arc::try_unwrap(plugin).map_err(|_| PluginError::LoadingError("Cannot unwrap plugin arc".to_string()))?;
            plugin.shutdown()?;
            Ok(())
        } else {
            Err(PluginError::PluginNotFound(name.to_string()))
        }
    }
    
    /// Unload all plugins
    pub fn unload_all_plugins(&mut self) -> Result<(), PluginError> {
        let plugin_names: Vec<String> = self.plugins.keys().cloned().collect();
        
        for name in plugin_names {
            self.unload_plugin(&name)?;
        }
        
        Ok(())
    }
}

/// Macro for defining a plugin
#[macro_export]
macro_rules! define_plugin {
    ($struct:ident, $name:expr, $version:expr, $description:expr) => {
        pub struct $struct {
            initialized: bool,
        }
        
        impl $struct {
            pub fn new() -> Self {
                Self {
                    initialized: false,
                }
            }
        }
        
        impl Plugin for $struct {
            fn name(&self) -> &str {
                $name
            }
            
            fn version(&self) -> &str {
                $version
            }
            
            fn description(&self) -> &str {
                $description
            }
            
            fn initialize(&mut self) -> Result<(), PluginError> {
                if self.initialized {
                    return Ok(());
                }
                
                // Initialize the plugin
                self.initialized = true;
                Ok(())
            }
            
            fn shutdown(&mut self) -> Result<(), PluginError> {
                if !self.initialized {
                    return Ok(());
                }
                
                // Shutdown the plugin
                self.initialized = false;
                Ok(())
            }
        }
        
        // Export the plugin creation function
        #[no_mangle]
        pub extern "C" fn create_plugin() -> *mut dyn Plugin {
            let plugin = Box::new($struct::new());
            Box::into_raw(plugin)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_plugin_manager_creation() {
        let manager = PluginManager::new();
        assert_eq!(manager.search_paths().len(), 0);
        assert_eq!(manager.plugins().len(), 0);
    }
    
    #[test]
    fn test_add_search_path() {
        let mut manager = PluginManager::new();
        manager.add_search_path("plugins");
        assert_eq!(manager.search_paths().len(), 1);
        assert_eq!(manager.search_paths()[0], "plugins");
    }
    
    #[test]
    fn test_load_plugin() {
        let mut manager = PluginManager::new();
        let result = manager.load_plugin("non_existent.dll");
        assert!(result.is_err());
        match result.unwrap_err() {
            PluginError::PluginNotFound(_) => (),
            _ => panic!("Expected PluginNotFound error"),
        }
    }
    
    #[test]
    fn test_get_plugin() {
        let manager = PluginManager::new();
        let plugin = manager.get_plugin("test");
        assert!(plugin.is_none());
    }
    
    #[test]
    fn test_unload_plugin() {
        let mut manager = PluginManager::new();
        let result = manager.unload_plugin("test");
        assert!(result.is_err());
        match result.unwrap_err() {
            PluginError::PluginNotFound(_) => (),
            _ => panic!("Expected PluginNotFound error"),
        }
    }
}
