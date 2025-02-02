#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_types

struct VertexInput {
    @location(0) position: vec3<f32>,
#ifndef OFFSET_ZERO
    @location(1) normal: vec3<f32>,
#endif
#ifdef SKINNED
    @location(2) joint_indexes: vec4<u32>,
    @location(3) joint_weights: vec4<f32>,
#endif
};

struct OutlineViewUniform {
    @align(16)
    scale: vec2<f32>,
};

struct OutlineVertexUniform {
    @align(16)
    origin: vec3<f32>,
    offset: f32,
};

@group(1) @binding(0)
var<uniform> mesh: Mesh;

#ifdef SKINNED
@group(1) @binding(1)
var<uniform> joint_matrices: SkinnedMesh;
#import bevy_pbr::skinning
#endif

@group(2) @binding(0)
var<uniform> view_uniform: OutlineViewUniform;

@group(3) @binding(0)
var<uniform> vstage: OutlineVertexUniform;

fn mat4to3(m: mat4x4<f32>) -> mat3x3<f32> {
    return mat3x3<f32>(
        m[0].xyz, m[1].xyz, m[2].xyz
    );
}

fn model_origin_z(plane: vec3<f32>, view_proj: mat4x4<f32>) -> f32 {
    var proj_zw = mat4x2<f32>(
        view_proj[0].zw, view_proj[1].zw,
        view_proj[2].zw, view_proj[3].zw);
    var zw = proj_zw * vec4<f32>(plane, 1.0);
    return zw.x / zw.y;
}

@vertex
fn vertex(vertex: VertexInput) -> @builtin(position) vec4<f32> {
#ifdef SKINNED
    let model = skin_model(vertex.joint_indexes, vertex.joint_weights);
#else
    let model = mesh.model;
#endif
    let clip_pos = view.view_proj * (model * vec4<f32>(vertex.position, 1.0));
#ifdef FLAT_DEPTH
    let ndc_pos = clip_pos.xy / clip_pos.w;
    let out_zw = vec2<f32>(model_origin_z(vstage.origin, view.view_proj), 1.0);
#else
    let ndc_pos = clip_pos.xy;
    let out_zw = clip_pos.zw;
#endif
#ifdef OFFSET_ZERO
    let out_xy = ndc_pos;
#else
    let clip_norm = mat4to3(view.view_proj) * (mat4to3(model) * vertex.normal);
    let ndc_delta = vstage.offset * normalize(clip_norm.xy) * view_uniform.scale * out_zw.y;
    let out_xy = ndc_pos + ndc_delta;
#endif
    return vec4<f32>(out_xy, out_zw);
}
