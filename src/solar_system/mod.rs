mod components;
mod resources;
mod setup;
mod systems;

use bevy::{
    app::{App, FixedUpdate, Plugin, PostUpdate, Startup, Update},
    ecs::schedule::IntoScheduleConfigs,
    transform::TransformSystems,
};
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};

use crate::materials::{DebugMaterialsPlugin, OrbitMaterialPlugin};

use resources::{new_camera_controller, new_orbit_timer};
use setup::{add_planets, add_star, default_viewport_scale, setup_mouse_tooltip, setup_viewport};
use systems::{
    apply_camera_scale, camera_controls_system, debug_control_ui, draw_mouse_tooltip,
    move_celestial_body, orbit_runner_keyboard_controls_system, time_control_ui,
    update_orbit_line_display, update_screen_labels, view_control_ui,
};

pub struct SolarSystemPlugin;

impl Plugin for SolarSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((OrbitMaterialPlugin, DebugMaterialsPlugin))
            .add_plugins(EguiPlugin::default())
            .insert_resource(new_orbit_timer())
            .insert_resource(new_camera_controller())
            .add_systems(
                EguiPrimaryContextPass,
                (time_control_ui, view_control_ui, debug_control_ui),
            )
            .add_systems(
                Startup,
                (
                    (add_star, add_planets).chain(),
                    (setup_viewport, default_viewport_scale).chain(),
                    setup_mouse_tooltip,
                ),
            )
            .add_systems(FixedUpdate, camera_controls_system)
            .add_systems(
                PostUpdate,
                draw_mouse_tooltip.after(TransformSystems::Propagate),
            )
            .add_systems(
                Update,
                (
                    apply_camera_scale,
                    orbit_runner_keyboard_controls_system,
                    update_screen_labels,
                    update_orbit_line_display,
                ),
            )
            .add_systems(FixedUpdate, move_celestial_body);
    }
}
