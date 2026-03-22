# BrepRs WebAssembly Example

This example demonstrates how to use BrepRs in a web browser using WebAssembly.

## Prerequisites

1. Install wasm-pack:
   ```bash
   cargo install wasm-pack
   ```

2. Install a local web server (e.g., serve):
   ```bash
   npm install -g serve
   ```

## Building

Run the build script:

```bash
cd examples/wasm
chmod +x build.sh
./build.sh
```

Or build manually:

```bash
wasm-pack build --target web --out-dir examples/wasm/pkg --scope breprs
```

## Running

1. Start a local web server:
   ```bash
   cd examples/wasm
   serve .
   ```

2. Open your browser and navigate to:
   ```
   http://localhost:3000
   ```

## Features

### Geometry Operations
- Create points, vectors, and planes
- Perform geometric calculations

### Primitives
- Box
- Sphere
- Cylinder
- Torus

### Boolean Operations
- Fuse (union)
- Cut (difference)
- Intersect

### Mesh Generation
- Generate triangle meshes for primitives
- Calculate bounding boxes

### File I/O
- Export to STL, OBJ formats
- Import from STL, OBJ, STEP, IGES formats

### Internationalization
- Support for multiple languages (English, Chinese, French, German)
- Runtime language switching

## API Usage

```javascript
import init, { Box, Sphere, BooleanOperations } from './pkg/breprs_wasm.js';

async function main() {
    await init();
    
    // Create shapes
    const box = new Box(10.0, 10.0, 10.0);
    const sphere = new Sphere(5.0);
    
    // Get properties
    console.log(`Box volume: ${box.volume()}`);
    console.log(`Sphere volume: ${sphere.volume()}`);
    
    // Boolean operations
    // const result = BooleanOperations.fuse(box, sphere);
}
```

## Browser Compatibility

This example requires a modern browser with WebAssembly support:
- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

## Troubleshooting

### "Failed to initialize WASM"
- Make sure you've built the WASM package
- Check that the browser supports WebAssembly
- Try a different browser

### "Import error"
- Make sure you're serving the files through a web server (not opening directly)
- Check the browser console for specific error messages

### "Memory allocation failed"
- The model may be too complex for the browser's memory limits
- Try simplifying the model or increasing browser memory limits
