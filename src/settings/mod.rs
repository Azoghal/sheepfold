use bevy::{
    app::{App, Plugin},
    camera::Projection,
    ecs::{
        schedule::{IntoScheduleConfigs, SystemSet},
        system::{Res, ResMut, Single},
    },
    state::{
        condition::in_state,
        state::{NextState, OnEnter},
    },
};
use bevy_egui::{EguiContexts, EguiPrimaryContextPass};

use crate::{
    AppState,
    resources::{OrbitLineWidthPx, PreviousAppState},
};

pub(super) struct SettingsPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct SettingsSet;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            EguiPrimaryContextPass,
            SettingsSet.run_if(in_state(AppState::Settings)),
        )
        .add_systems(OnEnter(AppState::Settings), reset_viewport_scale)
        .add_systems(EguiPrimaryContextPass, settings_ui.in_set(SettingsSet));
    }
}

fn reset_viewport_scale(camera_query: Single<&mut Projection>) {
    let mut projection = camera_query.into_inner();
    if let Projection::Orthographic(projection2d) = &mut *projection {
        projection2d.scale = 1.0;
    }
}

fn settings_ui(
    mut contexts: EguiContexts,
    mut app_state: ResMut<NextState<AppState>>,
    previous_state: Option<Res<PreviousAppState>>,
    mut orbit_line_width: ResMut<OrbitLineWidthPx>,
) {
    match contexts.ctx_mut() {
        Ok(context) => {
            bevy_egui::egui::CentralPanel::default().show(context, |ui| {
                ui.heading("Settings");

                ui.label("Orbit line width (px)");
                let mut width = orbit_line_width.value();
                if ui
                    .add(bevy_egui::egui::Slider::new(&mut width, 0.1..=5.0))
                    .changed()
                {
                    orbit_line_width.set(width);
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
