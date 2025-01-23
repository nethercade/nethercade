// Consts
const MAX_TEXTURES: u32 = 4;

struct PushConstants {
    blend_modes: u32,
    is_matcap: u32,
}

var<push_constant> push_constants: PushConstants;

// PER FRAME BINDINGS
@group(0) @binding(0)
var<storage> model_matrices: array<mat4x4<f32>>;

@group(0) @binding(1)
var<storage> view_matrices: array<mat4x4<f32>>;

@group(0) @binding(2)
var<storage> projection_matrices: array<mat4x4<f32>>;

@group(0) @binding(3)
var<storage> camera_positions: array<vec3<f32>>;

@group(0) @binding(4)
var texture_sampler: sampler;

@group(0) @binding(5)
var matcap_sampler: sampler;
// END PER FRAME BINDINGS

// TEXTURE BINDINGS
@group(1) @binding(0)
var texture1: texture_2d<f32>;

@group(1) @binding(1)
var texture2: texture_2d<f32>;

@group(1) @binding(2)
var texture3: texture_2d<f32>;

@group(1) @binding(3)
var texture4: texture_2d<f32>;
// END TEXTURE BINDINGS

struct InstanceInput {
    @location(4) model_index: u32,
    @location(5) view_pos_index: u32,
    @location(6) projection_index: u32,
}

// Vertex Color
struct VertexColorIn {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexColorOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_color(
    model: VertexColorIn,
    instance: InstanceInput,
) -> VertexColorOut {
    var out: VertexColorOut;
    let model_matrix = model_matrices[instance.model_index];
    let view_matrix = view_matrices[instance.view_pos_index];
    let projection_matrix = projection_matrices[instance.projection_index];

    out.color = model.color;
    out.clip_position = projection_matrix * view_matrix * model_matrix * vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_color(in: VertexColorOut) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}

// Vertex UVs
struct VertexUvIn {
    @location(0) position: vec3<f32>,
    @location(2) uvs: vec2<f32>,
};

struct VertexUvOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) uvs: vec2<f32>,
};

@vertex
fn vs_uv(
    model: VertexUvIn,
    instance: InstanceInput,
) -> VertexUvOut {
    var out: VertexUvOut;
    let model_matrix = model_matrices[instance.model_index];
    let view_matrix = view_matrices[instance.view_pos_index];
    let projection_matrix = projection_matrices[instance.projection_index];

    out.uvs = model.uvs;
    out.clip_position = projection_matrix * view_matrix * model_matrix * vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_uv(in: VertexUvOut) -> @location(0) vec4<f32> {
    let color = get_blended_textures(in.uvs);
    return vec4(color, 1.0);
}

// Vertex Color + UVs
struct VertexColorUvIn {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) uvs: vec2<f32>,
};

struct VertexColorUvOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) color: vec3<f32>,
    @location(2) uvs: vec2<f32>,
};

@vertex
fn vs_color_uv(
    model: VertexColorUvIn,
    instance: InstanceInput,
) -> VertexColorUvOut {
    var out: VertexColorUvOut;
    let model_matrix = model_matrices[instance.model_index];
    let view_matrix = view_matrices[instance.view_pos_index];
    let projection_matrix = projection_matrices[instance.projection_index];

    out.color = model.color;
    out.uvs = model.uvs;
    out.clip_position = projection_matrix * view_matrix * model_matrix * vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_color_uv(in: VertexColorUvOut) -> @location(0) vec4<f32> {
    let color = get_color_blended_textures(in.color, in.uvs);
    return vec4<f32>(color, 1.0);
}

@vertex
fn vs_quad_2d(
    model: VertexUvIn,
    instance: InstanceInput,
) -> VertexUvOut {
    var out: VertexUvOut;
    // TODO: This!
    // let model_matrix = model_matrices[instance.model_index];

    // out.uvs = model.uvs;
    // out.clip_position = camera.ortho * model_matrix * vec4<f32>(model.position, 1.0);

    return out;
}

fn matcap_uv(view: vec3<f32>, normal: vec3<f32>) -> vec2<f32> {
  let inv_depth = 1.0 / (1.0 + view.z);
  let proj_factor = -view.x * view.y * inv_depth;
  let basis1 = vec3(1.0 - view.x * view.x * inv_depth, proj_factor, -view.x);
  let basis2 = vec3(proj_factor, 1.0 - view.y * view.y * inv_depth, -view.y);
  let matcap_uv = vec2(dot(basis1, normal), dot(basis2, normal));

  return matcap_uv * vec2(0.5, -0.5) + 0.5;
}

struct VertexMatcapIn {
    @location(0) position: vec3<f32>,
    @location(3) normals: vec3<f32>,
}

struct VertexMatcapOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) view_pos: vec3<f32>,
    @location(3) normals: vec3<f32>,
}

@vertex
fn vs_matcap(
    model: VertexMatcapIn,
    instance: InstanceInput,
) -> VertexMatcapOut {
    var out: VertexMatcapOut;
    let model_matrix = model_matrices[instance.model_index];
    let view_matrix = view_matrices[instance.view_pos_index];
    let projection_matrix = projection_matrices[instance.projection_index];

    let view_position = view_matrix * model_matrix * vec4<f32>(model.position, 1.0);

    out.clip_position = projection_matrix * view_position;
    out.view_pos = view_position.xyz;
    out.normals = normalize((view_matrix * model_matrix * vec4<f32>(model.normals, 0.0)).xyz);

    return out;
}

@fragment
fn fs_matcap(in: VertexMatcapOut) -> @location(0) vec4<f32> {
    let normal = normalize(in.normals);
    let view = normalize(-in.view_pos);
    let color = get_blended_matcaps(view, normal);
    return vec4<f32>(color, 1.0);
}

struct VertexMatcapColorIn {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(3) normals: vec3<f32>,
};

struct VertexMatcapColorOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) color: vec3<f32>,
    @location(3) normals: vec3<f32>,
    @location(0) view_pos: vec3<f32>,
};

@vertex
fn vs_matcap_color(
    model: VertexMatcapColorIn,
    instance: InstanceInput,
) -> VertexMatcapColorOut {
    var out: VertexMatcapColorOut;
    let model_matrix = model_matrices[instance.model_index];
    let view_matrix = view_matrices[instance.view_pos_index];
    let projection_matrix = projection_matrices[instance.projection_index];

    let view_position = view_matrix * model_matrix * vec4<f32>(model.position, 1.0);

    out.clip_position = projection_matrix * view_position;
    out.view_pos = view_position.xyz;
    out.normals = normalize((view_matrix * model_matrix * vec4<f32>(model.normals, 0.0)).xyz);
    out.color = model.color;

    return out;
}

@fragment
fn fs_matcap_color(in: VertexMatcapColorOut) -> @location(0) vec4<f32> {
    let normal = normalize(in.normals);
    let view = normalize(-in.view_pos);
    let color = get_color_blended_matcaps(in.color, view, normal);
    return vec4<f32>(in.color * color, 1.0);
}

struct VertexMatcapUvIn {
    @location(0) position: vec3<f32>,
    @location(2) uvs: vec2<f32>,
    @location(3) normals: vec3<f32>,
};

struct VertexMatcapUvOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(2) uvs: vec2<f32>,
    @location(3) normals: vec3<f32>,
    @location(0) view_pos: vec3<f32>,
};

@vertex
fn vs_matcap_uv(
    model: VertexMatcapUvIn,
    instance: InstanceInput,
) -> VertexMatcapUvOut {
    var out: VertexMatcapUvOut;
    let model_matrix = model_matrices[instance.model_index];
    let view_matrix = view_matrices[instance.view_pos_index];
    let projection_matrix = projection_matrices[instance.projection_index];

    let view_position = view_matrix * model_matrix * vec4<f32>(model.position, 1.0);

    out.clip_position = projection_matrix * view_position;
    out.view_pos = view_position.xyz;
    out.normals = normalize((view_matrix * model_matrix * vec4<f32>(model.normals, 0.0)).xyz);
    out.uvs = model.uvs;

    return out;
}

@fragment
fn fs_matcap_uv(in: VertexMatcapUvOut) -> @location(0) vec4<f32> {
    let normal = normalize(in.normals);
    let view = normalize(-in.view_pos);
    let color = get_blended_both(in.uvs, view, normal);

    return vec4<f32>(color, 1.0);
}

struct VertexMatcapColorUvIn {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) uvs: vec2<f32>,
    @location(3) normals: vec3<f32>,
};

struct VertexMatcapColorUvOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) color: vec3<f32>,
    @location(2) uvs: vec2<f32>,
    @location(3) normals: vec3<f32>,
    @location(0) view_pos: vec3<f32>,
};

@vertex
fn vs_matcap_color_uv(
    model: VertexMatcapColorUvIn,
    instance: InstanceInput,
) -> VertexMatcapColorUvOut {
    var out: VertexMatcapColorUvOut;
    let model_matrix = model_matrices[instance.model_index];
    let view_matrix = view_matrices[instance.view_pos_index];
    let projection_matrix = projection_matrices[instance.projection_index];

    let view_position = view_matrix * model_matrix * vec4<f32>(model.position, 1.0);

    out.clip_position = projection_matrix * view_position;
    out.view_pos = view_position.xyz;
    out.normals = normalize((view_matrix * model_matrix * vec4<f32>(model.normals, 0.0)).xyz);
    out.uvs = model.uvs;
    out.color = model.color;

    return out;
}

@fragment
fn fs_matcap_color_uv(in: VertexMatcapColorUvOut) -> @location(0) vec4<f32> {
    let normal = normalize(in.normals);
    let view = normalize(-in.view_pos);
    let color = get_blended_both(in.uvs, view, normal);

    return vec4<f32>(in.color * color, 1.0);
}

fn get_blended_textures(uvs: vec2<f32>) -> vec3<f32> {
    var out = textureSample(texture1, texture_sampler, uvs).rgb;

    for (var i = 1u; i < MAX_TEXTURES; i++) {
        let blend_mode = (push_constants.blend_modes >> (i * 8u)) & 0xFF;
        if blend_mode == 0 {
            continue;
        }
        if (push_constants.is_matcap & (1u << u32(i))) != 0u {
            continue;
        }

        let texel = sample_texture_array(i, texture_sampler, uvs);
        out = blend_layers(out, texel, blend_mode);
    }
    return out;
}

fn get_color_blended_textures(color: vec3<f32>, uvs: vec2<f32>) -> vec3<f32> {
    var out = color;

    for (var i = 0u; i < MAX_TEXTURES; i++) {
        let blend_mode = (push_constants.blend_modes >> (i * 8u)) & 0xFF;
        if blend_mode == 0 {
            continue;
        }
        if (push_constants.is_matcap & (1u << u32(i))) != 0u {
            continue;
        }

        let texel = sample_texture_array(i, texture_sampler, uvs);
        out = blend_layers(out, texel, blend_mode);
    }
    return out;
}

fn get_blended_matcaps(view: vec3<f32>, normal: vec3<f32>) -> vec3<f32> {
    let matcap_uvs = matcap_uv(view, normal);
    var out = textureSample(texture1, matcap_sampler, matcap_uvs).rgb;

    for (var i = 1u; i < MAX_TEXTURES; i++) {
        let blend_mode = (push_constants.blend_modes >> (i * 8u)) & 0xFF;
        if blend_mode == 0 {
            continue;
        }
        if (push_constants.is_matcap & (1u << u32(i))) == 0u {
            continue;
        }

        let texel = sample_texture_array(i, matcap_sampler, matcap_uvs);
        out = blend_layers(out, texel, blend_mode);
    }
    return out;
}

fn get_color_blended_matcaps(color: vec3<f32>, view: vec3<f32>, normal: vec3<f32>) -> vec3<f32> {
    let matcap_uvs = matcap_uv(view, normal);

    var out = color;

    for (var i = 0u; i < MAX_TEXTURES; i++) {
        let blend_mode = (push_constants.blend_modes >> (i * 8u)) & 0xFF;
        if blend_mode == 0 {
            continue;
        }
        if (push_constants.is_matcap & (1u << u32(i))) == 0u {
            continue;
        }

        let texel = sample_texture_array(i, matcap_sampler, matcap_uvs);
        out = blend_layers(out, texel, blend_mode);
    }
    return out;
}

fn get_blended_both(uvs: vec2<f32>, view: vec3<f32>, normal: vec3<f32>) -> vec3<f32> {
    let matcap_uv = matcap_uv(view, normal);

    var out: vec3<f32>;

    if (push_constants.is_matcap & (1u << u32(0))) != 0u {
        out = textureSample(texture1, matcap_sampler, matcap_uv).rgb;
    } else {
        out = textureSample(texture1, texture_sampler, uvs).rgb;
    }
    
    for (var i = 1u; i < MAX_TEXTURES; i++) {
        let blend_mode = (push_constants.blend_modes >> (i * 8u)) & 0xFF;

        if blend_mode == 0 {
            continue;
        }

        var texel: vec3<f32>;
        if (push_constants.is_matcap & (1u << u32(i))) != 0u {
            texel = sample_texture_array(i, matcap_sampler, matcap_uv);
        } else {
            texel = sample_texture_array(i, texture_sampler, uvs);
        }

        out = blend_layers(out, texel, blend_mode);
    }

    return out;
}

fn get_color_blended_both(color: vec3<f32>, uvs: vec2<f32>, view: vec3<f32>, normal: vec3<f32>) -> vec3<f32> {
    let matcap_uv = matcap_uv(view, normal);

    var out = color;

    for (var i = 0u; i < MAX_TEXTURES; i++) {
        let blend_mode = (push_constants.blend_modes >> (i * 8u)) & 0xFF;

        if blend_mode == 0 {
            break;
        }

        var texel: vec3<f32>;
        if (push_constants.is_matcap & (1u << u32(i))) != 0u {
            texel = sample_texture_array(i, matcap_sampler, matcap_uv);
        } else {
            texel = sample_texture_array(i, texture_sampler, uvs);
        }

        out = blend_layers(out, texel, blend_mode);
    }

    return out;
}

fn sample_texture_array(index: u32, sam: sampler, uvs: vec2<f32>) -> vec3<f32> {
    switch index {
        case 1u: {
            return textureSample(texture2, sam, uvs).rgb;
        }
        case 2u: {
            return textureSample(texture3, sam, uvs).rgb;
        }
        case 3u: {
            return textureSample(texture4, sam, uvs).rgb;
        }
        default: {
            return textureSample(texture1, sam, uvs).rgb;
        }
    }
}

fn blend_layers(bottom: vec3<f32>, top: vec3<f32>, blend: u32) -> vec3<f32> {
    switch blend {
        case 1u: {
            return top;
        }
        case 2u: {
            return blend_add(bottom, top);
        }
        case 3u: {
            return blend_screen(bottom, top);
        }
        case 4u: {
            return blend_color_dodge(bottom, top);
        }
        case 5u: {
            return blend_subtract(bottom, top);
        }
        case 6u: {
            return blend_multiply(bottom, top);
        }
        case 7u: {
            return blend_color_burn(bottom, top);
        }
        case 8u: {
            return blend_overlay(bottom, top);
        }
        default: {
            return bottom;
        }
    }
}

// // Normal (Replace): Simply uses the blend color
// fn blend_normal(base: vec3<f32>, blend: vec3<f32>) -> vec3<f32> {
//     return blend;
// }

// Add: base + blend
fn blend_add(base: vec3<f32>, blend: vec3<f32>) -> vec3<f32> {
    return clamp(base + blend, vec3<f32>(0.0), vec3<f32>(1.0));
}

// Screen: 1 - (1 - base) * (1 - blend)
fn blend_screen(base: vec3<f32>, blend: vec3<f32>) -> vec3<f32> {
    return 1.0 - (1.0 - base) * (1.0 - blend);
}

// Color Dodge: base / (1 - blend)
fn blend_color_dodge(base: vec3<f32>, blend: vec3<f32>) -> vec3<f32> {
    return clamp(base / (1.0 - blend), vec3<f32>(0.0), vec3<f32>(1.0));
}

// Subtract: base - blend
fn blend_subtract(base: vec3<f32>, blend: vec3<f32>) -> vec3<f32> {
    return clamp(base - blend, vec3<f32>(0.0), vec3<f32>(1.0));
}

// Multiply: base * blend
fn blend_multiply(base: vec3<f32>, blend: vec3<f32>) -> vec3<f32> {
    return base * blend;
}

// Color Burn: 1 - ((1 - base) / blend)
fn blend_color_burn(base: vec3<f32>, blend: vec3<f32>) -> vec3<f32> {
    return clamp(1.0 - ((1.0 - base) / blend), vec3<f32>(0.0), vec3<f32>(1.0));
}

// Overlay: Combines Multiply and Screen based on base
fn blend_overlay(base: vec3<f32>, blend: vec3<f32>) -> vec3<f32> {
    let multiplier = 2.0 * base * blend;
    let screen = 1.0 - 2.0 * (1.0 - base) * (1.0 - blend);
    let mask = step(vec3<f32>(0.5), base); // Creates a vec3 mask based on base
    return mix(multiplier, screen, mask);
}
