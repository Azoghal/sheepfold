use std::f32::consts::TAU;

const INDICATOR_HALF_SIZE: f32 = 5.0;

use bevy::{
    app::AppExit, camera::{Camera, Projection}, ecs::{
        entity::Entity, message::MessageWriter, observer::On, query::With, system::{Commands, Query, Res, ResMut, Single}
    }, input::{ButtonInput, keyboard::KeyCode}, math::vec3, sprite_render::MeshMaterial2d, state::state::NextState, time::{Fixed, Time}, transform::components::{GlobalTransform, Transform}, ui::{ComputedNode, Display, Node, px, widget::Text}, window::Window
};

use bevy_egui::{EguiContexts, egui};

use crate::{
    AppState,
    materials::OrbitMaterial,
    resources::{OrbitLineWidthPx, PreviousAppState}, solar_system::components::Name,
};

use super::components::{
    CelestialBody, DebugUI, OrbitEllipse, Orbiter, PlanetClicked, PlanetHUD, TooltipText,
};
use super::resources::{CameraController, OrbitRunner};

pub(super) fn apply_camera_scale(
    camera_controller: Res<CameraController>,
    camera_query: Single<&mut Projection>,
) {
    let mut projection = camera_query.into_inner();
    if let Projection::Orthographic(projection2d) = &mut *projection {
        projection2d.scale = camera_controller.scale;
    }
}

pub(super) fn follow_camera_target(
    camera_controller: Res<CameraController>,
    targets: Query<&GlobalTransform>,
    camera_query: Single<(&Camera, &mut Transform)>,
) {
    // get optional target out of camera target, update camera to move with it.
    if let Some(target) = camera_controller.target
        && let Ok(target_transform) = targets.get(target) {
            let (_, mut transform) = camera_query.into_inner();
            let target_pos = target_transform.translation();
            transform.translation.x = target_pos.x;
            transform.translation.y = target_pos.y;
        }
}

pub(super) fn time_control_ui(mut contexts: EguiContexts, mut orbit_runner: ResMut<OrbitRunner>) {
    match contexts.ctx_mut() {
        Ok(context) => {
            egui::Window::new("Time").show(context, |ui| {
                if orbit_runner.paused {
                    ui.label("PAUSED");
                } else {
                    ui.small("Running");
                }
                ui.label(format!("Sim. Speed: {0}x", orbit_runner.timestep));
                if ui.button("Speed Up (.)").clicked() {
                    orbit_runner.speed_up();
                    println!("<UI Inp> Speed Up");
                }
                if ui.button("Slow Down (,)").clicked() {
                    orbit_runner.slow_down();
                    println!("<UI Inp> Slow Down");
                }
                if ui.button("Pause (space)").clicked() {
                    orbit_runner.toggle_pause();
                    println!("<UI Inp> Pause");
                }
            });
        }
        Err(e) => println!("Error finding egui context {0}", e),
    }
}

pub(super) fn view_control_ui(
    mut contexts: EguiContexts,
    mut camera_controller: ResMut<CameraController>,
) {
    match contexts.ctx_mut() {
        Ok(context) => {
            egui::Window::new("View").show(context, |ui| {
                if ui.button("Zoom Out (-)").clicked() {
                    camera_controller.zoom_out();
                }
                if ui.button("Zoom In (=)").clicked() {
                    camera_controller.zoom_in();
                }
            });
        }
        Err(e) => println!("Error finding egui context {0}", e),
    }
}

pub(super) fn body_follow_ui(
    mut contexts: EguiContexts,
    body_query: Query<(Entity, &Name), With<CelestialBody>>,
    mut camera_controller: ResMut<CameraController>,
    camera_query: Single<(&Camera, &mut Transform)>
) {
    match contexts.ctx_mut() {
        Ok(context) => {
            egui::Window::new("Celestial Bodies").show(context, |ui| {
                if ui.button("Reset").clicked() {
                    camera_controller.target = None;
                    let (_, mut camera_transform) = camera_query.into_inner();
                    camera_transform.translation.x = 0.0;
                    camera_transform.translation.y = 0.0;
                }
                for (id, body_name) in body_query.iter() {
                    if ui.button(body_name.0.to_string()).clicked() {
                        camera_controller.target = Some(id);
                    }
                }
            });
        }
        Err(e) => println!("Error finding egui context {0}", e),
    }
}

pub(super) fn debug_control_ui(
    mut contexts: EguiContexts,
    mut debug_ui_query: Query<&mut Node, With<DebugUI>>,
) {
    match contexts.ctx_mut() {
        Ok(context) => {
            egui::Window::new("Debug").show(context, |ui| {
                if ui.button("Show All").clicked() {
                    set_all_debug_ui_visible(true, &mut debug_ui_query);
                }
                if ui.button("Hide All").clicked() {
                    set_all_debug_ui_visible(false, &mut debug_ui_query);
                }
            });
        }
        Err(e) => println!("Error finding egui context {0}", e),
    }
}

pub(super) fn game_menu_ui(
    mut contexts: EguiContexts,
    mut app_exit_writer: MessageWriter<AppExit>,
    mut app_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
) {
    match contexts.ctx_mut() {
        Ok(context) => {
            egui::Window::new("Game").show(context, |ui| {
                if ui.button("Main Menu").clicked() {
                    app_state.set(AppState::MainMenu);
                }
                if ui.button("Settings").clicked() {
                    commands.insert_resource(PreviousAppState(AppState::Simulator));
                    app_state.set(AppState::Settings);
                }
                if ui.button("Quit to Desktop").clicked() {
                    app_exit_writer.write(AppExit::Success);
                }
            });
        }
        Err(e) => println!("Error finding egui context {0}", e),
    }
}

fn set_all_debug_ui_visible(visible: bool, query: &mut Query<&mut Node, With<DebugUI>>) {
    let desired_display = if visible {
        Display::Flex
    } else {
        Display::None
    };
    for mut node in query.iter_mut() {
        node.display = desired_display;
    }
}

pub(super) fn draw_mouse_tooltip(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    tooltip_query: Single<(&mut Text, &mut Node), With<TooltipText>>,
) {
    let (camera, camera_transform) = *camera_query;
    let (mut tooltip_text, mut tooltip_node) = tooltip_query.into_inner();

    if let Some(cursor_position) = window.cursor_position()
        && let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position)
    {
        tooltip_text.0 = format!("{:.1}, {:.1}", world_pos.x, world_pos.y);
        tooltip_node.left = px(cursor_position.x + 12.0);
        tooltip_node.top = px(cursor_position.y + 12.0);
    }
}

pub(super) fn update_planet_huds(
    mut huds: Query<(&mut Node, &ComputedNode, &PlanetHUD)>,
    targets: Query<&GlobalTransform>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = *camera_query;

    for (mut node, computed_node, hud) in huds.iter_mut() {
        let Ok(target_transform) = targets.get(hud.target) else {
            continue;
        };

        let world_position = target_transform.translation();
        let half_width = computed_node.size().x / 2.0;

        if let Ok(viewport_pos) = camera.world_to_viewport(camera_transform, world_position) {
            // Centre the container horizontally on the planet.
            // Shift up by half the indicator diameter so the circle sits on the planet position.
            node.left = px(viewport_pos.x - half_width);
            node.top = px(viewport_pos.y - INDICATOR_HALF_SIZE);
        }
    }
}

pub(super) fn on_planet_clicked(
    event: On<PlanetClicked>,
    mut camera_controller: ResMut<CameraController>,
) {
    camera_controller.target = Some(event.planet);
}

pub(super) fn update_orbit_line_display(
    mut orbit_materials: ResMut<bevy::asset::Assets<OrbitMaterial>>,
    camera_controller: Res<CameraController>,
    line_width: Res<OrbitLineWidthPx>,
    orbit_query: Query<(&OrbitEllipse, &MeshMaterial2d<OrbitMaterial>)>,
) {
    for (_ellipse, material_handle) in &orbit_query {
        if let Some(material) = orbit_materials.get_mut(material_handle) {
            material.world_per_pixel = camera_controller.scale;
            material.line_width_px = line_width.value();
        }
    }
}

pub(super) fn orbit_runner_keyboard_controls_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut timer: ResMut<OrbitRunner>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        println!("<Key INP> Toggle Pause");
        timer.toggle_pause();
    }
    if keyboard_input.just_pressed(KeyCode::Period) {
        println!("<Key INP> Speed Up");
        timer.speed_up();
    }
    if keyboard_input.just_pressed(KeyCode::Comma) {
        println!("<Key Inp> Slow Down");
        timer.slow_down();
    }
}

pub(super) fn camera_controls_system(
    input: Res<ButtonInput<KeyCode>>,
    mut camera_controller: ResMut<CameraController>,
    time: Res<Time<Fixed>>,
) {
    if input.pressed(KeyCode::Minus) {
        camera_controller.zoom_out_continuous(time.delta_secs());
    }
    if input.pressed(KeyCode::Equal) {
        camera_controller.zoom_in_continuous(time.delta_secs());
    }
}

pub(super) fn move_celestial_body(
    time: Res<Time>,
    orbit_runner: Res<OrbitRunner>,
    targets: Query<&GlobalTransform>,
    mut query: Query<(&mut Orbiter, &mut Transform), With<CelestialBody>>,
) {
    let simulated_time_delta_secs = time.delta_secs() * orbit_runner.timestep;

    if !orbit_runner.paused {
        for (mut orbiter, mut transform) in query.iter_mut() {

            let mut barycentre = vec3(0.0, 0.0, 0.0);

            if let Ok(barycenter_transform) = targets.get(orbiter.barycentre_target){
                barycentre.x = barycenter_transform.translation().x;
                barycentre.y = barycenter_transform.translation().y;
            }
            
            orbiter.polar_position += orbiter.polar_speed * simulated_time_delta_secs;
            if orbiter.polar_position > TAU {
                orbiter.polar_position %= TAU
            }
            let relative_x: f32 = (orbiter.radius * orbiter.polar_position.cos()).into();
            let relative_y: f32 = (orbiter.radius * orbiter.polar_position.sin()).into();

            transform.translation.x = barycentre.x + relative_x;
            transform.translation.y = barycentre.y + relative_y;
        }
    }
}
