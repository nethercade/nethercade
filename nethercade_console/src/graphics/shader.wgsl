const POSITIONS = array<vec2<f32>, 3>(
    vec2<f32>(0.0, 0.75),  // Top
    vec2<f32>(-0.75, -0.75),  // Bottom-left
    vec2<f32>(0.75, -0.75),  // Bottom-right
);

const COLORS = array<vec3<f32>, 3>(
    vec3<f32>(1.0, 0.0, 0.0),
    vec3<f32>(0.0, 1.0, 0.0),
    vec3<f32>(0.0, 0.0, 1.0),
);

struct VsOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(
    @builtin(vertex_index) index: u32
) -> VsOut {
    var out: VsOut;

    out.clip_position = vec4<f32>(POSITIONS[index], 0.0, 1.0);
    out.color = COLORS[index];
    return out;
}

@fragment
fn fs_main(
    in: VsOut,
) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}