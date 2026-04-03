mod components;
mod resources;
mod setup;
mod systems;

use bevy::{
    app::{App, FixedUpdate, Plugin, PostUpdate, Update},
    ecs::schedule::IntoScheduleConfigs,
    state::{condition::in_state, state::OnEnter},
    transform::TransformSystems,
};
use bevy_egui::EguiPrimaryContextPass;

use crate::{
    AppState,
    materials::{DebugMaterialsPlugin, OrbitMaterialPlugin},
    solar_system::systems::game_menu_ui,
};

use resources::{new_camera_controller, new_orbit_timer};
use setup::{add_planets, add_star, default_viewport_scale, setup_mouse_tooltip};
use systems::{
    apply_camera_scale, camera_controls_system, debug_control_ui, draw_mouse_tooltip,
    move_celestial_body, orbit_runner_keyboard_controls_system, time_control_ui,
    update_orbit_line_display, update_screen_labels, view_control_ui,
};

pub struct SolarSystemPlugin;

// TODO find a better way to handle not running any of these systems when not in AppState::Simulator

impl Plugin for SolarSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((OrbitMaterialPlugin, DebugMaterialsPlugin))
            .insert_resource(new_orbit_timer())
            .insert_resource(new_camera_controller())
            .add_systems(
                EguiPrimaryContextPass,
                (
                    time_control_ui,
                    view_control_ui,
                    debug_control_ui,
                    game_menu_ui,
                )
                    .run_if(in_state(AppState::Simulator)),
            )
            .add_systems(
                OnEnter(AppState::Simulator),
                (
                    (add_star, add_planets).chain(),
                    default_viewport_scale,
                    setup_mouse_tooltip,
                ),
            )
            .add_systems(
                FixedUpdate,
                camera_controls_system.run_if(in_state(AppState::Simulator)),
            )
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
                )
                    .run_if(in_state(AppState::Simulator)),
            )
            .add_systems(
                FixedUpdate,
                move_celestial_body.run_if(in_state(AppState::Simulator)),
            );
    }
}
