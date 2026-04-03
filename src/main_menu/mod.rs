use bevy::{
    app::{App, AppExit, Plugin},
    camera::Projection,
    ecs::{
        message::MessageWriter,
        schedule::{IntoScheduleConfigs, SystemSet},
        system::{Commands, ResMut, Single},
    },
    state::{
        condition::in_state,
        state::{NextState, OnEnter},
    },
};
use bevy_egui::{EguiContexts, EguiPrimaryContextPass};

use crate::{AppState, resources::PreviousAppState};

pub(super) struct MainMenuPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MainMenuSet;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            EguiPrimaryContextPass,
            MainMenuSet.run_if(in_state(AppState::MainMenu)),
        )
        .add_systems(OnEnter(AppState::MainMenu), reset_viewport_scale)
        .add_systems(EguiPrimaryContextPass, main_menu_ui.in_set(MainMenuSet));
    }
}

fn reset_viewport_scale(camera_query: Single<&mut Projection>) {
    let mut projection = camera_query.into_inner();
    if let Projection::Orthographic(projection2d) = &mut *projection {
        projection2d.scale = 1.0;
    }
}

fn main_menu_ui(
    mut contexts: EguiContexts,
    mut app_exit_writer: MessageWriter<AppExit>,
    mut app_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
) {
    match contexts.ctx_mut() {
        Ok(context) => {
            bevy_egui::egui::CentralPanel::default().show(context, |ui| {
                ui.heading("Sheepfold");
                if ui.button("Start Simulator").clicked() {
                    app_state.set(AppState::Simulator);
                }
                if ui.button("Settings").clicked() {
                    commands.insert_resource(PreviousAppState(AppState::MainMenu));
                    app_state.set(AppState::Settings);
                }
                if ui.button("Exit").clicked() {
                    app_exit_writer.write(AppExit::Success);
                }
            });
        }
        Err(e) => println!("Error finding egui context {0}", e),
    }
}
