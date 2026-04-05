use bevy::ecs::{component::Component, resource::Resource};

use crate::AppState;

#[derive(Resource, Debug, Component, PartialEq, Clone, Copy)]
pub struct OrbitLineWidthPx(f32);

impl OrbitLineWidthPx {
    pub fn new(v: f32) -> Self {
        Self(v)
    }

    pub fn value(&self) -> f32 {
        self.0
    }

    pub fn set(&mut self, v: f32) {
        self.0 = v;
    }
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct PreviousAppState(pub AppState);
