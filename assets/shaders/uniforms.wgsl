#import bevy_render::globals::Globals

@group(0) @binding(1)
var<uniform> globals: Globals;
@group(2) @binding(1)
var<uniform> res: vec2<f32>;

@fragment
fn fragment(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    // Change red channel over time
    // return vec4<f32>(abs(sin(globals.time)), 0.0, 0.0, 1.0);
    let st = pos.xy / res;
    return vec4<f32>(st.x, 1-st.y, 0.0, 1.0);
}