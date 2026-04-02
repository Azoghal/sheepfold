use bevy::{
    DefaultPlugins,
    app::App, state::{app::AppExtStates, state::States},
};

mod materials;
mod units;
mod resources;

mod solar_system;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum AppState {
    #[default]
    SplashScreen,
    MainMenu,
    Simulator,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(resources::OrbitLineWidthPx::new(0.5))
        .insert_resource(resources::PlanetScaleMultiplier::new(10000.))
        .init_state::<AppState>()
        // .add_plugins(splash_screen::SplashScreenPlugin)
        // .add_plugins(main_menu::MainMenuPlugin)
        .add_plugins(solar_system::SolarSystemPlugin)
        .run();
}
