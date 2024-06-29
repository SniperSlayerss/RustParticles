use bevy::prelude::*;
use crate::components::particle::{Particle, Velocity};

pub fn update_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Particle>>,
) {
    let delta_time = time.delta_seconds();
    let gravity = Vec3::new(0.0, -9.81 * 100.0, 0.0);
    let damping_factor = 0.99;
    let boundary_radius = 150.0;

    for (mut transform, mut velocity) in query.iter_mut() {
        velocity.0 += gravity * delta_time;
        velocity.0 *= damping_factor;

        transform.translation += velocity.0 * delta_time;
        check_and_place_within_bounds(&mut transform, &mut velocity, boundary_radius)
    }
}

fn check_and_place_within_bounds(transform: &mut Transform, velocity: &mut Velocity, boundary_radius: f32) {
    if transform.translation.length() > boundary_radius {
        let direction_to_origin = -transform.translation;
        let normalized_direction = direction_to_origin.normalize();

        transform.translation = -normalized_direction * boundary_radius;

        let restitution = 0.9;
        velocity.0 *= -restitution;
    }
}
