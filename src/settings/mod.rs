use bevy::{
    app::{App, Plugin, Startup},
    camera::Projection,
    ecs::{
        schedule::IntoScheduleConfigs,
        system::{Res, ResMut, Single},
    },
    state::{condition::in_state, state::NextState},
};
use bevy_egui::{EguiContexts, EguiPrimaryContextPass};

use crate::{resources::PreviousAppState, AppState};

pub(super) struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            EguiPrimaryContextPass,
            settings_ui.run_if(in_state(AppState::Settings)),
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

fn settings_ui(
    mut contexts: EguiContexts,
    mut app_state: ResMut<NextState<AppState>>,
    previous_state: Option<Res<PreviousAppState>>,
) {
    match contexts.ctx_mut() {
        Ok(context) => {
            bevy_egui::egui::CentralPanel::default().show(context, |ui| {
                ui.heading("Settings");
                if ui.button("Setting 1").clicked() {
                    println!("gotta do something init")
                }
                if ui.button("Back").clicked() {
                    let back_to = previous_state.map(|s| s.0).unwrap_or(AppState::MainMenu);
                    app_state.set(back_to);
                }
            });
        }
        Err(e) => println!("Error finding egui context {0}", e),
    }
}
