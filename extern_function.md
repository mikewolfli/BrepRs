# 3D Geometry Implementation List

## Point-related
- [ ] Point cloud processing: Loading, saving, filtering, sampling operations for point clouds
- [ ] Point clustering: Distance or density-based point clustering algorithms
- [ ] Point set topology analysis: Analysis of connection relationships between points

## Curve-related
- [ ] 3D Bezier curve (bezier_curve3d): Currently only 2D version exists
- [ ] 3D NURBS curve (nurbs_curve3d): Currently only 2D version exists
- [ ] Line segment (line_segment): Finite length line segment implementation
- [ ] Polyline: Continuous curve composed of multiple line segments
- [ ] B-spline curve: Spline type other than Bezier and NURBS
- [ ] Catmull-Rom spline: Interpolating spline curve

## Surface-related
- [ ] Triangle mesh: Surface representation composed of triangles
- [ ] Polygon face: Arbitrary polygon face implementation
- [ ] Mesh subdivision algorithms: More subdivision methods (e.g., Loop subdivision, Catmull-Clark subdivision)
- [ ] Surface reconstruction: Reconstructing surfaces from point clouds or other data
- [ ] Surface clipping: Clipping operations on surfaces
- [ ] UV parameterization: UV coordinate mapping for surfaces

## Solid-related
- [ ] Cube: Basic cube implementation
- [ ] Prism: Polygonal prism
- [ ] Pyramid: Polygonal pyramid
- [ ] Polyhedron: Solid composed of multiple polygonal faces
- [ ] Boolean operations: Union, intersection, difference operations between solids
- [ ] Solid decomposition: Breaking down complex solids into simpler parts
- [ ] Complete BREP implementation: More comprehensive boundary representation
- [ ] CSG representation: Constructive Solid Geometry representation
- [ ] Implicit surfaces: Surfaces defined by mathematical equations
- [ ] Fractal geometry: Generation and representation of fractal shapes

## Geometry operations-related
- [ ] Advanced intersection detection: Curve-surface, surface-surface intersections
- [ ] Collision detection: Collision detection between solids
- [ ] Distance calculation: Distance from point to surface, surface to surface
- [ ] Topological operations: Topological modifications of solids (e.g., edge splitting, face merging)
- [ ] Geometric constraint solving: Generating geometric shapes that satisfy specific constraints
- [ ] Parametric modeling: Parameter-based model generation and modification
- [ ] Mesh optimization: Improving mesh quality
- [ ] Geometric data exchange: Supporting import/export of more file formats
- [ ] Geometric validation: Checking validity of geometric models (e.g., manifoldness, self-intersections)
