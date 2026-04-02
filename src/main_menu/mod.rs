use bevy::{
    app::{App, Plugin, Startup},
    ecs::system::Commands,
    state::state_scoped::DespawnOnExit,
    ui::{Node, PositionType, widget::Text},
    utils::default,
};
use bevy_egui::{
    EguiContexts, EguiPlugin, EguiPrimaryContextPass,
    egui::{ Window},
};

use crate::AppState;

pub(super) struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .add_systems(EguiPrimaryContextPass, main_menu_ui)
            .add_systems(Startup, menu_setup);
    }
}

fn menu_setup(mut commands: Commands) {
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
            Window::new("Debug").show(context, |ui| {
                ui.label("Sheepfold");
            });
        }
        Err(e) => println!("Error finding egui context {0}", e),
    }
}
