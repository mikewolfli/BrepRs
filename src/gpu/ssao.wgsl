// Screen Space Ambient Occlusion Shader
// This shader computes ambient occlusion in screen space

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@group(0)
@binding(0)
var<uniform> ssao_params: vec4<f32>;

@group(0)
@binding(1)
var<uniform> projection_matrix: mat4x4<f32>;

@group(0)
@binding(2)
var<storage, read> depth_texture: texture_depth_2d;

@group(0)
@binding(3)
var<storage, read> normal_texture: texture_2d<vec3<f32>>;

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
fn fs_main(in: VertexOutput, @location(0) out_color: f32) {
    let radius = ssao_params.x;
    let bias = ssao_params.y;
    let intensity = ssao_params.z;

    let depth = textureLoad(&depth_texture, vec2<i32>(i32(in.uv * vec2<f32>(textureDimensions(&depth_texture))));
    let normal = normalize(textureLoad(&normal_texture, vec2<i32>(i32(in.uv * vec2<f32>(textureDimensions(&normal_texture))))).rgb);

    let width = f32(textureDimensions(&depth_texture).x);
    let height = f32(textureDimensions(&depth_texture).y);

    var occlusion: f32 = 0.0;
    let num_samples = 16u;

    for (var i: u32 = 0u; i < num_samples; i++) {
        let angle = f32(i) * 2.0 * 3.14159 / f32(num_samples);
        let offset = vec2<f32>(cos(angle), sin(angle)) * radius;

        let sample_uv = in.uv + offset / vec2<f32>(width, height);
        let sample_depth = textureLoad(&depth_texture, vec2<i32>(i32(sample_uv * vec2<f32>(textureDimensions(&depth_texture))));

        let range_check = smoothstep(0.0, 1.0, radius / abs(depth - sample_depth));
        occlusion += range_check * step(depth, sample_depth - bias);
    }

    occlusion = 1.0 - (occlusion / f32(num_samples)) * intensity;
    out_color = occlusion;
}
