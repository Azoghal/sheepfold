use std::f32::consts::TAU;

use bevy::{
    asset::Assets,
    color::{Alpha, Color, LinearRgba},
    ecs::{
        entity::Entity,
        observer::On,
        system::{Commands, Query, Res, ResMut, Single},
    },
    math::{
        Vec2,
        primitives::{Circle, Rectangle},
    },
    mesh::{Mesh, Mesh2d},
    picking::events::{Click, Out, Over, Pointer},
    sprite_render::{ColorMaterial, MeshMaterial2d},
    state::state_scoped::DespawnOnExit,
    text::{TextColor, TextFont},
    transform::components::Transform,
    ui::{
        AlignItems, BackgroundColor, BorderRadius, Display, FlexDirection, Node, PositionType, Val,
        widget::Text,
    },
    utils::default,
    window::Window,
};

use crate::{
    AppState,
    materials::OrbitMaterial,
    resources::PlanetScaleMultiplier,
    units::{ASTRONOMICAL_UNIT, INNER_SOLAR_SYSTEM_RADIUS, Kilometers},
};

use super::components::{
    BaseColor, CelestialBody, DebugUI, ForPlanet, Name, OrbitEllipse, Orbiter, PlanetClicked,
    PlanetHUD, PlanetIndicator, SatelliteBody, TooltipText,
};
use super::resources::CameraController;

const INDICATOR_SIZE: f32 = 10.0;

pub(super) fn default_viewport_scale(
    window: Single<&Window>,
    mut camera_controller: ResMut<CameraController>,
) {
    let window_size = window.resolution.physical_size().as_vec2();
    camera_controller.scale = (INNER_SOLAR_SYSTEM_RADIUS / window_size.x).into();
}

pub(super) fn setup_mouse_tooltip(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(AppState::Simulator),
        DebugUI,
        TooltipText,
        Text::new("x,y"),
        Node {
            position_type: PositionType::Absolute,
            display: Display::None,
            ..default()
        },
    ));
}

pub(super) fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut orbit_materials: ResMut<Assets<OrbitMaterial>>,
    planet_scale: Res<PlanetScaleMultiplier>,
) {
    let star_id = add_star(&mut commands, &mut meshes, &mut materials);

    add_all_satellites(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut orbit_materials,
        planet_scale,
        star_id,
    );
}

fn add_star(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    let colour = Color::hsl(60.0, 0.75, 0.75);
    let name = "Enlil";

    let star_id = commands
        .spawn((
            DespawnOnExit(AppState::Simulator),
            CelestialBody,
            Name(name.to_string()),
            Mesh2d(meshes.add(Circle::new(Kilometers::from(695700.0).into()))),
            MeshMaterial2d(materials.add(colour)),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .id();

    spawn_planet_hud(commands, star_id, name, colour, 14.0);

    star_id
}

struct SatelliteSpec {
    name: String,
    colour: Color,
    radius: Kilometers,
    orbit_radius: Kilometers,
    orbit_period: f32, // seconds
    satellites: Option<Vec<SatelliteSpec>>,
}

fn add_all_satellites(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    orbit_materials: &mut ResMut<Assets<OrbitMaterial>>,
    planet_scale: Res<PlanetScaleMultiplier>,
    star_id: Entity,
) {
    let scale = planet_scale.value();

    let planets = [
        SatelliteSpec {
            name: "Shamhat".to_string(),
            colour: Color::hsl(0.0, 0.85, 0.75),
            radius: Kilometers::from(3500.0 * scale),
            orbit_radius: ASTRONOMICAL_UNIT * 0.4,
            orbit_period: 30. * 24. * 60. * 60.,
            satellites: None,
        },
        SatelliteSpec {
            name: "Enkidu".to_string(),
            colour: Color::hsl(240.0, 0.75, 0.75),
            radius: Kilometers::from(6371.0 * scale),
            orbit_radius: ASTRONOMICAL_UNIT * 1.0,
            orbit_period: 365. * 24. * 60. * 60.,
            satellites: None,
        },
        SatelliteSpec {
            name: "Humbaba".to_string(),
            colour: Color::hsl(120.0, 0.75, 0.75),
            radius: Kilometers::from(4000.0 * scale),
            orbit_radius: ASTRONOMICAL_UNIT * 1.7,
            orbit_period: 710. * 24. * 60. * 60.,
            satellites: Some(vec![SatelliteSpec {
                name: "Inanna".to_string(),
                colour: Color::hsl(180.0, 0.75, 0.75),
                radius: Kilometers::from(400.0 * scale),
                orbit_radius: Kilometers::from(40000.0),
                orbit_period: 18. * 24. * 60. * 60.,
                satellites: None,
            }]),
        },
    ];

    for planet in planets {
        spawn_satellite(
            commands,
            meshes,
            materials,
            orbit_materials,
            planet,
            star_id,
            false,
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
        centre: ellipse.centre,
        semi_major: ellipse.semi_major,
        semi_minor: ellipse.semi_minor,
        rotation: ellipse.rotation,
        world_per_pixel: 1.0,
        line_width_px: 0.5,
        color: LinearRgba::from(color),
    });
    commands.spawn((
        DespawnOnExit(AppState::Simulator),
        Transform::from_translation(ellipse.centre.extend(-0.1)),
        Mesh2d(meshes.add(Rectangle::new(safe_size, safe_size))),
        MeshMaterial2d(material),
        ellipse,
    ));
}

fn spawn_satellite(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    orbit_materials: &mut ResMut<Assets<OrbitMaterial>>,
    planet: SatelliteSpec,
    system_barycentre: Entity,
    is_sub_satellite: bool,
) {
    let polar_speed = TAU / planet.orbit_period;
    println!("{0}:{1}", planet.name, polar_speed);

    let mut entity_cmd = commands.spawn((
        DespawnOnExit(AppState::Simulator),
        CelestialBody,
        Name(planet.name.to_string()),
        Orbiter {
            barycentre_target: system_barycentre,
            radius: planet.orbit_radius,
            polar_speed,
            polar_position: 0.0,
        },
        Mesh2d(meshes.add(Circle::new(planet.radius.into()))),
        MeshMaterial2d(materials.add(planet.colour)),
        Transform::from_xyz(0.0, 0.0, 0.0), // all body locations sorted out in update.
    ));
    if is_sub_satellite {
        entity_cmd.insert(SatelliteBody);
    }
    let planet_id = entity_cmd.id();

    spawn_orbit(
        commands,
        meshes,
        orbit_materials,
        OrbitEllipse {
            centre: Vec2::ZERO,
            semi_major: planet.orbit_radius.into(),
            semi_minor: planet.orbit_radius.into(),
            rotation: 0.0,
        },
        planet.colour.with_alpha(0.3),
    );

    spawn_planet_hud(commands, planet_id, &planet.name, planet.colour, 9.0);

    if let Some(satellites) = planet.satellites {
        for satellite in satellites.into_iter() {
            spawn_satellite(
                commands,
                meshes,
                materials,
                orbit_materials,
                satellite,
                planet_id,
                true,
            );
        }
    }
}

fn spawn_planet_hud(
    commands: &mut Commands,
    target: Entity,
    name: &str,
    colour: Color,
    font_size: f32,
) {
    commands
        .spawn((
            DespawnOnExit(AppState::Simulator),
            PlanetHUD { target },
            Node {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(2.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    PlanetIndicator,
                    ForPlanet(target),
                    BaseColor(colour),
                    Node {
                        width: Val::Px(INDICATOR_SIZE),
                        height: Val::Px(INDICATOR_SIZE),
                        border_radius: BorderRadius::all(Val::Percent(50.0)),
                        ..default()
                    },
                    BackgroundColor(colour.with_alpha(0.6)),
                ))
                .observe(on_indicator_over)
                .observe(on_indicator_out)
                .observe(on_hud_click);

            parent
                .spawn((
                    ForPlanet(target),
                    Text::new(name.to_string()),
                    TextFont {
                        font_size,
                        ..default()
                    },
                    TextColor(Color::WHITE.with_alpha(0.7)),
                    Node::default(),
                ))
                .observe(on_label_over)
                .observe(on_label_out)
                .observe(on_hud_click);
        });
}

fn on_hud_click(ev: On<Pointer<Click>>, query: Query<&ForPlanet>, mut commands: Commands) {
    if let Ok(for_planet) = query.get(ev.entity) {
        commands.trigger(PlanetClicked {
            planet: for_planet.0,
        });
    }
}

fn on_indicator_over(ev: On<Pointer<Over>>, mut query: Query<(&mut BackgroundColor, &BaseColor)>) {
    if let Ok((mut bg, base)) = query.get_mut(ev.entity) {
        bg.0 = base.0;
    }
}

fn on_indicator_out(ev: On<Pointer<Out>>, mut query: Query<(&mut BackgroundColor, &BaseColor)>) {
    if let Ok((mut bg, base)) = query.get_mut(ev.entity) {
        bg.0 = base.0.with_alpha(0.6);
    }
}

fn on_label_over(ev: On<Pointer<Over>>, mut query: Query<&mut TextColor>) {
    if let Ok(mut color) = query.get_mut(ev.entity) {
        color.0 = Color::WHITE;
    }
}

fn on_label_out(ev: On<Pointer<Out>>, mut query: Query<&mut TextColor>) {
    if let Ok(mut color) = query.get_mut(ev.entity) {
        color.0 = Color::WHITE.with_alpha(0.7);
    }
}
