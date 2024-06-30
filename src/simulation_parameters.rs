use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct SimulationParameters {
    pub boundary_radius: f32,
    pub gravity: Vec3,
}

impl SimulationParameters {
    pub fn new() -> Self {
        Self {
            boundary_radius: 400.0,
            gravity: Vec3::new(0.0, -9.81 * 10000.0 , 0.0),
        }
    }
}
