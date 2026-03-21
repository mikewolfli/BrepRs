use std::collections::HashMap;
use std::time::Instant;

/// Benchmark category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BenchmarkCategory {
    /// Geometry operations
    Geometry,
    /// Topology operations
    Topology,
    /// Mesh operations
    Mesh,
    /// Boolean operations
    Boolean,
    /// AI/ML operations
    AIML,
    /// Rendering operations
    Rendering,
    /// Other operations
    Other,
}

/// Benchmark result
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub name: String,
    pub category: BenchmarkCategory,
    pub execution_time_ms: f64,
    pub memory_used_mb: f64,
    pub operations_per_second: f64,
    pub iterations: usize,
    pub success: bool,
    pub error_message: Option<String>,
}

/// Benchmark suite
pub struct BenchmarkSuite {
    pub name: String,
    pub results: Vec<BenchmarkResult>,
    pub average_times: HashMap<BenchmarkCategory, f64>,
    pub total_time_ms: f64,
}

impl BenchmarkSuite {
    /// Create a new benchmark suite
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            results: Vec::new(),
            average_times: HashMap::new(),
            total_time_ms: 0.0,
        }
    }

    /// Run a benchmark
    pub fn run_benchmark<F>(
        &mut self,
        name: &str,
        category: BenchmarkCategory,
        iterations: usize,
        mut benchmark_fn: F,
    ) -> BenchmarkResult
    where
        F: FnMut() -> Result<(), String>,
    {
        let start_time = Instant::now();
        let start_memory = self.get_memory_usage();

        let mut success = true;
        let mut error_message = None;

        for _ in 0..iterations {
            if let Err(e) = benchmark_fn() {
                success = false;
                error_message = Some(e);
                break;
            }
        }

        let end_time = Instant::now();
        let end_memory = self.get_memory_usage();
        let execution_time_ms = end_time.duration_since(start_time).as_millis() as f64;
        let memory_used_mb = (end_memory - start_memory) as f64 / (1024.0 * 1024.0);
        let operations_per_second = iterations as f64 / (execution_time_ms / 1000.0);

        let result = BenchmarkResult {
            name: name.to_string(),
            category,
            execution_time_ms,
            memory_used_mb,
            operations_per_second,
            iterations,
            success,
            error_message,
        };

        self.results.push(result.clone());
        self.total_time_ms += execution_time_ms;

        // Update average times
        let current_average = self.average_times.entry(category).or_insert(0.0);
        *current_average = (*current_average * (self.results.len() - 1) as f64 + execution_time_ms)
            / self.results.len() as f64;

        result
    }

    /// Get memory usage
    fn get_memory_usage(&self) -> usize {
        // Implementation to get memory usage
        // This is platform-dependent
        #[cfg(target_os = "linux")]
        {
            use std::fs::File;
            use std::io::{BufRead, BufReader};
            if let Ok(file) = File::open("/proc/self/status") {
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    if let Ok(l) = line {
                        if l.starts_with("VmRSS:") {
                            let parts: Vec<&str> = l.split_whitespace().collect();
                            if parts.len() >= 2 {
                                if let Ok(kb) = parts[1].parse::<usize>() {
                                    return kb * 1024;
                                }
                            }
                        }
                    }
                }
            }
        }
        #[cfg(target_os = "macos")]
        {
            // Simplified macOS memory usage estimation
            // Using a simple heuristic based on process statistics
            if let Ok(output) = std::process::Command::new("ps")
                .args(["-o", "rss=", "-p", &std::process::id().to_string()])
                .output()
            {
                if let Ok(rss) = String::from_utf8_lossy(&output.stdout).trim().parse::<usize>() {
                    return rss * 1024; // Convert from KB to bytes
                }
            }
        }
        #[cfg(target_os = "windows")]
        {
            use winapi::um::psapi::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};
            use winapi::um::processthreadsapi::GetCurrentProcess;
            use std::mem::size_of;
            let mut counters = PROCESS_MEMORY_COUNTERS {
                cb: size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
                ..unsafe { std::mem::zeroed() }
            };
            unsafe {
                if GetProcessMemoryInfo(
                    GetCurrentProcess(),
                    &mut counters,
                    size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
                ) != 0
                {
                    return counters.WorkingSetSize as usize;
                }
            }
        }
        // Fallback: return 0 if not supported
        0
    }

    /// Print results
    pub fn print_results(&self) {
        println!("Benchmark Suite: {}", self.name);
        println!("Total time: {:.2} ms", self.total_time_ms);
        println!("Average times by category:");

        for (category, average_time) in &self.average_times {
            println!("  {:?}: {:.2} ms", category, average_time);
        }

        println!("\nDetailed results:");
        for result in &self.results {
            println!("  {}:", result.name);
            println!("    Category: {:?}", result.category);
            println!("    Execution time: {:.2} ms", result.execution_time_ms);
            println!("    Memory used: {:.2} MB", result.memory_used_mb);
            println!(
                "    Operations per second: {:.2}",
                result.operations_per_second
            );
            println!("    Iterations: {}", result.iterations);
            println!("    Success: {}", result.success);
            if let Some(error) = &result.error_message {
                println!("    Error: {}", error);
            }
            println!();
        }
    }

    /// Get best performing benchmark by category
    pub fn get_best_performer(&self, category: BenchmarkCategory) -> Option<&BenchmarkResult> {
        self.results
            .iter()
            .filter(|r| r.category == category && r.success)
            .min_by(|a, b| {
                a.execution_time_ms
                    .partial_cmp(&b.execution_time_ms)
                    .unwrap()
            })
    }

    /// Get worst performing benchmark by category
    pub fn get_worst_performer(&self, category: BenchmarkCategory) -> Option<&BenchmarkResult> {
        self.results
            .iter()
            .filter(|r| r.category == category && r.success)
            .max_by(|a, b| {
                a.execution_time_ms
                    .partial_cmp(&b.execution_time_ms)
                    .unwrap()
            })
    }
}

/// Performance analyzer
pub struct PerformanceAnalyzer {
    pub benchmarks: Vec<BenchmarkSuite>,
    pub optimization_suggestions: Vec<String>,
}

impl PerformanceAnalyzer {
    /// Create a new performance analyzer
    pub fn new() -> Self {
        Self {
            benchmarks: Vec::new(),
            optimization_suggestions: Vec::new(),
        }
    }

    /// Add benchmark suite
    pub fn add_benchmark_suite(&mut self, suite: BenchmarkSuite) {
        self.benchmarks.push(suite);
    }

    /// Analyze performance
    pub fn analyze(&mut self) {
        self.optimization_suggestions.clear();

        // Analyze each benchmark suite
        for suite in &self.benchmarks {
            for result in &suite.results {
                if !result.success {
                    self.optimization_suggestions.push(format!(
                        "Fix error in benchmark '{}': {}",
                        result.name,
                        result.error_message.as_ref().unwrap()
                    ));
                }

                // Check for performance issues
                if result.execution_time_ms > 1000.0 {
                    self.optimization_suggestions.push(format!(
                        "Optimize '{}' - execution time is {:.2} ms",
                        result.name, result.execution_time_ms
                    ));
                }

                if result.memory_used_mb > 100.0 {
                    self.optimization_suggestions.push(format!(
                        "Reduce memory usage in '{}' - using {:.2} MB",
                        result.name, result.memory_used_mb
                    ));
                }
            }
        }

        // Analyze category averages
        let mut category_averages = HashMap::new();
        for suite in &self.benchmarks {
            for (category, average_time) in &suite.average_times {
                let current_average = category_averages.entry(category).or_insert(Vec::new());
                current_average.push(*average_time);
            }
        }

        // Find categories with high average times
        for (category, times) in category_averages {
            let avg = times.iter().sum::<f64>() / times.len() as f64;
            if avg > 500.0 {
                self.optimization_suggestions.push(format!(
                    "Optimize {} operations - average time is {:.2} ms",
                    category.as_str(),
                    avg
                ));
            }
        }
    }

    /// Get optimization suggestions
    pub fn get_optimization_suggestions(&self) -> &Vec<String> {
        &self.optimization_suggestions
    }

    /// Print analysis results
    pub fn print_analysis(&self) {
        println!("Performance Analysis Results");
        println!("============================");

        println!("\nOptimization Suggestions:");
        if self.optimization_suggestions.is_empty() {
            println!("  No optimization suggestions.");
        } else {
            for (i, suggestion) in self.optimization_suggestions.iter().enumerate() {
                println!("  {}. {}", i + 1, suggestion);
            }
        }

        println!("\nBenchmark Suites:");
        for (i, suite) in self.benchmarks.iter().enumerate() {
            println!("  {}. {}", i + 1, suite.name);
            println!("     Total time: {:.2} ms", suite.total_time_ms);
            println!("     Benchmarks: {}", suite.results.len());
        }
    }
}

/// Helper methods for BenchmarkCategory
impl BenchmarkCategory {
    /// Convert to string
    pub fn as_str(&self) -> &str {
        match self {
            BenchmarkCategory::Geometry => "Geometry",
            BenchmarkCategory::Topology => "Topology",
            BenchmarkCategory::Mesh => "Mesh",
            BenchmarkCategory::Boolean => "Boolean",
            BenchmarkCategory::AIML => "AI/ML",
            BenchmarkCategory::Rendering => "Rendering",
            BenchmarkCategory::Other => "Other",
        }
    }
}

/// Benchmarking macros
#[macro_export]
macro_rules! benchmark {
    ($suite:expr, $name:expr, $category:expr, $iterations:expr, $code:block) => {
        $suite.run_benchmark($name, $category, $iterations, || {
            $code;
            Ok(())
        })
    };
}

#[macro_export]
macro_rules! benchmark_result {
    ($suite:expr, $name:expr, $category:expr, $iterations:expr, $code:expr) => {
        $suite.run_benchmark($name, $category, $iterations, || $code)
    };
}
