#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct OrbitMaterial {
    // Ellipse center in world space
    center: vec2<f32>,
    // Semi-major and semi-minor axes in world units
    semi_major: f32,
    semi_minor: f32,
    // Rotation of the ellipse in radians (argument of periapsis)
    rotation: f32,
    // The current camera zoom (world units per pixel)
    // i.e. world_units_visible / screen_pixels
    world_per_pixel: f32,
    // Desired line width in pixels
    line_width_px: f32,
    // Line color (RGBA)
    color: vec4<f32>,
}

@group(2) @binding(0)
var<uniform> material: OrbitMaterial;

// Approximate signed distance to an axis-aligned ellipse.
// Returns the distance from point `p` to the ellipse surface.
// Negative inside, positive outside.
// Based on Inigo Quilez's ellipse SDF approximation.
fn sdf_ellipse(p: vec2<f32>, ab: vec2<f32>) -> f32 {
    // Fold into first quadrant for symmetry
    let q = abs(p);
    
    // Initial guess on the ellipse
    var t = vec2<f32>(0.70710678, 0.70710678); // 45 degrees
    
    // Newton iterations to find the closest point on the ellipse
    for (var i = 0; i < 4; i++) {
        let e = ab * t;
        let r = vec2<f32>(
            ab.x * (q.x - e.x),
            ab.y * (q.y - e.y)
        );
        let s = vec2<f32>(e.x - q.x, e.y - q.y);
        
        // Gradient of the ellipse equation
        let tx = clamp(
            (r.x * ab.x + s.x * ab.y) / (dot(r, r) + dot(s, s)),
            -1.0, 1.0
        );
        let ty = clamp(
            (r.y * ab.y + s.y * ab.x) / (dot(r, r) + dot(s, s)),
            -1.0, 1.0
        );
        t = normalize(vec2<f32>(tx, ty));
    }
    
    let nearest = ab * t;
    let diff = q - nearest;
    
    // Sign: negative inside the ellipse
    let inside = select(1.0, -1.0, (q.x / ab.x) * (q.x / ab.x) + (q.y / ab.y) * (q.y / ab.y) < 1.0);
    return length(diff) * inside;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Transform fragment world position relative to ellipse center
    let world_pos = in.world_position.xy - material.center;
    
    // Rotate into the ellipse's local frame (un-rotate by `rotation`)
    let cos_r = cos(-material.rotation);
    let sin_r = sin(-material.rotation);
    let local_pos = vec2<f32>(
        world_pos.x * cos_r - world_pos.y * sin_r,
        world_pos.x * sin_r + world_pos.y * cos_r,
    );
    
    // Compute world-space distance to ellipse curve
    let world_dist = sdf_ellipse(local_pos, vec2<f32>(material.semi_major, material.semi_minor));
    
    // Convert world-space distance to pixel distance
    let pixel_dist = world_dist / material.world_per_pixel;
    
    // Half-width of the line in pixels (add 1px for AA ramp)
    let half_width = material.line_width_px * 0.5;
    
    // Smooth anti-aliased alpha: 1.0 at centre of line, 0.0 one pixel outside edge
    let alpha = smoothstep(half_width + 1.0, half_width - 1.0, abs(pixel_dist));
    
    if alpha <= 0.0 {
        discard;
    }
    
    return vec4<f32>(material.color.rgb, material.color.a * alpha);
}
