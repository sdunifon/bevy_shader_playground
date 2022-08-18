#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

struct FragmentInput {
  @builtin(front_facing) is_front: bool,
  @builtin(position) frag_coor: vec4<f32>,
  #import bevy_pbr::mesh_vertex_output
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var N = normalize(in.world_normal);
    return vec4(0.0, 1.0, 0.0, 1.0);
}