use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{Material2d, Material2dPlugin},
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct UvDebugMaterial {}

impl Material2d for UvDebugMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/debug.wgsl".into()
    }

    // fn vertex_shader() -> ShaderRef {
    //     "shaders/uv_debug.wgsl".into()
    // }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct RingDebugMaterial {
#[uniform(0)]
    pub world_per_pixel: f32,
    pub world_radius: f32,
    pub line_width_px: f32,
    pub color: LinearRgba,
}

impl Material2d for RingDebugMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/ring.wgsl".into()
    }

    // fn vertex_shader() -> ShaderRef {
    //     "shaders/uv_debug.wgsl".into()
    // }
}

pub struct DebugMaterialsPlugin;

impl Plugin for DebugMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            Material2dPlugin::<UvDebugMaterial>::default(), 
            Material2dPlugin::<RingDebugMaterial>::default(),
        ));
    }

}
