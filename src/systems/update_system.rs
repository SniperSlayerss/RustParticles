use crate::components::particle::Particle;
use crate::simulation_parameters::SimulationParameters;
use bevy::prelude::*;

pub fn update_system(
    time: Res<Time>,
    parameters: Res<SimulationParameters>,
    mut query: Query<(&mut Transform, &mut Particle), With<Particle>>,
) {
    let delta_time = time.delta_seconds();
    let gravity = parameters.gravity;

    // Apply gravity
    for (_, mut particle) in query.iter_mut() {
        apply_gravity(&mut particle, gravity, delta_time);
    }

    // Check and place within bounds
    for (mut transform, mut particle) in query.iter_mut() {
        check_and_place_within_bounds(&mut transform, &mut particle, parameters.boundary_radius);
    }

    // Collect particles for collision resolution
    let mut particles: Vec<(Vec3, Vec3, f32)> = query
        .iter()
        .map(|(transform, particle)| (transform.translation, particle.velocity, particle.radius))
        .collect();

    // Resolve collisions and update positions
    check_collisions(&mut particles, &mut query);

    // Update positions using Verlet integration
    for (mut transform, mut particle) in query.iter_mut() {
        update_position(&mut transform, &mut particle, delta_time);
    }
}

fn check_collisions(
    particles: &mut Vec<(Vec3, Vec3, f32)>,
    query: &mut Query<(&mut Transform, &mut Particle), With<Particle>>,
) {
    for _ in 0..8 {
        resolve_collisions(particles, 0.7);
    }

    // Update positions and velocities after collision resolution
    for ((mut transform, mut particle), (new_pos, new_vel, _)) in
        query.iter_mut().zip(particles.iter())
    {
        transform.translation = *new_pos;
        particle.velocity = *new_vel;
    }
}

fn check_and_place_within_bounds(
    transform: &mut Transform,
    particle: &mut Particle,
    boundary_radius: f32,
) {
    let current_distance = transform.translation.length();
    let max_distance = boundary_radius - particle.radius;

    if current_distance > max_distance {
        let direction_to_origin = -transform.translation;
        let normalized_direction = direction_to_origin.normalize();

        transform.translation = -normalized_direction * max_distance;

        let restitution = 0.7;
        let velocity_normal_component = particle.velocity.dot(normalized_direction);
        particle.velocity -= normalized_direction * (1.0 + restitution) * velocity_normal_component;
    }
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

fn apply_gravity(particle: &mut Particle, gravity: Vec3, delta_time: f32) {
    particle.acceleration += gravity * delta_time * 0.7;
}

fn resolve_collisions(particles: &mut Vec<(Vec3, Vec3, f32)>, restitution: f32) {
    let object_count = particles.len();
    for i in 0..object_count {
        let (pos1, vel1, radius1) = particles[i];
        for k in (i + 1)..object_count {
            let (pos2, vel2, radius2) = particles[k];
            let collision_axis = pos1 - pos2;
            let dist = collision_axis.length();
            let min_dist = radius1 + radius2;
            if dist < min_dist {
                // Normal vector of collision
                let n = collision_axis / dist;
                // Penetration distance
                let delta = min_dist - dist;

                // Correct positions to resolve collision
                particles[i].0 += 0.5 * delta * n;
                particles[k].0 -= 0.5 * delta * n;

                // Calculate relative velocity in terms of the normal direction
                let rel_vel = vel1 - vel2;
                let vel_along_normal = rel_vel.dot(n);

                // Do not resolve if velocities are separating
                if vel_along_normal > 0.0 {
                    continue;
                }

                // Calculate impulse scalar
                let j = -(1.0 + restitution) * vel_along_normal;
                let j = j / (1.0 / radius1 + 1.0 / radius2);

                // Apply impulse to the velocities
                let impulse = j * n;
                particles[i].1 += impulse / radius1;
                particles[k].1 -= impulse / radius2;
            }
        }
    }
}
