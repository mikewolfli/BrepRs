use crate::benchmarking::{BenchmarkCategory, BenchmarkSuite, PerformanceAnalyzer};

#[test]
fn test_benchmark_suite_basic() {
    // Create a benchmark suite
    let mut suite = BenchmarkSuite::new("Test Suite");

    // Run a simple benchmark
    let result = suite.run_benchmark("Simple Operation", BenchmarkCategory::Other, 100, || Ok(()));

    // Verify benchmark result
    assert_eq!(result.name, "Simple Operation");
    assert!(result.success);
    assert!(result.execution_time_ms > 0.0);
    assert!(result.operations_per_second > 0.0);

    // Test result retrieval
    assert!(!suite.results.is_empty());
    assert!(suite.get_best_performer(BenchmarkCategory::Other).is_some());
    assert!(suite
        .get_worst_performer(BenchmarkCategory::Other)
        .is_some());
}

#[test]
fn test_benchmark_suite_with_error() {
    // Create a benchmark suite
    let mut suite = BenchmarkSuite::new("Test Suite");

    // Run a benchmark that fails
    let result = suite.run_benchmark("Failing Operation", BenchmarkCategory::Other, 10, || {
        Err(crate::foundation::exception::Failure::runtime_error(
            "Test error",
            Some("test_benchmark_suite_with_error: Failing Operation"),
            None,
        ))
    });

    // Verify benchmark result
    assert_eq!(result.name, "Failing Operation");
    assert!(!result.success);
    assert!(result.error_message.is_some());
}

#[test]
fn test_performance_analyzer() {
    // Create a benchmark suite
    let mut suite = BenchmarkSuite::new("Test Suite");

    // Run some benchmarks
    suite.run_benchmark("Fast Operation", BenchmarkCategory::Geometry, 1000, || {
        Ok(())
    });

    suite.run_benchmark("Slow Operation", BenchmarkCategory::Topology, 10, || {
        // Simulate slow operation
        std::thread::sleep(std::time::Duration::from_millis(10));
        Ok(())
    });

    // Create performance analyzer
    let mut analyzer = PerformanceAnalyzer::new();
    analyzer.add_benchmark_suite(suite);

    // Analyze performance
    analyzer.analyze();

    // Verify analysis results
    assert!(!analyzer.benchmarks.is_empty());
    // The analyzer should generate some optimization suggestions
    // based on the slow operation
}

#[test]
fn test_benchmark_category_as_str() {
    // Test BenchmarkCategory::as_str() method
    assert_eq!(BenchmarkCategory::Geometry.as_str(), "Geometry");
    assert_eq!(BenchmarkCategory::Topology.as_str(), "Topology");
    assert_eq!(BenchmarkCategory::Mesh.as_str(), "Mesh");
    assert_eq!(BenchmarkCategory::Boolean.as_str(), "Boolean");
    assert_eq!(BenchmarkCategory::AIML.as_str(), "AI/ML");
    assert_eq!(BenchmarkCategory::Rendering.as_str(), "Rendering");
    assert_eq!(BenchmarkCategory::Other.as_str(), "Other");
}
