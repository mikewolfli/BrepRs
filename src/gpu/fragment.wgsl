// Fragment Shader
// This shader performs final shading for rendered geometry

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coord: vec2<f32>,
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
var<uniform> camera_position: vec3<f32>;

@group(0)
@binding(2)
var<uniform> light_direction: vec3<f32>;

@group(0)
@binding(3)
var<uniform> light_color: vec3<f32>;

@group(0)
@binding(4)
var<uniform> ambient_color: vec3<f32>;

@fragment
fn main(in: VertexOutput, @location(0) out_color: vec4<f32>) {
    let material = materials[0];

    // Normalize inputs
    let normal = normalize(in.normal);
    let view_dir = normalize(camera_position - in.world_position);
    let light_dir = normalize(light_direction);
    let half_dir = normalize(view_dir + light_dir);

    // Calculate lighting terms
    let n_dot_l = max(dot(normal, light_dir), 0.0);
    let n_dot_v = max(dot(normal, view_dir), 0.0);
    let n_dot_h = max(dot(normal, half_dir), 0.0);

    // Diffuse (Lambert)
    let diffuse = material.albedo * n_dot_l;

    // Specular (Blinn-Phong)
    let specular_strength = pow(n_dot_h, (1.0 - material.roughness) * 256.0);
    let specular = light_color * specular_strength * material.metallic;

    // Ambient
    let ambient = material.albedo * ambient_color;

    // Combine lighting
    let color = ambient + diffuse + specular + material.emissive;

    // Tone mapping (Reinhard)
    color = color / (color + vec3<f32>(1.0));
    color = pow(color, vec3<f32>(1.0 / 2.2));

    out_color = vec4<f32>(color, 1.0);
}
