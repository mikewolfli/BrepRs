# Quick Start

This section provides a quick introduction to using BrepRs in your project.

## Basic Concepts

Before diving into code, let's understand some basic concepts in BrepRs:

- **Geometry** - Represents points, vectors, curves, and surfaces
- **Topology** - Represents the connectivity of geometric entities (vertices, edges, faces, etc.)
- **Shape** - A combination of geometry and topology
- **Boolean Operations** - Operations like fuse, cut, common, and section
- **Modeling** - Creating and manipulating shapes

## Rust Example

Here's a simple example of using BrepRs in Rust:

```rust
use breprs::geometry::Point;
use breprs::topology::TopoDS_Shape;
use breprs::modeling::primitives::make_box;
use breprs::modeling::boolean::fuse;

fn main() {
    // Create two boxes
    let box1 = make_box(1.0, 1.0, 1.0, None);
    let box2 = make_box(1.0, 1.0, 1.0, Some(Point::new(0.5, 0.5, 0.5)));
    
    // Fuse the boxes
    let fused = fuse(&box1, &box2).unwrap();
    
    // Get the shape
    let shape: TopoDS_Shape = fused.into_shape();
    
    // Print shape information
    println!("Shape type: {:?}", shape.shape_type());
    println!("Shape is null: {}", shape.is_null());
}
```

## Python Example

Here's how to use BrepRs from Python:

```python
from breprs_python import I18n, Point, make_box, make_sphere

# Initialize i18n
I18n.init()

# Set language to English
I18n.set_language("en")

# Create a box
origin = Point(0.0, 0.0, 0.0)
box = make_box(1.0, 1.0, 1.0, origin)
print(f"Box created: {box}")

# Create a sphere
center = Point(0.5, 0.5, 0.5)
sphere = make_sphere(0.5, center)
print(f"Sphere created: {sphere}")
```

## Node.js Example

Here's how to use BrepRs from Node.js:

```javascript
const breprs = require('breprs');

// Initialize i18n
breprs.i18nInit();

// Set language to English
breprs.i18nSetLanguage('en');

// Translate a message
console.log('Error message:', breprs.i18nTranslate('ErrorUnknown'));

// Get available languages
const languages = breprs.i18nAvailableLanguages();
console.log('Available languages:', languages);
```

## C++ Example

Here's how to use BrepRs from C++:

```cpp
#include "breprs.h"
#include <iostream>

int main() {
    // Initialize i18n
    breprs_i18n_init();
    
    // Set language to English
    breprs_i18n_set_language("en");
    
    // Translate a message
    char* error_msg = breprs_i18n_translate("ErrorUnknown");
    std::cout << "Error message: " << error_msg << std::endl;
    breprs_free_string(error_msg);
    
    // Create a point
    BrepPoint* point = breprs_point_new(1.0, 2.0, 3.0);
    double x, y, z;
    breprs_point_get_coords(point, &x, &y, &z);
    std::cout << "Point: (" << x << ", " << y << ", " << z << ")" << std::endl;
    breprs_point_free(point);
    
    return 0;
}
```

## Java Example

Here's how to use BrepRs from Java:

```java
import com.breprs.I18n;

public class Main {
    public static void main(String[] args) {
        // Initialize i18n
        I18n.init();
        
        // Set language to English
        I18n.setLanguage("en");
        
        // Translate a message
        System.out.println("Error message: " + I18n.translate("ErrorUnknown"));
        
        // Get current language
        System.out.println("Current language: " + I18n.currentLanguage());
    }
}
```

## PHP Example

Here's how to use BrepRs from PHP:

```php
<?php
// Initialize i18n
i18n_init();

// Set language to English
i18n_set_language("en");

// Translate a message
echo "Error message: " . i18n_translate("ErrorUnknown") . "\n";

// Get available languages
$languages = i18n_available_languages();
echo "Available languages: " . implode(", ", $languages) . "\n";
?>
```

## Fortran Example

Here's how to use BrepRs from Fortran:

```fortran
use breprs_fortran
implicit none

character(len=:), allocatable :: error_msg

! Initialize i18n
call breprs_i18n_init()

! Set language to English
call breprs_i18n_set_language("en" // c_null_char())

! Translate a message
error_msg = c_to_f_string(breprs_i18n_translate("ErrorUnknown" // c_null_char()))
print *, "Error message: ", trim(error_msg)
call breprs_free_string(breprs_i18n_translate("ErrorUnknown" // c_null_char()))

end program
```

## Next Steps

After getting familiar with the basic usage, you may want to:

1. **Explore the Core Concepts** - Learn more about geometry and topology
2. **Check the API Reference** - Understand the complete API
3. **Run the Examples** - See more code samples
4. **Build Your Application** - Start developing with BrepRs
