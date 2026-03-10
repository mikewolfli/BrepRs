// Task Shader
// This shader distributes work among mesh shader workgroups

struct TaskPayload {
    meshlet_count: u32,
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
var<storage, read> meshlets: array<Meshlet>;

@group(0)
@binding(1)
var<storage, read_write> task_payload: TaskPayload;

@task
fn main(@builtin(num_workgroups) num_workgroups: vec3<u32>) {
    task_payload.meshlet_count = u32(num_workgroups.x * num_workgroups.y * num_workgroups.z);
}

@mesh
fn main(@builtin(task_payload) payload: TaskPayload) {
    let task_id = workgroup_id.x * workgroup_size.x +
                 workgroup_id.y * workgroup_size.x * num_workgroups.x +
                 workgroup_id.z * workgroup_size.x * num_workgroups.y * num_workgroups.z;

    if (task_id >= payload.meshlet_count) {
        return;
    }

    let meshlet = meshlets[task_id];

    // Frustum culling
    let center = meshlet.bounding_sphere.xyz;
    let radius = meshlet.bounding_sphere.w;

    var visible: bool = true;

    // Simple sphere-frustum test (simplified)
    if (center.z + radius < 0.0 || center.z - radius > 100.0) {
        visible = false;
    }

    if (visible) {
        // Set meshlet output
        SetMeshOutputs(meshlet.vertex_count, meshlet.index_count / 3);
    }
}
