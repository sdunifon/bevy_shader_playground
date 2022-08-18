#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

struct FragmentInput {
  @builtin(front_facing) is_front: bool,
  @builtin(position) frag_coor: vec4<f32>,
  #import bevy_pbr::mesh_vertex_output
}
struct Material {
    color: vec4<f32>
}

@group(1) @binding(0)
var<uniform> uniform_data: Material;
@group(1) @binding(1)
var texture: texture_2d<f32>;
@group(1) @binding(2)
var texture_sampler: sampler;


@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var N  = normalize(in.world_normal);
    var V = normalize(view.world_position.xyz - in.world_position.xyz);

    let NdotV = max(dot(N,V), 0.00001);

   let glow_amt = 13.3;
   let glow = pow(NdotV,glow_amt);

   let color = mix(vec4(0.0,0.0,0.0,0.0),uniform_data.color , glow);
   return vec4(color);

}
//fn fragment_color(in: FragmentInput) -> @location(0) vec4<f32> {
//    var N = normalize(in.world_normal);
////    return vec4(0.2, 0.2, 0.3 , 1.0);
////      var output_color = vec4<f32>(in.uv,0.0,1.0);
//
//    var output_color = vec4<f32>(1.0,1.0,1.0,1.0);
//    var output_color = output_color * (textureSample(texture, texture_sampler, in.uv)* vec4<f32>(0.0,0.5,1.0,1.0));
//    var output_color = output_color + uniform_data.color;
//
//    return output_color;
//
//}