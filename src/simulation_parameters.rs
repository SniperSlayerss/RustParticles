use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct SimulationParameters {
    pub target_density: f32,
    pub pressure_mult: f32,
    pub boundary_radius: f32,
    pub gravity: Vec3,
    pub smoothing_radius: f32,
}

impl SimulationParameters {
    pub fn new() -> Self {
        Self {
            target_density: 1000.0,
            pressure_mult: 2000.0,
            boundary_radius: 500.0,
            gravity: Vec3::new(0.0, -9.81 * 50.0, 0.0),
            smoothing_radius: 10.0,
        }
    }
}
