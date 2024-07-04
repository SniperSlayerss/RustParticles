use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct SimulationParameters {
    pub boundary_radius: f32,
    pub gravity: Vec3,
    pub restitution: f32,
}

impl SimulationParameters {
    pub fn new() -> Self {
        Self {
            boundary_radius: 400.0,
            gravity: Vec3::new(0.0, -628.84, 0.0),
            restitution: 0.8,
        }
    }
}

