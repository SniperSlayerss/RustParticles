use bevy::prelude::*;

#[derive(Component)]
pub struct Particle {
    pub radius: f32,
    pub velocity: Vec3,
    pub density: f32,
}


