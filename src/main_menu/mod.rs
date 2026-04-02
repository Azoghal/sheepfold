use bevy::{
    app::{App, AppExit, Plugin, Startup},
    camera::{Camera, Camera2d, ClearColorConfig, Viewport},
    color::Color,
    ecs::{
        message::MessageWriter,
        schedule::IntoScheduleConfigs,
        system::{Commands, ResMut, Single},
    },
    state::{condition::in_state, state::NextState, state_scoped::DespawnOnExit},
    ui::{Node, PositionType, widget::Text},
    utils::default,
    window::Window,
};
use bevy_egui::{EguiContexts, EguiPrimaryContextPass};

use crate::AppState;

pub(super) struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            EguiPrimaryContextPass,
            main_menu_ui.run_if(in_state(AppState::MainMenu)),
        )
        .add_systems(Startup, menu_setup);
    }
}

fn menu_setup(mut commands: Commands, window: Single<&Window>) {
    let window_size = window.resolution.physical_size().as_vec2();

    commands.spawn((
        DespawnOnExit(AppState::MainMenu),
        Camera2d,
        Camera {
            viewport: Some(Viewport {
                physical_position: (window_size * 0.0).as_uvec2(),
                physical_size: (window_size * 1.0).as_uvec2(),
                ..default()
            }),
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
    ));

    commands.spawn((
        DespawnOnExit(AppState::MainMenu),
        Text::new("hello"),
        Node {
            position_type: PositionType::Absolute,
            ..default()
        },
    ));
}

fn main_menu_ui(
    mut contexts: EguiContexts,
    mut app_exit_writer: MessageWriter<AppExit>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    match contexts.ctx_mut() {
        Ok(context) => {
            println!("running main menu ui");
            bevy_egui::egui::CentralPanel::default().show(context, |ui| {
                ui.heading("Sheepfold");
                if ui.button("Start Simulator").clicked() {
                    app_state.set(AppState::Simulator);
                }
                // if ui.button("Settings").clicked() {
                //     println!("Settings clicked");
                // }
                if ui.button("Exit").clicked() {
                    app_exit_writer.write(AppExit::Success);
                }
            });
        }
        Err(e) => println!("Error finding egui context {0}", e),
    }
}
