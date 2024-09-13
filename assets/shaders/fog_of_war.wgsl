#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> blur: f32;
@group(2) @binding(1) var texture: texture_2d<f32>;
@group(2) @binding(2) var texture_sampler: sampler;
@group(2) @binding(3) var mask_texture: texture_2d<f32>;
@group(2) @binding(4) var mask_texture_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let tex_color = textureSample(texture, texture_sampler, mesh.uv);
    return tex_color;
}

// @fragment
// fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
//     let tex_color = textureSample(mask_texture, mask_texture_sampler, mesh.uv);
//     return tex_color;
// }

// @fragment
// fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
//     let tex_color = textureSample(texture, texture_sampler, mesh.uv);
//     var blur_size: f32 = 1.0 / f32(textureDimensions(mask_texture).x);
//     var mask_alpha: f32 = 0.0;
//     // blur
//     var kernel_extents: i32 = 4;
//     var blur_factor
//     : i32 = 1;
//     for (var x: i32 = -kernel_extents; x <= kernel_extents; x = x + 1) {
//         for (var y: i32 = -kernel_extents; y <= kernel_extents; y = y + 1) {
//             mask_alpha = mask_alpha + textureSample(mask_texture, mask_texture_sampler, mesh.uv + vec2<f32>(f32(x * blur_factor
//             ), f32(y * blur_factor
//             )) * blur_size).r;
//         }
//     }
//     var kernel_size: f32 = f32(kernel_extents) * 2.0 + 1.0;
//     mask_alpha = mask_alpha / (kernel_size * kernel_size);

//     return vec4<f32>(tex_color.rgb, mask_alpha);
// }
