use bevy::{DefaultPlugins, app::{App, Startup, Update}, ecs::{component::Component, query::With, schedule::IntoScheduleConfigs, system::{Commands, Query}}};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, (update_people, greet_people).chain()))
        .run();
}

fn hello_world(){
    println!("hello world!");
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Bob Smith".to_string())));
    commands.spawn((Person, Name("Bill Smith".to_string())));
    commands.spawn((Person, Name("Barnaby Smith".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
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

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);