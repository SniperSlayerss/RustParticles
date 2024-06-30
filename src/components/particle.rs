use bevy::prelude::*;

#[derive(Component)]
pub struct Particle {
    pub position_old: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub radius: f32,
}
