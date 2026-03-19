use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsStr;
use std::fmt;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

/// Plugin error type
#[derive(Debug)]
pub enum PluginError {
    /// Failed to load plugin
    LoadError(String),
    /// Plugin not found
    NotFound(String),
    /// Invalid plugin format
    InvalidFormat(String),
    /// Plugin initialization failed
    InitializationError(String),
}

impl fmt::Display for PluginError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PluginError::LoadError(msg) => write!(f, "Failed to load plugin: {}", msg),
            PluginError::NotFound(name) => write!(f, "Plugin not found: {}", name),
            PluginError::InvalidFormat(msg) => write!(f, "Invalid plugin format: {}", msg),
            PluginError::InitializationError(msg) => {
                write!(f, "Plugin initialization failed: {}", msg)
            }
        }
    }
}

impl Error for PluginError {}

/// Plugin interface trait
pub trait Plugin: Send + Sync {
    /// Get plugin name
    fn name(&self) -> &str;
    /// Get plugin version
    fn version(&self) -> &str;
    /// Get plugin description
    fn description(&self) -> &str;
    /// Initialize plugin
    fn initialize(&mut self) -> Result<(), PluginError>;
    /// Shutdown plugin
    fn shutdown(&mut self) -> Result<(), PluginError>;
    /// Check if plugin is initialized
    fn is_initialized(&self) -> bool;
}

/// Plugin manager
pub struct PluginManager {
    plugins: HashMap<String, Arc<Mutex<dyn Plugin + Send + Sync>>>,
    plugin_paths: Vec<String>,
    initialized: bool,
}

impl PluginManager {
    /// Create a new plugin manager
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            plugin_paths: Vec::new(),
            initialized: false,
        }
    }

    /// Add plugin search path
    pub fn add_plugin_path(&mut self, path: &str) {
        self.plugin_paths.push(path.to_string());
    }

    /// Initialize plugin manager
    pub fn initialize(&mut self) -> Result<(), PluginError> {
        if self.initialized {
            return Ok(());
        }

        // Load plugins from all search paths - clone to avoid borrow conflicts
        let paths = self.plugin_paths.clone();
        for path in &paths {
            self.load_plugins_from_path(path)?;
        }

        // Initialize all plugins - collect errors first to avoid borrow conflicts
        let mut init_errors = Vec::new();
        for (name, plugin) in &self.plugins {
            let mut plugin_mut = plugin.lock().unwrap();
            if let Err(e) = plugin_mut.initialize() {
                init_errors.push(PluginError::InitializationError(format!(
                    "Plugin {} initialization failed: {}",
                    name, e
                )));
            }
        }

        if !init_errors.is_empty() {
            return Err(init_errors.into_iter().next().unwrap());
        }

        self.initialized = true;
        Ok(())
    }

    /// Load plugins from a directory
    fn load_plugins_from_path(&mut self, path: &str) -> Result<(), PluginError> {
        let path = Path::new(path);
        if !path.exists() || !path.is_dir() {
            return Ok(()); // Directory doesn't exist, skip
        }

        // Iterate through files in the directory
        for entry in fs::read_dir(path).map_err(|e| PluginError::LoadError(e.to_string()))? {
            let entry = entry.map_err(|e| PluginError::LoadError(e.to_string()))?;
            let path = entry.path();

            // Check if it's a dynamic library
            if path.is_file() && is_dynamic_library(&path) {
                self.load_plugin(&path)?;
            }
        }

        Ok(())
    }

    /// Load a single plugin
    fn load_plugin(&mut self, path: &Path) -> Result<(), PluginError> {
        // In a real implementation, this would use libloading to load the dynamic library
        // For now, we'll just simulate loading
        let plugin_name = path
            .file_stem()
            .unwrap_or_else(|| OsStr::new("unknown"))
            .to_str()
            .unwrap_or("unknown");

        // Create a dummy plugin for demonstration
        struct DummyPlugin {
            name: String,
            version: String,
            description: String,
            initialized: bool,
        }

        impl Plugin for DummyPlugin {
            fn name(&self) -> &str {
                &self.name
            }

            fn version(&self) -> &str {
                &self.version
            }

            fn description(&self) -> &str {
                &self.description
            }

            fn initialize(&mut self) -> Result<(), PluginError> {
                self.initialized = true;
                Ok(())
            }

            fn shutdown(&mut self) -> Result<(), PluginError> {
                self.initialized = false;
                Ok(())
            }

            fn is_initialized(&self) -> bool {
                self.initialized
            }
        }

        let dummy_plugin = DummyPlugin {
            name: plugin_name.to_string(),
            version: "1.0.0".to_string(),
            description: format!("Dummy plugin for {}", plugin_name),
            initialized: false,
        };

        self.plugins
            .insert(plugin_name.to_string(), Arc::new(Mutex::new(dummy_plugin)));
        Ok(())
    }

    /// Get a plugin by name
    pub fn get_plugin(&self, name: &str) -> Option<Arc<Mutex<dyn Plugin + Send + Sync>>> {
        self.plugins.get(name).cloned()
    }

    /// Get all plugins
    pub fn get_plugins(&self) -> &HashMap<String, Arc<Mutex<dyn Plugin + Send + Sync>>> {
        &self.plugins
    }

    /// Shutdown plugin manager
    pub fn shutdown(&mut self) -> Result<(), PluginError> {
        if !self.initialized {
            return Ok(());
        }

        // Shutdown all plugins
        for (name, plugin) in &self.plugins {
            let mut plugin_mut = plugin.lock().unwrap();
            if let Err(e) = plugin_mut.shutdown() {
                return Err(PluginError::InitializationError(format!(
                    "Plugin {} shutdown failed: {}",
                    name, e
                )));
            }
        }

        self.plugins.clear();
        self.initialized = false;
        Ok(())
    }

    /// Check if plugin manager is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}

/// Check if a file is a dynamic library
fn is_dynamic_library(path: &Path) -> bool {
    #[cfg(windows)]
    let extensions = ["dll"];
    #[cfg(all(unix, not(target_os = "macos")))]
    let extensions = ["so"];
    #[cfg(target_os = "macos")]
    let extensions = ["dylib"];

    if let Some(ext) = path.extension() {
        extensions.contains(&ext.to_str().unwrap_or(""))
    } else {
        false
    }
}

/// Plugin registry trait for static plugins
pub trait PluginRegistry {
    /// Register plugins
    fn register_plugins(manager: &mut PluginManager);
}

/// Macro to register a plugin
#[macro_export]
macro_rules! register_plugin {
    ($plugin:ty) => {
        #[no_mangle]
        pub fn register_plugin(manager: &mut $crate::plugin::PluginManager) {
            let plugin = Box::new(<$plugin as Default>::default());
            manager
                .plugins
                .insert(plugin.name().to_string(), std::sync::Arc::new(plugin));
        }
    };
}
