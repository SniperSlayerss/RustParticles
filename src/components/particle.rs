use bevy::prelude::*;

#[derive(Component)]
pub struct Particle {
    pub radius: f32,
}

#[derive(Component)]
pub struct Velocity(pub Vec3);

