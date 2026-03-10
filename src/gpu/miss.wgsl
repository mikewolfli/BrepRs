// Miss Shader
// This shader handles rays that miss all geometry

struct MissPayload {
    color: vec4<f32>,
};

@group(0)
@binding(0)
var<uniform> sky_color: vec4<f32>;

@miss
fn main(@builtin(ray_payload) payload: ptr<function, MissPayload>) {
    (*payload).color = sky_color;
}
