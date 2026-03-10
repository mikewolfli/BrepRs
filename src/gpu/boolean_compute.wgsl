// Boolean Operations Compute Shader
// This shader performs boolean operations on meshes using GPU compute

struct Vertex {
    pos: vec3<f32>,
    normal: vec3<f32>,
};

struct Triangle {
    v0: Vertex,
    v1: Vertex,
    v2: Vertex,
};

struct MeshData {
    vertices: array<Vertex>,
    triangles: array<Triangle>,
};

@group(0)
@binding(0)
var<storage, read> input_mesh1: MeshData;

@group(0)
@binding(1)
var<storage, read> input_mesh2: MeshData;

@group(0)
@binding(2)
var<storage, read_write> output_mesh: MeshData;

@compute
@workgroup_size(64, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;

    // Simple union operation: combine vertices from both meshes
    // In a real implementation, this would perform actual boolean operations
    // using spatial hashing, BSP trees, or other algorithms

    let total_vertices = arrayLength(&input_mesh1.vertices) + arrayLength(&input_mesh2.vertices);

    if (index < total_vertices) {
        if (index < arrayLength(&input_mesh1.vertices)) {
            output_mesh.vertices[index] = input_mesh1.vertices[index];
        } else {
            let offset = index - u32(arrayLength(&input_mesh1.vertices));
            if (offset < arrayLength(&input_mesh2.vertices)) {
                output_mesh.vertices[index] = input_mesh2.vertices[offset];
            }
        }
    }
}

// Intersection operation
@compute
@workgroup_size(64, 1, 1)
fn intersection(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;

    // Find overlapping triangles between meshes
    // This is a simplified placeholder - real implementation would use
    // BVH trees or spatial partitioning for efficient intersection

    let num_triangles1 = arrayLength(&input_mesh1.triangles);
    let num_triangles2 = arrayLength(&input_mesh2.triangles);

    if (index < num_triangles1) {
        for (var i: u32 = 0u; i < num_triangles2; i++) {
            if (triangles_intersect(&input_mesh1.triangles[index], &input_mesh2.triangles[i])) {
                // Add intersecting triangle to output
                output_mesh.triangles[index] = input_mesh1.triangles[index];
            }
        }
    }
}

// Difference operation
@compute
@workgroup_size(64, 1, 1)
fn difference(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;

    // Subtract mesh2 from mesh1
    // This is a simplified placeholder

    let num_triangles1 = arrayLength(&input_mesh1.triangles);
    let num_triangles2 = arrayLength(&input_mesh2.triangles);

    if (index < num_triangles1) {
        var keep_triangle = true;
        for (var i: u32 = 0u; i < num_triangles2; i++) {
            if (triangles_intersect(&input_mesh1.triangles[index], &input_mesh2.triangles[i])) {
                keep_triangle = false;
                break;
            }
        }

        if (keep_triangle) {
            output_mesh.triangles[index] = input_mesh1.triangles[index];
        }
    }
}

// Triangle intersection test
fn triangles_intersect(t1: Triangle, t2: Triangle) -> bool {
    // Simplified AABB test - real implementation would use
    // Möller–Trumbore intersection algorithm or similar

    let min1 = min(min(t1.v0.pos, t1.v1.pos), t1.v2.pos);
    let max1 = max(max(t1.v0.pos, t1.v1.pos), t1.v2.pos);

    let min2 = min(min(t2.v0.pos, t2.v1.pos), t2.v2.pos);
    let max2 = max(max(t2.v0.pos, t2.v1.pos), t2.v2.pos);

    return all(min1 <= max2) && all(min2 <= max1);
}
