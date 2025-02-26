#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    //let t_1 = abs(sin(globals.time * 2.0));
    let base_color = vec3<f32>(47.0 / 255.0, 180.0 / 255.0, 229.0 / 255.0);

    return vec4<f32>(base_color, 1.0);
}
