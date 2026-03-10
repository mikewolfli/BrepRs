// Screen Space Reflections Shader
// This shader computes reflections in screen space

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@group(0)
@binding(0)
var<uniform> ssr_params: vec4<f32>;

@group(0)
@binding(1)
var<uniform> projection_matrix: mat4x4<f32>;

@group(0)
@binding(2)
var<uniform> inv_projection_matrix: mat4x4<f32>;

@group(0)
@binding(3)
var<uniform> inv_view_matrix: mat4x4<f32>;

@group(0)
@binding(4)
var<uniform> camera_position: vec3<f32>;

@group(0)
@binding(5)
var<storage, read> depth_texture: texture_depth_2d;

@group(0)
@binding(6)
var<storage, read> normal_texture: texture_2d<vec3<f32>>;

@group(0)
@binding(7)
var<storage, read> color_texture: texture_2d<vec4<f32>>;

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    let positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(1.0, 1.0)
    );

    var output: VertexOutput;
    output.clip_position = vec4<f32>(positions[vertex_index], 0.0, 1.0);
    output.uv = positions[vertex_index] * 0.5 + 0.5;
    return output;
}

@fragment
fn fs_main(in: VertexOutput, @location(0) out_color: vec4<f32>) {
    let max_distance = ssr_params.x;
    let thickness = ssr_params.y;
    let roughness_cutoff = ssr_params.z;

    let depth = textureLoad(&depth_texture, vec2<i32>(i32(in.uv * vec2<f32>(textureDimensions(&depth_texture)))));
    let normal = normalize(textureLoad(&normal_texture, vec2<i32>(i32(in.uv * vec2<f32>(textureDimensions(&normal_texture))))).rgb);

    let width = f32(textureDimensions(&depth_texture).x);
    let height = f32(textureDimensions(&depth_texture).y);

    // Reconstruct world position
    let ndc = vec2<f32>(in.uv) * 2.0 - 1.0;
    let clip_pos = vec4<f32>(ndc, depth, 1.0);
    let view_pos = inv_projection_matrix * clip_pos;
    view_pos = view_pos / view_pos.w;
    let world_pos = inv_view_matrix * view_pos;

    // Calculate reflection vector
    let view_dir = normalize(world_pos.xyz - camera_position);
    let reflect_dir = reflect(view_dir, normal);

    // Ray march in screen space
    var ray_pos: vec3<f32> = world_pos.xyz;
    var ray_dir: vec3<f32> = reflect_dir * 0.01;

    var color: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    var hit: bool = false;

    for (var i: u32 = 0u; i < 64u; i++) {
        ray_pos = ray_pos + ray_dir * f32(i);

        // Project back to screen space
        let view_ray_pos = view_matrix * vec4<f32>(ray_pos, 1.0);
        let clip_ray_pos = projection_matrix * view_ray_pos;
        let ndc_ray_pos = clip_ray_pos.xy / clip_ray_pos.w;

        let screen_uv = ndc_ray_pos * 0.5 + 0.5;

        // Check if still in screen bounds
        if (screen_uv.x < 0.0 || screen_uv.x > 1.0 ||
            screen_uv.y < 0.0 || screen_uv.y > 1.0) {
            break;
        }

        // Check depth
        let sample_depth = textureLoad(&depth_texture, vec2<i32>(i32(screen_uv * vec2<f32>(width, height))));

        if (abs(sample_depth - clip_ray_pos.z) < thickness) {
            color = textureLoad(&color_texture, vec2<i32>(i32(screen_uv * vec2<f32>(width, height))));
            hit = true;
            break;
        }
    }

    // Fade out reflections at grazing angles
    let fresnel = pow(1.0 - max(dot(normal, -view_dir), 0.0), 3.0);

    // Apply roughness fade
    let roughness = textureLoad(&normal_texture, vec2<i32>(i32(in.uv * vec2<f32>(textureDimensions(&normal_texture))))).a;
    let roughness_fade = 1.0 - smoothstep(roughness_cutoff, 1.0, roughness);

    out_color = color * vec4<f32>(fresnel * roughness_fade);
}
