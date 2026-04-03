use bevy::{
    app::{App, AppExit, Plugin, Startup},
    camera::Projection,
    ecs::{
        message::MessageWriter,
        schedule::IntoScheduleConfigs,
        system::{ResMut, Single},
    },
    state::{condition::in_state, state::NextState},
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
        .add_systems(Startup, default_viewport_scale);
    }
}

fn default_viewport_scale(camera_query: Single<&mut Projection>) {
    let mut projection = camera_query.into_inner();
    if let Projection::Orthographic(projection2d) = &mut *projection {
        projection2d.scale = 1.0;
    }
}

fn main_menu_ui(
    mut contexts: EguiContexts,
    mut app_exit_writer: MessageWriter<AppExit>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    match contexts.ctx_mut() {
        Ok(context) => {
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
