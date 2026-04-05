#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct OrbitMaterial {
    centre: vec2<f32>,
    semi_major: f32,
    semi_minor: f32,
    rotation: f32,
    world_per_pixel: f32,
    line_width_px: f32,
    color: vec4<f32>,
}

@group(2) @binding(0)
var<uniform> material: OrbitMaterial;

// Approximate signed distance to an axis-aligned ellipse centred at the origin.
// Exact for circles (semi_major == semi_minor). Good approximation for ellipses.
// Negative inside, positive outside.
fn sdf_ellipse(p: vec2<f32>, ab: vec2<f32>) -> f32 {
    let k1 = length(p / ab);
    let k2 = length(p / (ab * ab));
    return (k1 - 1.0) * k1 / k2;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Translate into ellipse-local frame and un-rotate.
    let p = in.world_position.xy - material.centre;
    let cos_r = cos(-material.rotation);
    let sin_r = sin(-material.rotation);
    let local = vec2<f32>(
        p.x * cos_r - p.y * sin_r,
        p.x * sin_r + p.y * cos_r,
    );

    // World-space signed distance to the ellipse curve, converted to pixels.
    let pixel_dist = sdf_ellipse(local, vec2<f32>(material.semi_major, material.semi_minor))
                     / material.world_per_pixel;

    // Anti-aliased line with fixed pixel width regardless of zoom.
    let half_width = material.line_width_px * 0.5;
    let alpha = 1.0 - smoothstep(half_width - 1.0, half_width + 1.0, abs(pixel_dist));

    if alpha <= 0.0 {
        discard;
    }

    return vec4<f32>(material.color.rgb, material.color.a * alpha);
}
