const POSITIONS = array<vec2<f32>, 4>(
    // 0 - 1
    // |   |
    // 2 - 3
    vec2<f32>(-1.0, 1.0),  // Top-left
    vec2<f32>(1.0, 1.0),  // Top-right
    vec2<f32>(-1.0, -1.0),  // Bottom-left
    vec2<f32>(1.0, -1.0),  // Bottom-right

);

// Define the texture coordinates (UVs) for each vertex
const UVS = array<vec2<f32>, 4>(
    vec2<f32>(0.0, 0.0),  // Top-left (0, 0)
    vec2<f32>(1.0, 0.0),  // Top-right (1, 0)
    vec2<f32>(0.0, 1.0),  // Bottom-left (0, 1)
    vec2<f32>(1.0, 1.0),  // Bottom-right (1, 1)
);

// Texture Bindings
@group(0) @binding(0)
var t_texture: texture_2d<f32>;

@group(0) @binding(1)
var s_sampler: sampler;

struct VsOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uvs: vec2<f32>,
}

@vertex
fn vs_main(
    @builtin(vertex_index) index: u32
) -> VsOut {
    var out: VsOut;

    out.clip_position = vec4<f32>(POSITIONS[index], 0.0, 1.0);
    out.uvs = UVS[index];
    return out;
}

@fragment
fn fs_main(
    in: VsOut,
) -> @location(0) vec4<f32> {
    return vec4<f32>(textureSample(t_texture, s_sampler, in.uvs).rgb, 1.0);
}