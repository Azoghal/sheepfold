use std::f64::consts::PI;

use bevy::{DefaultPlugins, app::{App, Plugin, Startup, Update}, ecs::{component::Component, query::With, resource::Resource, system::{Commands, Query, Res, ResMut}}, input::{ButtonInput, keyboard::{Key, KeyCode}}, time::{Time, Timer, TimerMode}};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SolarSystemPlugin)
        .run();
}

fn add_planets(mut commands: Commands) {
    commands.spawn((CelestialBody, Name("Chesterton".to_string()), Orbit(400000.0), PolarPosition(0.0)));
    commands.spawn((CelestialBody, Name("Abbey".to_string()), Orbit(600000.0), PolarPosition(0.5*PI)));
    commands.spawn((CelestialBody, Name("Petersfield".to_string()), Orbit(800000.0), PolarPosition(1.5*PI)));
}

fn move_celestial_body(time: Res<Time>, mut timer: ResMut<OrbitTimer>, mut query: Query<(&Name, &mut PolarPosition, &Orbit), With<CelestialBody>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for (name, mut polar_position, orbit) in query.iter_mut() {
            // Noddy way to make big orbits go slower
            polar_position.0 += 1.0/orbit.0;
            println!("Planet {} is now at {}/{} of its orbit.", name.0, polar_position.0, 2.0*PI);
        }
    }
}

fn keyboard_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    key_input: Res<ButtonInput<Key>>,
) {
    if keyboard_input.just_pressed(KeyCode::Period) {
        println!("speed uppppp")
    }

    if keyboard_input.just_pressed(KeyCode::Comma) {
        println!("slow downnnn")
    }

    let key = Key::Character("?".into());
    if key_input.just_pressed(key.clone()) {
        println!("show helpppp");
    }
}

#[derive(Resource)]
struct OrbitTimer(Timer);

#[derive(Component)]
struct CelestialBody;

#[derive(Component)]
struct Name(String);

// Orbit is oversimplified for now, always a circle.
#[derive(Component)]
struct Orbit(f64);

#[derive(Component)]
struct PolarPosition(f64);

pub struct SolarSystemPlugin;

impl Plugin for SolarSystemPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(OrbitTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_planets);
        app.add_systems(Update,  (move_celestial_body,  keyboard_input_system));
    }
}
