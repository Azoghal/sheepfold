use std::f32::consts::TAU;

use bevy::{
    asset::Assets,
    camera::{Camera, Camera2d, ClearColorConfig, Viewport},
    color::{Alpha, Color, LinearRgba},
    ecs::system::{Commands, ResMut, Single},
    math::{
        Vec2,
        primitives::{Circle, Rectangle},
    },
    mesh::{Mesh, Mesh2d},
    sprite_render::{ColorMaterial, MeshMaterial2d},
    text::TextFont,
    transform::components::Transform,
    ui::{Display, Node, PositionType, widget::Text},
    utils::default,
    window::Window,
};

use bevy_egui::PrimaryEguiContext;

use crate::{
    materials::OrbitMaterial,
    units::{ASTRONOMICAL_UNIT, INNER_SOLAR_SYSTEM_RADIUS, Kilometers},
};

use super::components::{
    CelestialBody, DebugUI, Name, OrbitEllipse, Orbiter, ScreenLabel, TooltipText,
};
use super::resources::CameraController;

const PLANET_DRAW_SCALE: f32 = 100.0;

pub(super) fn setup_viewport(mut commands: Commands, window: Single<&Window>) {
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

pub(super) fn default_viewport_scale(
    window: Single<&Window>,
    mut camera_controller: ResMut<CameraController>,
) {
    let window_size = window.resolution.physical_size().as_vec2();
    camera_controller.scale = (INNER_SOLAR_SYSTEM_RADIUS / window_size.x).into();
}

pub(super) fn setup_mouse_tooltip(mut commands: Commands) {
    commands.spawn((
        DebugUI,
        TooltipText,
        Text::new("x,y"),
        Node {
            position_type: PositionType::Absolute,
            display: Display::None, // Toggled on via debug ui
            ..default()
        },
    ));
}

pub(super) fn add_star(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let enlil_colour = Color::hsl(60.0, 0.75, 0.75);
    let enlil_shape = meshes.add(Circle::new(Kilometers::from(695700.0).into()));

    let name = "Enlil";

    let enlil_id = commands
        .spawn((
            CelestialBody,
            Name(name.to_string()),
            Mesh2d(enlil_shape),
            MeshMaterial2d(materials.add(enlil_colour)),
            Transform::from_xyz(0.0, 0., 0.),
        ))
        .id();

    commands.spawn((
        Text::new(name.to_string()),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            align_items: bevy::ui::AlignItems::Center,
            ..default()
        },
        ScreenLabel { target: enlil_id },
    ));
}

struct PlanetSpec {
    name: String,
    colour: Color,
    radius: Kilometers,
    orbit_radius: Kilometers,
    orbit_period: f32, // seconds
}

pub(super) fn add_planets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut orbit_materials: ResMut<Assets<OrbitMaterial>>,
) {
    let shamhat = PlanetSpec {
        name: "Shamhat".to_string(),
        colour: Color::hsl(0.0, 0.85, 0.75),
        radius: Kilometers::from(3500.0 * PLANET_DRAW_SCALE),
        orbit_radius: ASTRONOMICAL_UNIT * 0.4,
        orbit_period: 30. * 24. * 60. * 60.,
    };

    let enkidu = PlanetSpec {
        name: "Enkidu".to_string(),
        colour: Color::hsl(240.0, 0.75, 0.75),
        radius: Kilometers::from(6371.0 * PLANET_DRAW_SCALE),
        orbit_radius: ASTRONOMICAL_UNIT * 1.0,
        orbit_period: 365. * 24. * 60. * 60.,
    };

    let humbaba = PlanetSpec {
        name: "Humbaba".to_string(),
        colour: Color::hsl(120.0, 0.75, 0.75),
        radius: Kilometers::from(4000.0 * PLANET_DRAW_SCALE),
        orbit_radius: ASTRONOMICAL_UNIT * 1.7,
        orbit_period: 710. * 24. * 60. * 60.,
    };

    for planet in [shamhat, enkidu, humbaba] {
        spawn_planet(
            &mut commands,
            &mut meshes,
            &mut materials,
            &mut orbit_materials,
            planet,
        );
    }
}

fn spawn_orbit(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    orbit_materials: &mut Assets<OrbitMaterial>,
    ellipse: OrbitEllipse,
    color: Color,
) {
    let safe_size = ellipse.semi_major * 2.2;

    let material = orbit_materials.add(OrbitMaterial {
        center: ellipse.center,
        semi_major: ellipse.semi_major,
        semi_minor: ellipse.semi_minor,
        rotation: ellipse.rotation,
        world_per_pixel: 1.0, // overwritten each frame by update_orbit_line_display
        line_width_px: 0.5,
        color: LinearRgba::from(color),
    });

    commands.spawn((
        Transform::from_translation(ellipse.center.extend(-0.1)),
        Mesh2d(meshes.add(Rectangle::new(safe_size, safe_size))),
        MeshMaterial2d(material),
        ellipse,
    ));
}

fn spawn_planet(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    orbit_materials: &mut ResMut<Assets<OrbitMaterial>>,
    planet: PlanetSpec,
) {
    let planet_shape = meshes.add(Circle::new(planet.radius.into()));
    let polar_speed = TAU / planet.orbit_period;
    println!("{0}:{1}", planet.name, polar_speed);

    let planet_id = commands
        .spawn((
            CelestialBody,
            Name(planet.name.to_string()),
            Orbiter {
                radius: planet.orbit_radius,
                polar_speed,
                polar_position: 0.0,
            },
            Mesh2d(planet_shape),
            MeshMaterial2d(materials.add(planet.colour)),
            Transform::from_xyz(planet.orbit_radius.into(), 0., 0.),
        ))
        .id();

    commands.spawn((
        Text::new(planet.name.to_string()),
        TextFont {
            font_size: 9.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            ..default()
        },
        ScreenLabel { target: planet_id },
    ));

    spawn_orbit(
        commands,
        meshes,
        orbit_materials,
        OrbitEllipse {
            center: Vec2::ZERO,
            semi_major: planet.orbit_radius.into(),
            semi_minor: planet.orbit_radius.into(),
            rotation: 0.0,
        },
        planet.colour.with_alpha(0.3),
    );
}
