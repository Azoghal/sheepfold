mod components;
mod resources;
mod setup;
mod systems;

use bevy::{
    app::{App, FixedUpdate, Plugin, PostUpdate, Update},
    ecs::schedule::{IntoScheduleConfigs, SystemSet},
    state::{condition::in_state, state::OnEnter},
    transform::TransformSystems,
};
use bevy_egui::EguiPrimaryContextPass;

use crate::{
    AppState,
    materials::{DebugMaterialsPlugin, OrbitMaterialPlugin},
    solar_system::{
        setup::setup_system,
        systems::{body_follow_ui, follow_camera_target},
    },
};

use resources::{new_camera_controller, new_orbit_timer};
use setup::{default_viewport_scale, setup_mouse_tooltip};
use systems::{
    apply_camera_scale, camera_controls_system, debug_control_ui, draw_mouse_tooltip, game_menu_ui,
    move_primary_orbiters, move_sub_orbiters, on_planet_clicked,
    orbit_runner_keyboard_controls_system, time_control_ui, update_orbit_line_display,
    update_planet_huds, update_satellite_orbit_centres, view_control_ui,
};

pub struct SolarSystemPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct SimulatorSet;

impl Plugin for SolarSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((OrbitMaterialPlugin, DebugMaterialsPlugin))
            .insert_resource(new_orbit_timer())
            .insert_resource(new_camera_controller())
            .configure_sets(
                EguiPrimaryContextPass,
                SimulatorSet.run_if(in_state(AppState::Simulator)),
            )
            .configure_sets(Update, SimulatorSet.run_if(in_state(AppState::Simulator)))
            .configure_sets(
                FixedUpdate,
                SimulatorSet.run_if(in_state(AppState::Simulator)),
            )
            .configure_sets(
                PostUpdate,
                SimulatorSet.run_if(in_state(AppState::Simulator)),
            )
            .add_observer(on_planet_clicked)
            .add_systems(
                OnEnter(AppState::Simulator),
                (setup_system, default_viewport_scale, setup_mouse_tooltip),
            )
            .add_systems(
                EguiPrimaryContextPass,
                (
                    time_control_ui,
                    view_control_ui,
                    debug_control_ui,
                    game_menu_ui,
                    body_follow_ui,
                )
                    .in_set(SimulatorSet),
            )
            .add_systems(
                FixedUpdate,
                (
                    camera_controls_system,
                    (
                        move_primary_orbiters,
                        move_sub_orbiters,
                        update_satellite_orbit_centres,
                        follow_camera_target,
                    )
                        .chain(),
                )
                    .in_set(SimulatorSet),
            )
            .add_systems(
                PostUpdate,
                (update_planet_huds, draw_mouse_tooltip)
                    .chain()
                    .after(TransformSystems::Propagate)
                    .in_set(SimulatorSet),
            )
            .add_systems(
                Update,
                (
                    apply_camera_scale,
                    orbit_runner_keyboard_controls_system,
                    update_orbit_line_display,
                )
                    .in_set(SimulatorSet),
            );
    }
}
