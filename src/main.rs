use std::{f64::consts::PI, time::Duration};

use bevy::{
    DefaultPlugins,
    app::{App, FixedUpdate, Plugin, PostUpdate, Startup, Update},
    asset::Assets,
    camera::{Camera, Camera2d, Projection, Viewport},
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        resource::Resource,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, ResMut, Single},
    },
    input::{
        ButtonInput,
        keyboard::{Key, KeyCode},
    },
    math::{
        ops::powf,
        primitives::{Annulus, Circle},
    },
    mesh::{Mesh, Mesh2d},
    sprite_render::{ColorMaterial, MeshMaterial2d},
    time::{Fixed, Time, Timer, TimerMode},
    transform::{
        TransformSystems,
        components::{GlobalTransform, Transform},
    },
    ui::{Display, Node, PositionType, px, widget::Text},
    utils::default,
    window::Window,
};

use crate::units::{ASTRONOMICAL_UNIT, INNER_SOLAR_SYSTEM_RADIUS, Kilometers};

mod units;

// Screen size

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SolarSystemPlugin)
        .add_systems(
            Startup,
            ((setup_viewport, default_viewport_scale).chain(), setup_menu),
        )
        .add_systems(FixedUpdate, camera_controls_system)
        .add_systems(PostUpdate, draw_tooltip.after(TransformSystems::Propagate))
        .run();
}

fn setup_viewport(mut commands: Commands, window: Single<&Window>) {
    let window_size = window.resolution.physical_size().as_vec2();

    commands.spawn((
        Camera2d,
        Camera {
            viewport: Some(Viewport {
                physical_position: (window_size * 0.0).as_uvec2(), // exact top left
                physical_size: (window_size * 1.0).as_uvec2(), // exact window size, full window viewport
                ..default()
            }), // we want this viewport to occupy the entire window.
            clear_color: bevy::camera::ClearColorConfig::Custom(Color::BLACK), // space is black init
            ..default()
        },
        // projection,
    ));
}

fn default_viewport_scale(camera_query: Single<&mut Projection>, window: Single<&Window>) {
    let mut projection = camera_query.into_inner();
    let window_size = window.resolution.physical_size().as_vec2();

    // Camera zoom controls
    if let Projection::Orthographic(projection2d) = &mut *projection {
        projection2d.scale = (INNER_SOLAR_SYSTEM_RADIUS / window_size.x).into();
    }
}

fn setup_menu(mut commands: Commands) {
    commands.spawn((
        HelpText,
        Text::new(
            "SPACE         : Pause\n\
            COMMA / PERIOD: Sim Speed\n\
            MINUS / EQUALS: Zoom in/out",
        ),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            right: px(12),
            align_items: bevy::ui::AlignItems::End,
            justify_content: bevy::ui::JustifyContent::End,
            ..default()
        },
    ));

    commands.spawn((
        TooltipText,
        Text::new("x,y"),
        Node {
            position_type: PositionType::Absolute,

            ..default()
        },
    ));
}

fn draw_tooltip(
    camera_query: Single<(&Camera, &GlobalTransform)>, // can match on particular camera here
    window: Single<&Window>,
    tooltip_query: Single<(&mut Text, &mut Node), With<TooltipText>>,
) {
    let (camera, camera_transform) = *camera_query;
    let (mut tooltip_text, mut tooltip_node) = tooltip_query.into_inner();

    if let Some(cursor_position) = window.cursor_position()
        && let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position)
    {
        tooltip_text.0 = format!("{:.1}, {:.1}", world_pos.x, world_pos.y);

        tooltip_node.left = px(cursor_position.x + 12.0);
        tooltip_node.top = px(cursor_position.y + 12.0);
    }
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
    help_text_query: Single<&mut Node, With<HelpText>>,
) {
    let key = Key::Character("?".into());

    let mut help_node = help_text_query.into_inner();

    if key_input.just_pressed(key.clone()) {
        if help_node.display == Display::None {
            help_node.display = Display::Flex
        } else {
            help_node.display = Display::None
        }
    }
}

fn camera_controls_system(
    camera_query: Single<&mut Projection>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Fixed>>,
) {
    let mut projection = camera_query.into_inner();

    // Camera zoom controls
    if let Projection::Orthographic(projection2d) = &mut *projection {
        if input.pressed(KeyCode::Minus) {
            projection2d.scale *= powf(4.0f32, time.delta_secs());
        }

        if input.pressed(KeyCode::Equal) {
            projection2d.scale *= powf(0.25f32, time.delta_secs());
        }
    }
}

#[derive(Resource)]
struct OrbitTimer {
    current_interval: usize,
    times: [f32; 6], // tick times increasing in duration with index
    timer: Timer,
}

impl OrbitTimer {
    fn slow_down(&mut self) {
        if self.current_interval == self.times.len() - 1 {
            return;
        }

        self.current_interval += 1;
        self.timer
            .set_duration(Duration::from_secs_f32(self.times[self.current_interval]));
    }

    fn speed_up(&mut self) {
        if self.current_interval == 0 {
            return;
        }

        self.current_interval -= 1;
        self.timer
            .set_duration(Duration::from_secs_f32(self.times[self.current_interval]));
    }

    fn toggle_pause(&mut self) {
        if self.timer.is_paused() {
            self.timer.unpause();
        } else {
            self.timer.pause();
        }
    }
}

fn new_orbit_timer() -> OrbitTimer {
    let times = [0.05, 0.1, 0.25, 0.5, 1.0, 2.0]; // in seconds
    let current_interval = times.len() - 1;

    OrbitTimer {
        current_interval,
        times,
        timer: Timer::from_seconds(times[current_interval], TimerMode::Repeating),
    }
}

#[derive(Component)]
struct TooltipText;

#[derive(Component)]
struct HelpText;

#[derive(Component)]
struct CelestialBody;

#[derive(Component)]
struct Name(String);

// Orbit is oversimplified for now, always a circle.
#[derive(Component)]
struct Orbit(Kilometers);

#[derive(Component)]
struct OrbitRing {
    pub planet: Entity,
}

#[derive(Component)]
struct PolarPosition(f32);

pub struct SolarSystemPlugin;

impl Plugin for SolarSystemPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(new_orbit_timer())
            .add_systems(Startup, add_planets)
            .add_systems(
                Update,
                (
                    move_celestial_body,
                    timer_keyboard_controls_system,
                    ui_keyboard_controls_system,
                ),
            );
    }
}

fn add_planets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let abbey_colour = Color::hsl(60.0, 0.75, 0.75);
    let abbey_shape = meshes.add(Circle::new(Kilometers::from(695700.0).into()));

    let chesterton_colour = Color::hsl(240.0, 0.75, 0.75);
    let chesterton_shape = meshes.add(Circle::new(Kilometers::from(6371.0 * 100.0).into()));

    commands.spawn((
        CelestialBody,
        Name("Abbey".to_string()),
        Mesh2d(abbey_shape),
        MeshMaterial2d(materials.add(abbey_colour)),
        Transform::from_xyz(0.0, 0., 0.),
    ));

    let chesterton_orbit_radius = ASTRONOMICAL_UNIT * 1.0;

    let chesterton_id = commands
        .spawn((
            CelestialBody,
            Name("Chesterton".to_string()),
            Orbit(chesterton_orbit_radius),
            PolarPosition(0.0),
            Mesh2d(chesterton_shape),
            MeshMaterial2d(materials.add(chesterton_colour)),
            Transform::from_xyz(chesterton_orbit_radius.into(), 0., 0.),
        ))
        .id();

    commands.spawn((
        OrbitRing {
            planet: chesterton_id,
        },
        Mesh2d(meshes.add(Annulus::new(
            (chesterton_orbit_radius - 100000.0).into(),
            (chesterton_orbit_radius + 100000.0).into(),
        ))),
        MeshMaterial2d(materials.add(Color::srgba(1., 1., 1., 0.15))),
        Transform::from_xyz(0., 0., -1.),
    ));
}

fn move_celestial_body(
    time: Res<Time>,
    mut timer: ResMut<OrbitTimer>,
    mut query: Query<(&Name, &mut PolarPosition, &Orbit, &mut Transform), With<CelestialBody>>,
) {
    if timer.timer.tick(time.delta()).just_finished() {
        for (name, mut polar_position, orbit, mut transform) in query.iter_mut() {
            // Noddy way to make big orbits go slower
            polar_position.0 += 0.01; // 1.0 / f32::from(orbit.0);
            let x = (orbit.0 * polar_position.0.cos()).into();
            let y = (orbit.0 * polar_position.0.sin()).into();
            transform.translation.x = x;
            transform.translation.y = y;
            println!(
                "Planet {} is now at {}/{} of its orbit.",
                name.0,
                polar_position.0,
                2.0 * PI
            );
        }
    }
}
