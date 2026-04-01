use bevy::{
    DefaultPlugins,
    app::App,
};

mod materials;
mod units;

mod solar_system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(solar_system::SolarSystemPlugin)
        .run();
}
