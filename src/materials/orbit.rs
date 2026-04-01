use bevy::{
    prelude::*, reflect::TypePath, render::render_resource::AsBindGroup, shader::ShaderRef, sprite_render::{AlphaMode2d, Material2d, Material2dPlugin}
};

/// GPU-side uniform block — must match the struct in orbit.wgsl exactly,
/// including alignment (std140: each field is 4 bytes, vec2 is 8 bytes).
#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct OrbitMaterial {
    /// Ellipse centre in world space.
    #[uniform(0)]
    pub center: Vec2,
    /// Semi-major axis in world units.
    #[uniform(0)]
    pub semi_major: f32,
    /// Semi-minor axis in world units.
    #[uniform(0)]
    pub semi_minor: f32,
    /// Ellipse rotation in radians.
    #[uniform(0)]
    pub rotation: f32,
    /// World units per screen pixel (derived from camera zoom).
    #[uniform(0)]
    pub world_per_pixel: f32,
    /// Desired line width in screen pixels.
    #[uniform(0)]
    pub line_width_px: f32,
    /// Line colour.
    #[uniform(0)]
    pub color: LinearRgba,
}

impl Material2d for OrbitMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/orbit.wgsl".into()
    }

    // Transparent — we discard fully-transparent fragments in the shader
    // but still need Bevy's pipeline to blend the anti-aliased edge.
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

pub struct OrbitMaterialPlugin;

impl Plugin for OrbitMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<OrbitMaterial>::default());
    }
}
