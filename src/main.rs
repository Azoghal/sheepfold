use bevy::{
    DefaultPlugins,
    app::{App, Startup},
    camera::{Camera, Camera2d, ClearColorConfig, Viewport},
    color::Color,
    ecs::system::{Commands, Single},
    state::{app::AppExtStates, state::States},
    utils::default,
    window::Window,
};
use bevy_egui::{EguiPlugin, PrimaryEguiContext};

mod materials;
mod resources;
mod units;

mod main_menu;
mod settings;
mod solar_system;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum AppState {
    #[default]
    MainMenu,
    Simulator,
    Settings,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .insert_resource(resources::OrbitLineWidthPx::new(0.5))
        .insert_resource(resources::PlanetScaleMultiplier::new(100.))
        .init_state::<AppState>()
        .add_systems(Startup, setup_viewport)
        .add_plugins(main_menu::MainMenuPlugin)
        .add_plugins(settings::SettingsPlugin)
        .add_plugins(solar_system::SolarSystemPlugin)
        .run();
}

fn setup_viewport(mut commands: Commands, window: Single<&Window>) {
    let window_size = window.resolution.physical_size().as_vec2();

    commands.spawn((
        PrimaryEguiContext,
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
}
