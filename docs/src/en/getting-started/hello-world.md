# Hello World

This section provides a step-by-step guide to creating your first BrepRs application.

## Rust Hello World

### Step 1: Create a New Rust Project

```bash
# Create a new Rust project
cargo new breprs-hello
cd breprs-hello
```

### Step 2: Add BrepRs as a Dependency

Edit `Cargo.toml` to add BrepRs:

```toml
[dependencies]
breprs = "0.1.0"
```

### Step 3: Write Your First Program

Edit `src/main.rs`:

```rust
use breprs::geometry::Point;
use breprs::modeling::primitives::make_box;
use breprs::i18n::{I18n, Language};

fn main() {
    println!("BrepRs Hello World!");
    println!("==================");

    // Initialize internationalization
    I18n::init();
    println!("Current language: {}", I18n::current_language().code());

    // Set language to English
    I18n::set_language(Language::English);
    println!("Language set to English");

    // Translate some messages
    println!("\nTranslations:");
    println!("  ErrorUnknown: {}", I18n::tr(breprs::i18n::MessageKey::ErrorUnknown));
    println!("  LabelFile: {}", I18n::tr(breprs::i18n::MessageKey::LabelFile));
    println!("  OpBooleanFuse: {}", I18n::tr(breprs::i18n::MessageKey::OpBooleanFuse));

    // Create a box
    let box_shape = make_box(1.0, 1.0, 1.0, None);
    println!("\nCreated box shape");
    println!("Shape type: {:?}", box_shape.shape_type());
    println!("Shape is null: {}", box_shape.is_null());

    // Create a box at a specific location
    let origin = Point::new(1.0, 1.0, 1.0);
    let box_at_origin = make_box(1.0, 1.0, 1.0, Some(origin));
    println!("\nCreated box at origin ({}, {}, {})", origin.x, origin.y, origin.z);
    println!("Shape type: {:?}", box_at_origin.shape_type());

    println!("\nHello World completed successfully!");
}
```

### Step 4: Run Your Program

```bash
cargo run
```

You should see output like:

```
BrepRs Hello World!
==================
Current language: en
Language set to English

Translations:
  ErrorUnknown: Unknown error
  LabelFile: File
  OpBooleanFuse: Fuse

Created box shape
Shape type: Solid
Shape is null: false

Created box at origin (1, 1, 1)
Shape type: Solid

Hello World completed successfully!
```

## Python Hello World

### Step 1: Set Up Python Environment

```bash
# Create a virtual environment
python3 -m venv venv
source venv/bin/activate

# Install maturin
pip install maturin
```

### Step 2: Build and Install BrepRs Python Bindings

```bash
# Navigate to bindings/python
cd path/to/breprs/bindings/python

# Build and install
maturin develop --release
```

### Step 3: Create Python Script

Create `hello.py`:

```python
from breprs_python import I18n, Point, make_box

print("BrepRs Python Hello World!")
print("=========================")

# Initialize i18n
I18n.init()
print(f"Current language: {I18n.current_language()}")

# Set language to English
I18n.set_language("en")
print("Language set to English")

# Translate some messages
print("\nTranslations:")
print(f"  ErrorUnknown: {I18n.translate('ErrorUnknown')}")
print(f"  LabelFile: {I18n.translate('LabelFile')}")
print(f"  OpBooleanFuse: {I18n.translate('OpBooleanFuse')}")

# Create a box
origin = Point(0.0, 0.0, 0.0)
box = make_box(1.0, 1.0, 1.0, origin)
print("\nCreated box shape")
print(f"Shape type: {box.shape_type()}")
print(f"Shape is null: {box.is_null()}")

print("\nHello World completed successfully!")
```

### Step 4: Run the Script

```bash
python hello.py
```

## Node.js Hello World

### Step 1: Set Up Node.js Environment

```bash
# Navigate to bindings/nodejs
cd path/to/breprs/bindings/nodejs

# Install dependencies
npm install

# Build the binding
npm run build
```

### Step 2: Create Node.js Script

Create `hello.js`:

```javascript
const breprs = require('./build/Release/breprs_nodejs');

console.log('BrepRs Node.js Hello World!');
console.log('==========================');

// Initialize i18n
breprs.i18nInit();
console.log(`Current language: ${breprs.i18nCurrentLanguage()}`);

// Set language to English
breprs.i18nSetLanguage('en');
console.log('Language set to English');

// Translate some messages
console.log('\nTranslations:');
console.log(`  ErrorUnknown: ${breprs.i18nTranslate('ErrorUnknown')}`);
console.log(`  LabelFile: ${breprs.i18nTranslate('LabelFile')}`);
console.log(`  OpBooleanFuse: ${breprs.i18nTranslate('OpBooleanFuse')}`);

// Get available languages
const languages = breprs.i18nAvailableLanguages();
console.log('\nAvailable languages:', languages);

console.log('\nHello World completed successfully!');
```

### Step 3: Run the Script

```bash
node hello.js
```

## Next Steps

Congratulations! You've created your first BrepRs application. Here are some next steps:

1. **Explore More Examples** - Check out the [Examples](../examples.md) section
2. **Learn Core Concepts** - Understand geometry and topology in BrepRs
3. **Read the API Reference** - Explore the complete API
4. **Build a Real Application** - Start developing your own project with BrepRs
