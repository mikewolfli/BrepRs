# BrepRS Future Development Plan

> This document records the list of features that legacy CAD kernels currently lack (or implement poorly), serving as a product roadmap for BrepRS Phase 2 "Beyond Legacy Kernels".
>
> Based on the latest information from legacy CAD kernel version 8.0.0 (released in Q1 2026).

---

## I. Legacy CAD Kernel's Core Weaknesses (Opportunities for Overtaking)

### 1. Technical Debt and Modernization Lag (The Biggest Pain Point)

Legacy CAD kernel 8.0 is working hard to address these issues, but this also exposes its previous severe deficiencies:

| Weakness Area | Legacy Kernel Status (Even 8.0 is still in transition) | BrepRS Rust Advantage |
|--------------|-----------------------------------------------|----------------------|
| Memory Safety | 60%+ bugs related to memory management (wild pointers, memory leaks) | Compiler-guaranteed memory safety, no wild pointers |
| Concurrent Programming | Just migrated to std::mutex, but C++ concurrency remains an advanced skill prone to errors | Native Send/Sync, Rayon enables parallelism in one line |
| Build System | Custom CMake logic is complex, cross-platform pitfalls | Cargo unified management, cross-platform out-of-the-box |
| Dependency Management | No official package manager, manual configuration | crates.io ecosystem, clear dependencies |
| Language Features | C++17 migration in progress, large legacy codebase | Modern Rust, no historical baggage |

### 2. Excessive API Complexity

Legacy CAD kernel's API has been criticized for years:

- **Verbose naming**: `BRepOffsetAPI_MakePipeShell` (like casting a spell)
- **Type confusion**: Custom types (`Standard_Real`) mixed with C++ native types, only unified in 8.0
- **Exception handling**: Custom exception system incompatible with modern C++, planned migration to `std::exception` in 8.0
- **Smart pointers**: Own Handle macro, not `std::shared_ptr`

### 3. Weak Support for Modern File Formats

| Format Type | Legacy Kernel Support Level | Market Demand |
|------------|-------------------|---------------|
| glTF | No native support | Standard for 3D Web, gaming, AR/VR |
| USD | No native support | Next-gen 3D exchange format promoted by NVIDIA, Apple, Pixar |
| 3MF | Basic support | 3D printing industry standard, legacy kernel support incomplete |
| FBX | None | Commonly used in gaming/animation industry |
| OBJ/STL | Available, but average performance | Basic formats, but legacy kernel's read/write performance has room for optimization |

### 4. Difficult Cross-Language Calling

Legacy CAD kernel is a C++ library, making it very difficult to call from other languages:

- **Python bindings**: Community-maintained, incomplete
- **JavaScript/WebAssembly**: Can only be compiled through emscripten with difficulty, large size, poor performance
- **Node.js, C#, Java**: Almost impossible to call directly

### 5. Almost Zero WebAssembly Support

Legacy CAD kernel cannot run in browsers, while Rust + Wasm can achieve:

- Geometry kernel running directly in browser
- Pure Web-based CAD becomes possible
- No server needed, local data processing

### 6. Algorithm-Level Optimization Space

- **Boolean operations**: Legacy kernel has them, but complex models often fail
- **Mesh generation**: Legacy kernel's BRep to mesh quality is average
- **Subdivision surfaces**: Legacy kernel basically doesn't support (only basic ones)
- **Machine learning integration**: No native support, legacy kernel's data structures cannot be fed directly to PyTorch/TensorFlow

---

## II. Features Just Added in Legacy Kernel 8.0 (Learn from them, no need to reinvent)

These are features just implemented in the latest legacy kernel version. Learn from their API design, but implement more elegantly in Rust:

| New Feature | Legacy Kernel 8.0 Implementation | Opportunity |
|------------|------------------------|-------------|
| Helix module | Helix generation | Can be implemented more concisely in Rust |
| New exchange format | New format replacing BRep | Can be directly compatible |
| Hash caching | Geometry hash caching | Rust's HashMap is inherently strong |
| constexpr support | Just added | Rust's const fn is natively supported |
| Unit testing | Just introduced gtest | Rust native integration testing |

---

## III. Feature List for BrepRS to "Go Beyond Legacy Kernels"

### Phase 1 (Can be done immediately after compatibility period)

#### 1. Rust Native API (Not a translation of C++ style)

- **Method chaining**: `shape.offset(5.0).fillet(2.0).boolean(&other)`
- **Generics + Trait bounds**: Compile-time type safety guarantees
- **Iterators**: `face.edges().filter(|e| e.length() > 1.0)`

#### 2. Python Bindings

- Use PyO3 to automatically generate Python packages
- Let Python users call the geometry kernel directly

#### 3. WebAssembly Support

- wasm-pack compiles to wasm in one command
- Run CAD algorithms in the browser

#### 4. Serialization Support

- Serde derive: One-click implementation of JSON/BSON/MessagePack and other formats
- Directly connect to databases, network transmission

### Phase 2 (Deep Innovation)

#### 1. Concurrent Geometric Algorithms

- Parallelize boolean operations
- Parallelize mesh generation
- Use Rayon: `.par_iter().map(...)`

#### 2. Native Support for Modern File Formats

- glTF export (directly connect to Three.js/Babylon.js)
- USD export (connect to NVIDIA Omniverse)
- Full 3MF support

#### 3. Machine Learning Integration

- Direct geometric data to Tensor (using tch-rs or candle)
- AI model training for feature recognition, model repair

#### 4. Incremental Compilation / Hot Reload

- Second-level recompilation after code changes
- Development experience crushes C++

---

## IV. BrepRS Development Roadmap

```text
BrepRS Development Roadmap
├── Phase 1 (1-2 years): Legacy Kernel API Compatibility
│   ├── Core topological data structures (Brep)
│   ├── Basic geometry (points, lines, surfaces)
│   ├── Boolean operations (union, intersection, difference)
│   ├── File I/O (STEP/IGES)
│   └── Basic visualization
│
├── Phase 2 (2-3 years): Rust Native Advantages
│   ├── Python bindings (PyO3)
│   ├── WebAssembly support
│   ├── Concurrent geometric algorithms
│   ├── Modern file formats (glTF/USD)
│   └── Better error handling (Result/Option)
│
└── Phase 3 (3-5 years): Beyond Legacy Kernels
    ├── AI integration (geometric feature recognition)
    ├── Cloud-native design (WebRTC streaming)
    ├── Real-time collaborative editing (CRDTs)
    └── Next-generation API designed specifically for Rust ecosystem
```

---

## V. Implementation Recommendations

1. **Current Phase**: Focus on completing basic features while maintaining compatibility with legacy kernel API
2. **Technical Preparation**: Reserve extension points during implementation for future features
3. **Community Building**: Establish a Rust CAD community to attract more contributors
4. **Performance Benchmarks**: Establish performance test benchmarks for comparison with legacy kernels
5. **Documentation First**: Comprehensive documentation is key to attracting users

---

*Note: This document is for long-term planning and will be implemented after basic features are complete.*
