// Ray Generation Shader
// This shader generates rays for ray tracing

struct RayDesc {
    origin: vec3<f32>,
    direction: vec3<f32>,
    t_min: f32,
    t_max: f32,
};

struct Camera {
    position: vec3<f32>,
    forward: vec3<f32>,
    right: vec3<f32>,
    up: vec3<f32>,
    fov: f32,
    aspect_ratio: f32,
};

@group(0)
@binding(0)
var<uniform> camera: Camera;

@group(0)
@binding(1)
var<storage, read_write> output_image: texture_storage_2d<rgba32float, read_write>;

@group(0)
@binding(2)
var<storage, read> acceleration_structure: acceleration_structure;

@compute
@workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let width = textureDimensions(&output_image).x;
    let height = textureDimensions(&output_image).y;

    if (global_id.x >= width || global_id.y >= height) {
        return;
    }

    // Calculate normalized device coordinates
    let uv = vec2<f32>(
        f32(global_id.x) / f32(width),
        f32(global_id.y) / f32(height),
    );

    // Generate ray direction
    let ndc = uv * 2.0 - 1.0;
    let ray_dir = normalize(
        camera.right * ndc.x * camera.aspect_ratio * tan(camera.fov / 2.0) +
        camera.up * ndc.y * tan(camera.fov / 2.0) +
        camera.forward
    );

    let ray = RayDesc {
        origin: camera.position,
        direction: ray_dir,
        t_min: 0.001,
        t_max: 10000.0,
    };

    // Trace ray
    var payload: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 1.0);

    let ray_query = rayQueryCreate();
    rayQueryInitialize(
        ray_query,
        acceleration_structure,
        0xFFu,
        0u,
        ray.origin,
        ray.t_min,
        ray.direction,
        ray.t_max
    );

    while (rayQueryProceed(ray_query)) {
        if (rayQueryGetIntersectionType(ray_query) == RAY_QUERY_INTERSECTION_TRIANGLE) {
            let hit_t = rayQueryGetIntersectionT(ray_query);
            let barycentrics = rayQueryGetIntersectionBarycentrics(ray_query);
            let instance_id = rayQueryGetIntersectionInstanceCustomIndex(ray_query);

            // Simple shading based on instance ID
            let base_color = vec3<f32>(
                f32(instance_id % 3) / 3.0,
                f32((instance_id / 3) % 3) / 3.0,
                f32((instance_id / 9) % 3) / 3.0,
            );

            payload = vec4<f32>(base_color, 1.0);
            rayQueryTerminate(ray_query);
        }
    }

    // Store result
    textureStore(&output_image, vec2<i32>(i32(global_id.x), i32(global_id.y)), payload);
}
