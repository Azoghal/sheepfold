#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct RingDebugMaterial {
    world_per_pixel: f32,
    world_radius: f32,
    line_width_px: f32,
    color: vec4<f32>,
}

@group(2) @binding(0)
var<uniform> material: RingDebugMaterial;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let world_pos: vec2<f32> = in.world_position.xy;

    // World-space distance from this fragment to the circle surface.
    let world_dist = abs(length(world_pos) - material.world_radius);

    // Convert to screen-pixel distance.  Dividing by world_per_pixel keeps
    // the rendered line width constant in pixels regardless of camera zoom.
    let pixel_dist = world_dist / material.world_per_pixel;

    // Smooth anti-aliased alpha: 1.0 at the line centre, 0.0 one pixel outside.
    let half_width = material.line_width_px * 0.5;
    let alpha = 1.0 - smoothstep(half_width - 1.0, half_width + 1.0, pixel_dist);

    if alpha <= 0.0 {
        discard;
    }

    return vec4<f32>(material.color.rgb, material.color.a * alpha);
}
