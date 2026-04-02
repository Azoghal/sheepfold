use bevy::ecs::{component::Component, resource::Resource};

#[derive(Resource, Debug, Component, PartialEq, Clone, Copy)]
pub struct OrbitLineWidthPx(f32);

impl OrbitLineWidthPx {
    pub fn new(v: f32) -> Self {
        Self(v)
    }
}

#[derive(Resource, Debug, Component, PartialEq, Clone, Copy)]
pub struct PlanetScaleMultiplier(f32);

impl PlanetScaleMultiplier {
    pub fn new(v: f32) -> Self {
        Self(v)
    }
}
