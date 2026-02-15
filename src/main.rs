use bevy::{DefaultPlugins, app::{App, Plugin, Startup, Update}, ecs::{component::Component, query::With, resource::Resource, schedule::IntoScheduleConfigs, system::{Commands, Query, Res, ResMut}}, time::{Time, Timer, TimerMode}};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Bob Smith".to_string())));
    commands.spawn((Person, Name("Bill Smith".to_string())));
    commands.spawn((Person, Name("Barnaby Smith".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Bob Smith" {
            name.0 = "Bob John".to_string();
            break; // We don't need to change any other names.
        }
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update,  (update_people, greet_people).chain());
    }
}
