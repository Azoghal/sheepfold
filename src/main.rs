use bevy::{
    DefaultPlugins,
    app::App,
};

mod debug_material;
mod orbit_material;
mod units;

mod solar_system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(solar_system::SolarSystemPlugin)
        .run();
}
