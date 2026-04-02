use bevy::{
    DefaultPlugins,
    app::App,
    state::{app::AppExtStates, state::States},
};
use bevy_egui::EguiPlugin;

mod materials;
mod resources;
mod units;

mod main_menu;
mod solar_system;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum AppState {
    #[default]
    MainMenu,
    Simulator,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .insert_resource(resources::OrbitLineWidthPx::new(0.5))
        .insert_resource(resources::PlanetScaleMultiplier::new(10000.))
        .init_state::<AppState>()
        .add_plugins(main_menu::MainMenuPlugin)
        .add_plugins(solar_system::SolarSystemPlugin)
        .run();
}
