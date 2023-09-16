// This shader computes the GB palette effect

#import bevy_pbr::utils
#import bevy_core_pipeline::fullscreen_vertex_shader FullscreenVertexOutput

@group(0) @binding(0)
var screen_texture: texture_2d<f32>;

@group(0) @binding(1)
var texture_sampler: sampler;

struct GBShaderSettings {
    color_0: vec4<f32>,
    color_1: vec4<f32>,
    color_2: vec4<f32>,
    color_3: vec4<f32>,
}
@group(0) @binding(2)
var<uniform> settings: GBShaderSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    var color = textureSample(screen_texture, texture_sampler, in.uv);
    var palette = settings.color_0;

    if (color[0] == 1.0) {
        palette = settings.color_3;
    } else if (color[0] > 0.4) {
        palette = settings.color_2;
    } else if (color[0] > 0.1) {
        palette = settings.color_1;
    }

    color[0] = palette[0];
    color[1] = palette[1];
    color[2] = palette[2];
    color[3] = 1.0;

    return color;
}