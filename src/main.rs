use std::{f64::consts::PI, time::Duration};

use bevy::{DefaultPlugins, app::{App, Plugin, Startup, Update}, asset::Assets, camera::Camera2d, color::Color, ecs::{component::Component, query::With, resource::Resource, system::{Commands, Query, Res, ResMut}}, input::{ButtonInput, keyboard::{Key, KeyCode}}, math::primitives::{Circle, ToRing}, mesh::{Mesh, Mesh2d}, sprite_render::{ColorMaterial, MeshMaterial2d}, time::{Time, Timer, TimerMode}, transform::components::Transform};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SolarSystemPlugin)
        .run();
}

fn timer_keyboard_controls_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    // key_input: Res<ButtonInput<Key>>, // if you want a key that appears in multiple locations
    ref mut timer: ResMut<OrbitTimer>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        println!("<INP> Toggle Pause");
        timer.toggle_pause();
    }

    if keyboard_input.just_pressed(KeyCode::Period) {
        println!("<INP> Speed Up");
        timer.speed_up();
    }

    if keyboard_input.just_pressed(KeyCode::Comma) {
        println!("<Inp> Slow Down");
        timer.slow_down();
    }
}


fn ui_keyboard_controls_system(
    // keyboard_input: Res<ButtonInput<KeyCode>>, // if you want a single clickable key on the keyboard
    key_input: Res<ButtonInput<Key>>,
) {
    let key = Key::Character("?".into());
    if key_input.just_pressed(key.clone()) {
        println!("show helpppp");
    }
}

#[derive(Resource)]
struct OrbitTimer{
    current_interval: usize,
    times: [f32; 6], // tick times increasing in duration with index 
    timer:  Timer
}

impl OrbitTimer {
    fn slow_down(&mut self){
        if self.current_interval == self.times.len()-1 {
            return
        }

        self.current_interval += 1;
        self.timer.set_duration(Duration::from_secs_f32( self.times[self.current_interval]));
    }
    
    fn speed_up(&mut self) {
        if self.current_interval == 0 {
            return
        }

        self.current_interval -= 1;
        self.timer.set_duration(Duration::from_secs_f32( self.times[self.current_interval]));
    }

    fn toggle_pause(&mut self){
        if self.timer.is_paused() {
            self.timer.unpause();
        } else {
            self.timer.pause();
        }
    }

    // fn set_pause(&mut self, pause:bool){
    //     self.paused = pause;
    // }
}

fn new_orbit_timer() -> OrbitTimer{
    let times = [0.05, 0.1, 0.25, 0.5, 1.0, 2.0];  // in seconds
    let current_interval = times.len() - 1;

    return OrbitTimer{
        current_interval,
        times,
        timer: Timer::from_seconds(times[current_interval], TimerMode::Repeating)
    }
}

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
        app.insert_resource(new_orbit_timer());
        app.add_systems(Startup, add_planets);
        app.add_systems(Update,  (move_celestial_body,  timer_keyboard_controls_system, ui_keyboard_controls_system));
    }
}


fn add_planets(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {

    commands.spawn(Camera2d);

    let colour = Color::hsl(144., 0.95, 0.7);
    let shape = meshes.add(Circle::new(50.0).to_ring(5.));

    commands.spawn((
        CelestialBody, 
        Name("Chesterton".to_string()),
        Orbit(400000.0), 
        PolarPosition(0.0),
        Mesh2d(shape),
        MeshMaterial2d(materials.add(colour)),
        Transform::from_xyz(-500.0, -75.0, 0.0)
    ));
    commands.spawn((CelestialBody, Name("Abbey".to_string()), Orbit(600000.0), PolarPosition(0.5*PI)));
    commands.spawn((CelestialBody, Name("Petersfield".to_string()), Orbit(800000.0), PolarPosition(1.5*PI)));
}

fn move_celestial_body(time: Res<Time>, mut timer: ResMut<OrbitTimer>, mut query: Query<(&Name, &mut PolarPosition, &Orbit), With<CelestialBody>>) {
    if timer.timer.tick(time.delta()).just_finished() {
        for (name, mut polar_position, orbit) in query.iter_mut() {
            // Noddy way to make big orbits go slower
            polar_position.0 += 1.0/orbit.0;
            println!("Planet {} is now at {}/{} of its orbit.", name.0, polar_position.0, 2.0*PI);
        }
    }
}