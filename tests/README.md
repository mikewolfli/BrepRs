# Test Directory Structure

This directory contains tests for the BrepRs project.

## Test Categories
- `unit/` - Unit tests for individual components
- `integration/` - Integration tests for component interactions
- `performance/` - Performance benchmarks
- `stress/` - Stress testing scenarios

## Running Tests

```bash
# Run all tests
cargo test

# Run specific test category
cargo test --test unit
cargo test --test integration
cargo test --test performance
cargo test --test stress
```

## Test Coverage

To generate test coverage reports:

```bash
cargo tarpaulin --out Html
```