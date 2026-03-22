use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, SystemTime};

/// Hot reload manager
pub struct HotReloadManager {
    /// Watched files and their last modified times
    watched_files: Arc<RwLock<HashMap<PathBuf, SystemTime>>>,
    /// Files that have changed
    changed_files: Arc<Mutex<HashSet<PathBuf>>>,
    /// Hot reload enabled
    enabled: bool,
    /// Watch interval
    watch_interval: Duration,
    /// Callback for when files change
    change_callback: Option<Arc<dyn Fn(&[PathBuf]) + Send + Sync>>,
}

impl HotReloadManager {
    /// Create a new hot reload manager
    pub fn new() -> Self {
        Self {
            watched_files: Arc::new(RwLock::new(HashMap::new())),
            changed_files: Arc::new(Mutex::new(HashSet::new())),
            enabled: false,
            watch_interval: Duration::from_secs(1),
            change_callback: None,
        }
    }

    /// Set watch interval
    pub fn with_watch_interval(mut self, interval: Duration) -> Self {
        self.watch_interval = interval;
        self
    }

    /// Set change callback
    pub fn with_change_callback<F>(&mut self, callback: F)
    where
        F: Fn(&[PathBuf]) + Send + Sync + 'static,
    {
        self.change_callback = Some(Arc::new(callback));
    }

    /// Enable hot reload
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable hot reload
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Add file to watch
    pub fn add_file(&self, path: &Path) {
        if let Ok(metadata) = fs::metadata(path) {
            if let Ok(modified) = metadata.modified() {
                self.watched_files
                    .write()
                    .unwrap()
                    .insert(path.to_path_buf(), modified);
            }
        }
    }

    /// Add directory to watch
    pub fn add_directory(&self, path: &Path) {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let entry_path = entry.path();
                    if entry_path.is_file() {
                        self.add_file(&entry_path);
                    } else if entry_path.is_dir() {
                        self.add_directory(&entry_path);
                    }
                }
            }
        }
    }

    /// Remove file from watch
    pub fn remove_file(&self, path: &Path) {
        self.watched_files.write().unwrap().remove(path);
    }

    /// Clear all watched files
    pub fn clear(&self) {
        self.watched_files.write().unwrap().clear();
        self.changed_files.lock().unwrap().clear();
    }

    /// Check for changes
    pub fn check_changes(&self) -> Vec<PathBuf> {
        if !self.enabled {
            return Vec::new();
        }

        let mut changed = Vec::new();
        let mut watched = self.watched_files.write().unwrap();
        let mut changed_files = self.changed_files.lock().unwrap();

        for (path, last_modified) in watched.iter_mut() {
            if let Ok(metadata) = fs::metadata(path) {
                if let Ok(current_modified) = metadata.modified() {
                    if current_modified > *last_modified {
                        *last_modified = current_modified;
                        changed.push(path.clone());
                        changed_files.insert(path.clone());
                    }
                }
            }
        }

        // Call callback if provided
        if !changed.is_empty() && self.change_callback.is_some() {
            let callback = self.change_callback.as_ref().unwrap();
            callback(&changed);
        }

        changed
    }

    /// Get changed files
    pub fn get_changed_files(&self) -> Vec<PathBuf> {
        self.changed_files.lock().unwrap().iter().cloned().collect()
    }

    /// Clear changed files
    pub fn clear_changed_files(&self) {
        self.changed_files.lock().unwrap().clear();
    }

    /// Start watching
    pub fn start_watching(&self) {
        let watched_files = self.watched_files.clone();
        let changed_files = self.changed_files.clone();
        let interval = self.watch_interval;
        let callback = self.change_callback.clone();

        std::thread::spawn(move || loop {
            std::thread::sleep(interval);

            let mut changed = Vec::new();
            let mut watched = watched_files.write().unwrap();
            let mut changed_set = changed_files.lock().unwrap();

            for (path, last_modified) in watched.iter_mut() {
                if let Ok(metadata) = fs::metadata(path) {
                    if let Ok(current_modified) = metadata.modified() {
                        if current_modified > *last_modified {
                            *last_modified = current_modified;
                            changed.push(path.clone());
                            changed_set.insert(path.clone());
                        }
                    }
                }
            }

            if !changed.is_empty() && callback.is_some() {
                let callback = callback.as_ref().unwrap();
                callback(&changed);
            }
        });
    }
}

/// Incremental compiler
pub struct IncrementalCompiler {
    /// Hot reload manager
    hot_reload_manager: HotReloadManager,
    /// Compilation cache
    compilation_cache: Arc<RwLock<HashMap<PathBuf, CompilationResult>>>,
    /// Build output directory
    output_dir: PathBuf,
    /// Source directory
    source_dir: PathBuf,
}

/// Compilation result
#[derive(Clone)]
pub struct CompilationResult {
    /// Compiled artifact path
    artifact_path: PathBuf,
    /// Compilation time
    compilation_time: SystemTime,
    /// Dependencies
    dependencies: HashSet<PathBuf>,
    /// Success status
    success: bool,
    /// Error message
    error: Option<String>,
}

impl CompilationResult {
    /// Create a new compilation result
    pub fn new(
        artifact_path: PathBuf,
        dependencies: HashSet<PathBuf>,
        success: bool,
        error: Option<String>,
    ) -> Self {
        Self {
            artifact_path,
            compilation_time: SystemTime::now(),
            dependencies,
            success,
            error,
        }
    }

    /// Get artifact path
    pub fn artifact_path(&self) -> &PathBuf {
        &self.artifact_path
    }

    /// Get compilation time
    pub fn compilation_time(&self) -> SystemTime {
        self.compilation_time
    }

    /// Get dependencies
    pub fn dependencies(&self) -> &HashSet<PathBuf> {
        &self.dependencies
    }

    /// Is successful
    pub fn is_success(&self) -> bool {
        self.success
    }

    /// Get error message
    pub fn error(&self) -> Option<&String> {
        self.error.as_ref()
    }
}

impl IncrementalCompiler {
    /// Create a new incremental compiler
    pub fn new(source_dir: &Path, output_dir: &Path) -> Self {
        let hot_reload_manager = HotReloadManager::new();

        Self {
            hot_reload_manager,
            compilation_cache: Arc::new(RwLock::new(HashMap::new())),
            output_dir: output_dir.to_path_buf(),
            source_dir: source_dir.to_path_buf(),
        }
    }

    /// Set hot reload enabled
    pub fn with_hot_reload(mut self, enabled: bool) -> Self {
        if enabled {
            self.hot_reload_manager.enable();
        } else {
            self.hot_reload_manager.disable();
        }
        self
    }

    /// Add file to watch
    pub fn add_file(&self, path: &Path) {
        self.hot_reload_manager.add_file(path);
    }

    /// Add directory to watch
    pub fn add_directory(&self, path: &Path) {
        self.hot_reload_manager.add_directory(path);
    }

    /// Compile file incrementally
    pub fn compile_file(&self, path: &Path) -> Result<CompilationResult, String> {
        // Check if file is in cache and up to date
        let cache = self.compilation_cache.read().unwrap();
        if let Some(result) = cache.get(path) {
            let mut up_to_date = true;

            // Check if file has changed
            if let Ok(metadata) = fs::metadata(path) {
                if let Ok(modified) = metadata.modified() {
                    if modified > result.compilation_time {
                        up_to_date = false;
                    }
                }
            }

            // Check if dependencies have changed
            if up_to_date {
                for dep in result.dependencies() {
                    if let Ok(metadata) = fs::metadata(dep) {
                        if let Ok(modified) = metadata.modified() {
                            if modified > result.compilation_time {
                                up_to_date = false;
                                break;
                            }
                        }
                    }
                }
            }

            if up_to_date {
                return Ok(result.clone());
            }
        }
        drop(cache);

        // Compile file
        let result = self.compile_file_impl(path)?;

        // Update cache
        let mut cache = self.compilation_cache.write().unwrap();
        cache.insert(path.to_path_buf(), result.clone());

        Ok(result)
    }

    /// Compile file implementation
    fn compile_file_impl(&self, path: &Path) -> Result<CompilationResult, String> {
        // Simplified compilation implementation
        // In a real system, this would use the actual compiler

        // Create output directory if it doesn't exist
        if !self.output_dir.exists() {
            fs::create_dir_all(&self.output_dir)
                .map_err(|e| format!("Failed to create output directory: {}", e))?;
        }

        // Generate artifact path
        let relative_path = path
            .strip_prefix(&self.source_dir)
            .map_err(|e| format!("File not in source directory: {}", e))?;
        let artifact_path = self
            .output_dir
            .join(relative_path)
            .with_extension("compiled");

        // Create parent directory for artifact
        if let Some(parent) = artifact_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create artifact directory: {}", e))?;
            }
        }

        // Read source file
        let mut source_file =
            File::open(path).map_err(|e| format!("Failed to open source file: {}", e))?;
        let mut source = String::new();
        source_file
            .read_to_string(&mut source)
            .map_err(|e| format!("Failed to read source file: {}", e))?;

        // Simulate compilation
        std::thread::sleep(Duration::from_millis(100));

        // Write compiled artifact
        let mut artifact_file = File::create(&artifact_path)
            .map_err(|e| format!("Failed to create artifact file: {}", e))?;
        writeln!(artifact_file, "Compiled: {}", path.display())
            .map_err(|e| format!("Failed to write artifact file: {}", e))?;
        writeln!(artifact_file, "Source length: {}", source.len())
            .map_err(|e| format!("Failed to write artifact file: {}", e))?;

        // Extract dependencies (simplified)
        let dependencies = self.extract_dependencies(path, &source);

        Ok(CompilationResult::new(
            artifact_path,
            dependencies,
            true,
            None,
        ))
    }

    /// Extract dependencies from source
    fn extract_dependencies(&self, path: &Path, source: &str) -> HashSet<PathBuf> {
        let mut dependencies = HashSet::new();

        // Get the directory of the current file for relative imports
        let current_dir = path.parent().unwrap_or(&self.source_dir);

        // Parse Rust use statements
        for line in source.lines() {
            let trimmed = line.trim();
            
            // Skip comments
            if trimmed.starts_with("//") || trimmed.starts_with("/*") {
                continue;
            }

            // Handle `use` statements
            if trimmed.starts_with("use ") {
                // Extract the module path from use statement
                // Examples: use crate::module; use super::module; use self::module; use module;
                if let Some(module_path) = self.parse_use_statement(trimmed, current_dir) {
                    dependencies.insert(module_path);
                }
            }

            // Handle `mod` declarations
            if trimmed.starts_with("mod ") {
                // Extract module name and look for corresponding file
                if let Some(module_path) = self.parse_mod_declaration(trimmed, current_dir) {
                    dependencies.insert(module_path);
                }
            }

            // Handle `include!` macro
            if trimmed.starts_with("include!") {
                if let Some(include_path) = self.parse_include_macro(trimmed, current_dir) {
                    dependencies.insert(include_path);
                }
            }

            // Handle `#[path = "..."]` attributes
            if trimmed.starts_with("#[path") {
                if let Some(path_attr) = self.parse_path_attribute(trimmed, current_dir) {
                    dependencies.insert(path_attr);
                }
            }
        }

        // Also check for common dependency patterns in the file
        self.extract_implicit_dependencies(source, &mut dependencies);

        dependencies
    }

    /// Parse a use statement and return the corresponding file path
    fn parse_use_statement(&self, line: &str, current_dir: &Path) -> Option<PathBuf> {
        // Remove 'use ' prefix and trailing semicolon
        let use_content = line.strip_prefix("use ")?.trim_end_matches(';').trim();
        
        // Handle different use patterns
        let module_path = if use_content.starts_with("crate::") {
            // Absolute path from crate root
            let relative = use_content.strip_prefix("crate::")?;
            self.source_dir.join(relative.replace("::", "/") + ".rs")
        } else if use_content.starts_with("super::") {
            // Parent module
            let relative = use_content.strip_prefix("super::")?;
            let parent_dir = current_dir.parent().unwrap_or(&self.source_dir);
            parent_dir.join(relative.replace("::", "/") + ".rs")
        } else if use_content.starts_with("self::") {
            // Current module
            let relative = use_content.strip_prefix("self::")?;
            current_dir.join(relative.replace("::", "/") + ".rs")
        } else {
            // External crate or relative path
            // Check if it's a local module first
            let parts: Vec<&str> = use_content.split("::").collect();
            if !parts.is_empty() {
                let first_part = parts[0];
                // Check for local module file
                let local_mod = current_dir.join(format!("{}.rs", first_part));
                if local_mod.exists() {
                    return Some(local_mod);
                }
                // Check for local module directory
                let local_mod_dir = current_dir.join(first_part).join("mod.rs");
                if local_mod_dir.exists() {
                    return Some(local_mod_dir);
                }
            }
            return None;
        };

        // Check if the resolved path exists
        if module_path.exists() {
            Some(module_path)
        } else {
            // Try with mod.rs for directories
            let mod_path = module_path.parent()?.join(module_path.file_stem()?).join("mod.rs");
            if mod_path.exists() {
                Some(mod_path)
            } else {
                None
            }
        }
    }

    /// Parse a mod declaration and return the corresponding file path
    fn parse_mod_declaration(&self, line: &str, current_dir: &Path) -> Option<PathBuf> {
        // Extract module name from 'mod name;'
        let mod_content = line.strip_prefix("mod ")?.trim_end_matches(';').trim();
        let module_name = mod_content.split_whitespace().next()?;
        
        // Check for module file (name.rs)
        let mod_file = current_dir.join(format!("{}.rs", module_name));
        if mod_file.exists() {
            return Some(mod_file);
        }

        // Check for module directory (name/mod.rs)
        let mod_dir_file = current_dir.join(module_name).join("mod.rs");
        if mod_dir_file.exists() {
            return Some(mod_dir_file);
        }

        None
    }

    /// Parse an include! macro and return the file path
    fn parse_include_macro(&self, line: &str, current_dir: &Path) -> Option<PathBuf> {
        // Extract path from include!("path")
        let start = line.find('"')? + 1;
        let end = line[start..].find('"')? + start;
        let include_path = &line[start..end];
        
        let full_path = current_dir.join(include_path);
        if full_path.exists() {
            Some(full_path)
        } else {
            None
        }
    }

    /// Parse a #[path = "..."] attribute and return the file path
    fn parse_path_attribute(&self, line: &str, current_dir: &Path) -> Option<PathBuf> {
        // Extract path from #[path = "..."]
        let start = line.find('"')? + 1;
        let end = line[start..].find('"')? + start;
        let attr_path = &line[start..end];
        
        let full_path = current_dir.join(attr_path);
        if full_path.exists() {
            Some(full_path)
        } else {
            None
        }
    }

    /// Extract implicit dependencies from source content
    fn extract_implicit_dependencies(&self, source: &str, dependencies: &mut HashSet<PathBuf>) {
        // Look for common patterns that indicate dependencies
        let patterns = [
            "derive(",
            "derive_macro",
            "proc_macro",
        ];

        for pattern in &patterns {
            if source.contains(pattern) {
                // These might indicate proc-macro dependencies
                // In a real system, we'd resolve these properly
            }
        }

        // Check for build script dependencies
        if source.contains("build.rs") {
            let build_script = self.source_dir.join("build.rs");
            if build_script.exists() {
                dependencies.insert(build_script);
            }
        }
    }

    /// Compile all files
    pub fn compile_all(&self) -> Result<Vec<CompilationResult>, String> {
        let mut results = Vec::new();

        // Compile all watched files
        let watched = self.hot_reload_manager.watched_files.read().unwrap();
        for path in watched.keys() {
            let result = self.compile_file(path)?;
            results.push(result);
        }

        Ok(results)
    }

    /// Start incremental compilation
    pub fn start_incremental(&mut self) {
        let compiler = self.clone();

        self.hot_reload_manager
            .with_change_callback(move |changed_files| {
                println!("Detected changes in files: {:?}", changed_files);

                for file in changed_files {
                    match compiler.compile_file(file) {
                        Ok(result) => {
                            if result.is_success() {
                                println!("Successfully compiled: {}", file.display());
                            } else {
                                println!(
                                    "Compilation failed for {}: {:?}",
                                    file.display(),
                                    result.error()
                                );
                            }
                        }
                        Err(e) => {
                            println!("Error compiling {}: {}", file.display(), e);
                        }
                    }
                }
            });

        self.hot_reload_manager.start_watching();
    }
}

impl Clone for IncrementalCompiler {
    fn clone(&self) -> Self {
        Self {
            hot_reload_manager: HotReloadManager::new(),
            compilation_cache: self.compilation_cache.clone(),
            output_dir: self.output_dir.clone(),
            source_dir: self.source_dir.clone(),
        }
    }
}

/// Build system
pub struct BuildSystem {
    /// Incremental compiler
    incremental_compiler: IncrementalCompiler,
    /// Build configuration
    config: BuildConfig,
    /// Build status
    status: BuildStatus,
}

/// Build configuration
pub struct BuildConfig {
    /// Source directory
    source_dir: PathBuf,
    /// Output directory
    output_dir: PathBuf,
    /// Enable incremental compilation
    incremental: bool,
    /// Enable hot reload
    hot_reload: bool,
    /// Build mode
    mode: BuildMode,
}

/// Build mode
pub enum BuildMode {
    /// Debug mode
    Debug,
    /// Release mode
    Release,
    /// Profile mode
    Profile,
}

/// Build status
pub enum BuildStatus {
    /// Idle
    Idle,
    /// Building
    Building,
    /// Success
    Success,
    /// Failed
    Failed(String),
}

impl BuildConfig {
    /// Create a new build configuration
    pub fn new(source_dir: &Path, output_dir: &Path) -> Self {
        Self {
            source_dir: source_dir.to_path_buf(),
            output_dir: output_dir.to_path_buf(),
            incremental: true,
            hot_reload: true,
            mode: BuildMode::Debug,
        }
    }

    /// Set incremental compilation
    pub fn with_incremental(mut self, incremental: bool) -> Self {
        self.incremental = incremental;
        self
    }

    /// Set hot reload
    pub fn with_hot_reload(mut self, hot_reload: bool) -> Self {
        self.hot_reload = hot_reload;
        self
    }

    /// Set build mode
    pub fn with_mode(mut self, mode: BuildMode) -> Self {
        self.mode = mode;
        self
    }

    /// Get source directory
    pub fn source_dir(&self) -> &PathBuf {
        &self.source_dir
    }

    /// Get output directory
    pub fn output_dir(&self) -> &PathBuf {
        &self.output_dir
    }

    /// Is incremental enabled
    pub fn is_incremental(&self) -> bool {
        self.incremental
    }

    /// Is hot reload enabled
    pub fn is_hot_reload(&self) -> bool {
        self.hot_reload
    }

    /// Get build mode
    pub fn mode(&self) -> &BuildMode {
        &self.mode
    }
}

impl BuildSystem {
    /// Create a new build system
    pub fn new(config: BuildConfig) -> Self {
        let incremental_compiler =
            IncrementalCompiler::new(config.source_dir(), config.output_dir())
                .with_hot_reload(config.is_hot_reload());

        // Add source directory to watch
        incremental_compiler.add_directory(config.source_dir());

        Self {
            incremental_compiler,
            config,
            status: BuildStatus::Idle,
        }
    }

    /// Build project
    pub fn build(&mut self) -> Result<(), String> {
        self.status = BuildStatus::Building;

        match self.incremental_compiler.compile_all() {
            Ok(results) => {
                let all_success = results.iter().all(|r| r.is_success());
                if all_success {
                    self.status = BuildStatus::Success;
                    Ok(())
                } else {
                    let errors: Vec<String> = results
                        .iter()
                        .filter(|r| !r.is_success())
                        .filter_map(|r| r.error().map(|e| e.clone()))
                        .collect();
                    let error_msg = errors.join("\n");
                    self.status = BuildStatus::Failed(error_msg.clone());
                    Err(error_msg)
                }
            }
            Err(e) => {
                self.status = BuildStatus::Failed(e.clone());
                Err(e)
            }
        }
    }

    /// Start incremental build
    pub fn start_incremental(&mut self) {
        self.incremental_compiler.start_incremental();
    }

    /// Get build status
    pub fn status(&self) -> &BuildStatus {
        &self.status
    }

    /// Get build configuration
    pub fn config(&self) -> &BuildConfig {
        &self.config
    }

    /// Add file to watch
    pub fn add_file(&self, path: &Path) {
        self.incremental_compiler.add_file(path);
    }

    /// Add directory to watch
    pub fn add_directory(&self, path: &Path) {
        self.incremental_compiler.add_directory(path);
    }
}
