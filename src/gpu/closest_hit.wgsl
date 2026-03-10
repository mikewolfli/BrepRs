// Closest Hit Shader
// This shader handles rays that hit geometry

struct HitAttributes {
    barycentrics: vec2<f32>,
};

struct Material {
    albedo: vec3<f32>,
    metallic: f32,
    roughness: f32,
    emissive: vec3<f32>,
};

@group(0)
@binding(0)
var<storage, read> materials: array<Material>;

@group(0)
@binding(1)
var<storage, read> vertex_buffer: array<vec3<f32>>;

@group(0)
@binding(2)
var<storage, read> index_buffer: array<u32>;

@closest_hit
fn main(
    @builtin(ray_payload) payload: ptr<function, vec4<f32>>,
    @builtin(ray_attrib) attributes: HitAttributes,
) {
    let bary = attributes.barycentrics;
    let triangle_index = u32(attributes.instance_id);

    // Get triangle vertices
    let i0 = index_buffer[triangle_index * 3];
    let i1 = index_buffer[triangle_index * 3 + 1];
    let i2 = index_buffer[triangle_index * 3 + 2];

    let v0 = vertex_buffer[i0];
    let v1 = vertex_buffer[i1];
    let v2 = vertex_buffer[i2];

    // Interpolate position
    let position = v0 * (1.0 - bary.x - bary.y) + v1 * bary.x + v2 * bary.y;

    // Calculate normal
    let edge1 = v1 - v0;
    let edge2 = v2 - v0;
    let normal = normalize(cross(edge1, edge2));

    // Get material
    let material = materials[triangle_index % arrayLength(&materials)];

    // Simple PBR-like shading
    let view_dir = normalize(-position);
    let light_dir = normalize(vec3<f32>(1.0, 1.0, 1.0));
    let half_dir = normalize(view_dir + light_dir);

    let n_dot_l = max(dot(normal, light_dir), 0.0);
    let n_dot_v = max(dot(normal, view_dir), 0.0);
    let n_dot_h = max(dot(normal, half_dir), 0.0);

    // Diffuse
    let diffuse = material.albedo * n_dot_l;

    // Specular (simplified)
    let specular = pow(n_dot_h, (1.0 - material.roughness) * 256.0) * material.metallic;

    // Combine
    let color = diffuse + specular + material.emissive;

    *payload = vec4<f32>(color, 1.0);
}
