use bevy::{
    color::Color,
    ecs::{component::Component, entity::Entity, event::Event},
    math::Vec2,
};

use crate::units::Kilometers;

#[derive(Component)]
pub(crate) struct DebugUI;

#[derive(Component)]
pub(crate) struct TooltipText;

#[derive(Component)]
pub(crate) struct CelestialBody;

#[derive(Component)]
pub(crate) struct Name(pub(crate) String);

#[derive(Component)]
pub(crate) struct PlanetHUD {
    pub(crate) target: Entity,
}

#[derive(Component)]
pub(crate) struct PlanetIndicator;

#[derive(Component)]
pub(crate) struct ForPlanet(pub(crate) Entity);

#[derive(Component, Clone, Copy)]
pub(crate) struct BaseColor(pub(crate) Color);

#[derive(Event, Debug)]
pub(crate) struct PlanetClicked {
    pub(crate) planet: Entity,
}

// Orbit is oversimplified for now, always a circle.
#[derive(Component)]
pub(crate) struct Orbiter {
    pub(crate) radius: Kilometers,
    pub(crate) polar_speed: f32,    // radians per second
    pub(crate) polar_position: f32, // radians
}

#[derive(Component, Clone)]
pub(crate) struct OrbitEllipse {
    pub(crate) center: Vec2,
    pub(crate) semi_major: f32,
    pub(crate) semi_minor: f32,
    /// Argument of periapsis in radians.
    pub(crate) rotation: f32,
}
