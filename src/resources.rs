use bevy::ecs::{component::Component, resource::Resource};

use crate::AppState;

#[derive(Resource, Debug, Component, PartialEq, Clone, Copy)]
pub struct OrbitLineWidthPx(f32);

impl OrbitLineWidthPx {
    pub fn new(v: f32) -> Self {
        Self(v)
    }
}

#[derive(Resource, Debug, Component, PartialEq, Clone, Copy)]
pub struct PlanetScaleMultiplier(f32);

#[derive(Resource, Debug, Clone, Copy)]
pub struct PreviousAppState(pub AppState);

impl PlanetScaleMultiplier {
    pub fn new(v: f32) -> Self {
        Self(v)
    }
}
