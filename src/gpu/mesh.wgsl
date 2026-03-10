// Mesh Shader
// This shader generates meshlets for rendering

struct Vertex {
    position: vec3<f32>,
    normal: vec3<f32>,
    tex_coord: vec2<f32>,
};

struct Meshlet {
    vertex_offset: u32,
    vertex_count: u32,
    index_offset: u32,
    index_count: u32,
    bounding_sphere: vec4<f32>,
};

@group(0)
@binding(0)
var<storage, read> vertices: array<Vertex>;

@group(0)
@binding(1)
var<storage, read> indices: array<u32>;

@group(0)
@binding(2)
var<storage, read> meshlets: array<Meshlet>;

@group(0)
@binding(3)
var<uniform> model_matrix: mat4x4<f32>;

@group(0)
@binding(4)
var<uniform> view_proj_matrix: mat4x4<f32>;

@group(0)
@binding(5)
var<uniform> camera_position: vec3<f32>;

@mesh
fn main(
    @builtin(meshlet_index) meshlet_id: u32,
    @builtin(local_invocation_id) local_id: vec3<u32>,
    @builtin(global_invocation_id) global_id: vec3<u32>,
) {
    let meshlet = meshlets[meshlet_id];

    // Output vertices for this meshlet
    for (var i: u32 = 0u; i < meshlet.vertex_count; i++) {
        let vertex_index = meshlet.vertex_offset + i;
        let vertex = vertices[vertex_index];

        // Transform vertex to world space
        let world_pos = model_matrix * vec4<f32>(vertex.position, 1.0);
        let clip_pos = view_proj_matrix * world_pos;

        // Calculate distance to camera for LOD
        let distance = length(world_pos.xyz - camera_position);

        // Set vertex output
        SetMeshOutputVertices(i, clip_pos);
    }

    // Output indices for this meshlet
    for (var i: u32 = 0u; i < meshlet.index_count; i += 3u) {
        let i0 = indices[meshlet.index_offset + i];
        let i1 = indices[meshlet.index_offset + i + 1u];
        let i2 = indices[meshlet.index_offset + i + 2u];

        SetMeshOutputIndices(i / 3u, u32(i0), u32(i1), u32(i2));
    }
}
