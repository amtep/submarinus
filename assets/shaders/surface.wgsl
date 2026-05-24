#import bevy_sprite::mesh2d_vertex_output::VertexOutput

const PI: f32 = 3.14159265359;

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> material_color: vec4<f32>;

fn plot(in: vec2f) -> f32 {
    return smoothstep(0.4, 0.0, abs((in.y - 0.5) * 2.0 - sin(in.x * PI)));
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(material_color.rgb, plot(in.uv));
}
