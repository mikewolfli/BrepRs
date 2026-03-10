// Global Illumination Compute Shader
// This shader computes global illumination using various techniques

struct Light {
    position: vec3<f32>,
    color: vec3<f32>,
    intensity: f32,
    radius: f32,
};

struct Voxel {
    position: vec3<f32>,
    color: vec3<f32>,
    normal: vec3<f32>,
    occlusion: f32,
};

@group(0)
@binding(0)
var<storage, read> lights: array<Light>;

@group(0)
@binding(1)
var<storage, read> voxels: array<Voxel>;

@group(0)
@binding(2)
var<storage, read_write> gi_buffer: array<vec4<f32>>;

@group(0)
@binding(3)
var<uniform> gi_config: vec4<u32>;

@compute
@workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let width = gi_config.x;
    let height = gi_config.y;
    let num_lights = gi_config.z;
    let technique = gi_config.w;

    if (global_id.x >= width || global_id.y >= height) {
        return;
    }

    let pixel_index = global_id.y * width + global_id.x;

    var gi_color: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);

    if (technique == 0u) {
        // Voxel-based GI
        gi_color = compute_voxel_gi(global_id.xy, width, height);
    } else if (technique == 1u) {
        // Light propagation volumes
        gi_color = compute_lpv_gi(global_id.xy, width, height);
    } else if (technique == 2u) {
        // Ray-traced GI
        gi_color = compute_rt_gi(global_id.xy, width, height, num_lights);
    } else {
        // Hybrid approach
        gi_color = compute_hybrid_gi(global_id.xy, width, height, num_lights);
    }

    gi_buffer[pixel_index] = vec4<f32>(gi_color, 1.0);
}

// Voxel-based global illumination
fn compute_voxel_gi(pixel: vec2<u32>, width: u32, height: u32) -> vec3<f32> {
    let uv = vec2<f32>(pixel) / vec2<f32>(width, height);

    var gi: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    let num_samples = 16u;

    for (var i: u32 = 0u; i < num_samples; i++) {
        let angle = f32(i) * 2.0 * 3.14159 / f32(num_samples);
        let direction = vec3<f32>(cos(angle), sin(angle), 0.0);

        // Sample voxels along direction
        let voxel_index = u32(uv.x * 64.0) + u32(uv.y * 64.0) * 64u;

        if (voxel_index < arrayLength(&voxels)) {
            let voxel = voxels[voxel_index];
            gi += voxel.color * voxel.occlusion * (1.0 - voxel.occlusion);
        }
    }

    return gi / f32(num_samples);
}

// Light propagation volumes
fn compute_lpv_gi(pixel: vec2<u32>, width: u32, height: u32) -> vec3<f32> {
    let uv = vec2<f32>(pixel) / vec2<f32>(width, height);

    var gi: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);

    // Sample from LPV grid
    let grid_x = u32(uv.x * 32.0);
    let grid_y = u32(uv.y * 32.0);
    let grid_z = u32(uv.x * 32.0 + uv.y * 32.0) % 32u;

    let lpv_index = grid_x + grid_y * 32u + grid_z * 1024u;

    if (lpv_index < arrayLength(&voxels)) {
        let voxel = voxels[lpv_index];
        gi = voxel.color * voxel.occlusion;
    }

    return gi;
}

// Ray-traced global illumination
fn compute_rt_gi(pixel: vec2<u32>, width: u32, height: u32, num_lights: u32) -> vec3<f32> {
    let uv = vec2<f32>(pixel) / vec2<f32>(width, height);

    var gi: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);

    // Trace rays to lights
    for (var i: u32 = 0u; i < num_lights && i < arrayLength(&lights); i++) {
        let light = lights[i];

        // Simple distance-based contribution
        let light_dir = normalize(light.position - vec3<f32>(uv * 2.0 - 1.0, 0.0));
        let distance = length(light.position - vec3<f32>(uv * 2.0 - 1.0, 0.0));

        let attenuation = smoothstep(light.radius, 0.0, distance);
        gi += light.color * light.intensity * attenuation;
    }

    return gi / f32(min(num_lights, arrayLength(&lights)));
}

// Hybrid global illumination
fn compute_hybrid_gi(pixel: vec2<u32>, width: u32, height: u32, num_lights: u32) -> vec3<f32> {
    let voxel_gi = compute_voxel_gi(pixel, width, height);
    let rt_gi = compute_rt_gi(pixel, width, height, num_lights);

    // Blend based on distance
    let blend = 0.5;
    return mix(voxel_gi, rt_gi, blend);
}
