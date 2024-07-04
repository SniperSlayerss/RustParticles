use crate::components::particle::Particle;
use crate::initialisation::simulation_parameters::SimulationParameters;
use crate::systems::collision_system;
use bevy::prelude::*;

pub fn update_system(
    time: Res<Time>,
    parameters: Res<SimulationParameters>,
    mut query: Query<(&mut Transform, &mut Particle), With<Particle>>,
) {
    let delta_time = time.delta_seconds();
    let gravity = parameters.gravity;
    let restitution = parameters.restitution;
    let boundary_radius = parameters.boundary_radius;

    // Collect particles for collision resolution
    let mut particles: Vec<(Vec3, Vec3, f32)> = query
        .iter()
        .map(|(transform, particle)| (transform.translation, particle.velocity, particle.radius))
        .collect();

    // Resolve collisions and update positions
    collision_system::check_collisions(&mut particles, 8, restitution);

    // Update positions and velocities after collision resolution
    for ((mut transform, mut particle), (new_pos, new_vel, _)) in
        query.iter_mut().zip(particles.iter())
    {
        transform.translation = *new_pos;
        particle.velocity = *new_vel;
    }

    // Check and place within bounds
    for (mut transform, mut particle) in query.iter_mut() {
        //check_and_place_within_bounds(&mut transform, &mut particle, boundary_radius, restitution);
        check_and_place_within_bounds_rectangle(
            &mut transform,
            &mut particle,
            Vec3::new(-500.0, -500.0, -500.0),
            Vec3::new(500.0, 500.0, 500.0),
            restitution,
        );
    }

    // Update positions using Verlet integration
    for (mut transform, mut particle) in query.iter_mut() {
        apply_gravity(&mut particle, gravity, delta_time);
        update_position(&mut transform, &mut particle, delta_time);
    }
}

fn apply_gravity(particle: &mut Particle, gravity: Vec3, delta_time: f32) {
    particle.acceleration += gravity * 30.0 * delta_time;
}

fn update_position(transform: &mut Transform, particle: &mut Particle, delta_time: f32) {
    let current_position = transform.translation;
    let velocity = current_position - particle.position_old;

    // Verlet integration
    let new_position =
        current_position + velocity + particle.acceleration * delta_time * delta_time;

    particle.position_old = current_position;
    transform.translation = new_position;

    // Reset acceleration
    particle.acceleration = Vec3::ZERO;
}

fn check_and_place_within_bounds(
    transform: &mut Transform,
    particle: &mut Particle,
    boundary_radius: f32,
    restitution: f32,
) {
    let current_distance = transform.translation.length();
    let max_distance = boundary_radius - particle.radius;

    if current_distance > max_distance {
        let direction_to_origin = -transform.translation;
        if direction_to_origin.length_squared() > 0.0 {
            let normalized_direction = direction_to_origin.normalize();

            // Place particle on the boundary
            transform.translation = -normalized_direction * max_distance;

            // Reflect velocity with restitution
            let velocity_normal_component = particle.velocity.dot(normalized_direction);
            particle.velocity -=
                normalized_direction * (1.0 + restitution) * velocity_normal_component;
        }
    }
}

fn check_and_place_within_bounds_rectangle(
    transform: &mut Transform,
    particle: &mut Particle,
    min_bounds: Vec3,
    max_bounds: Vec3,
    restitution: f32,
) {
    let mut position = transform.translation;

    // Check and resolve collisions with the boundary on the x-axis
    if position.x - particle.radius < min_bounds.x {
        position.x = min_bounds.x + particle.radius;
        particle.velocity.x = -particle.velocity.x * restitution;
    } else if position.x + particle.radius > max_bounds.x {
        position.x = max_bounds.x - particle.radius;
        particle.velocity.x = -particle.velocity.x * restitution;
    }

    // Check and resolve collisions with the boundary on the y-axis
    if position.y - particle.radius < min_bounds.y {
        position.y = min_bounds.y + particle.radius;
        particle.velocity.y = -particle.velocity.y * restitution;
    } else if position.y + particle.radius > max_bounds.y {
        position.y = max_bounds.y - particle.radius;
        particle.velocity.y = -particle.velocity.y * restitution;
    }

    // Check and resolve collisions with the boundary on the z-axis
    if position.z - particle.radius < min_bounds.z {
        position.z = min_bounds.z + particle.radius;
        particle.velocity.z = -particle.velocity.z * restitution;
    } else if position.z + particle.radius > max_bounds.z {
        position.z = max_bounds.z - particle.radius;
        particle.velocity.z = -particle.velocity.z * restitution;
    }

    transform.translation = position;
}
