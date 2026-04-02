use bevy::{
    app::{App, Plugin, Startup}, camera::{Camera, Camera2d, ClearColorConfig, Viewport}, color::Color, ecs::system::{Commands, Single}, state::state_scoped::DespawnOnExit, ui::{Node, PositionType, widget::Text}, utils::default, window::Window
};
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass};

use crate::AppState;

pub(super) struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .add_systems(EguiPrimaryContextPass, main_menu_ui)
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

fn main_menu_ui(mut contexts: EguiContexts) {
    match contexts.ctx_mut() {
        Ok(context) => {
            bevy_egui::egui::Window::new("Debug").show(context, |ui| {
                ui.label("Sheepfold");
            });
        }
        Err(e) => println!("Error finding egui context {0}", e),
    }
}
