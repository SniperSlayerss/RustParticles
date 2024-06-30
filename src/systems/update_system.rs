use crate::components::particle::Particle;
use crate::simulation_parameters::SimulationParameters;
use bevy::prelude::*;
use std::f32::consts::PI;

pub fn update_system(
    time: Res<Time>,
    parameters: Res<SimulationParameters>,
    mut query: Query<(&mut Transform, &mut Particle), With<Particle>>,
) {
    let delta_time = time.delta_seconds();

    // Collect particles for density calculations
    let mut particles: Vec<(Vec3, f32, Vec3, f32)> = query
        .iter()
        .map(|(transform, particle)| {
            (
                transform.translation,
                particle.density,
                particle.velocity,
                particle.radius,
            )
        })
        .collect();

    // First loop: Update density
    for (transform, mut particle) in query.iter_mut() {
        particle.density = calculate_density(&transform, &particles, parameters.smoothing_radius);
    }

    // Second loop: Calculate and apply density gradient force
    for (transform, mut particle) in query.iter_mut() {
        let density_gradient_force: Vec3 = calculate_density_gradient_force(
            transform.translation,
            &particles,
            parameters.smoothing_radius,
        );
        particle.velocity += density_gradient_force * delta_time;

        // Debug output
        //println!("Particle Position: {:?}", transform.translation);
        //println!("Particle Density: {:?}", particle.density);
        //println!("Density Gradient Force: {:?}", density_gradient_force);
        //println!("Updated Velocity: {:?}", particle.velocity);
    }

    // Third loop: Resolve collisions
    resolve_collisions(&mut particles, 0.7);

    // Fourth loop: Update position and check boundaries
    for (mut transform, mut particle) in query.iter_mut() {
        //particle.velocity += parameters.gravity * delta_time * 0.7;
        transform.translation += particle.velocity * delta_time;
        check_and_place_within_bounds(&mut transform, &mut particle, parameters.boundary_radius);
    }
}



fn calculate_density(
    current_particle: &Transform,
    particles: &[(Vec3, f32, Vec3, f32)],
    smoothing_radius: f32,
) -> f32 {
    let mut density: f32 = 0.0;
    let mass: f32 = 10.0;  // Adjust mass based on your simulation setup

    for (position, _, _, _) in particles.iter() {
        let dist: f32 = (*position - current_particle.translation).length();
        //println!("Distance between {:?} and {:?}: {}", current_particle.translation, position, dist);
        if dist < smoothing_radius {
            let influence: f32 = smoothing_kernel(smoothing_radius, dist);
            density += mass * influence;
            //println!("Distance: {}, Influence: {}, Partial Density: {}", dist, influence, mass * influence);
        }
    }

    // Debug output for density calculation
    println!("Calculated Density for Position {:?}: {}", current_particle.translation, density);

    density
}



fn calculate_density_gradient_force(
    particle_pos: Vec3,
    particles: &[(Vec3, f32, Vec3, f32)],
    smoothing_radius: f32,
) -> Vec3 {
    let mut density_gradient: Vec3 = Vec3::ZERO;

    for (position, density, _, _) in particles.iter() {
        let dist: f32 = (*position - particle_pos).length();
        if dist > 0.0 && dist < smoothing_radius {
            let dir: Vec3 = (*position - particle_pos) / dist;
            let influence: f32 = smoothing_kernel_derivative(smoothing_radius, dist);
            density_gradient += dir * influence * *density;
        }
    }

    // Debug output for density gradient force calculation
    let scaled_force = -density_gradient * 100.0; // Adjust the scaling factor as needed

    // println!(
    // "Calculated Density Gradient Force for Position {:?}: {:?}",
    // particle_pos,
    //   scaled_force
    // );

    scaled_force
}



fn smoothing_kernel(radius: f32, dist: f32) -> f32 {
    if dist >= radius {
        return 0.0;
    }
    let q = dist / radius;
    let sigma = 315.0 / (64.0 * std::f32::consts::PI * radius.powi(9));  // 3D normalization factor
    sigma * (1.0 - q * q).powi(3)
}

fn smoothing_kernel_derivative(radius: f32, dist: f32) -> f32 {
    if dist >= radius {
        return 0.0;
    }
    let q = dist / radius;
    let sigma = 45.0 / (std::f32::consts::PI * radius.powi(6));  // 3D normalization factor
    sigma * (1.0 - q).powi(2)
}
fn resolve_collisions(particles: &mut Vec<(Vec3, f32, Vec3, f32)>, restitution: f32) {
    let num_particles = particles.len();

    for i in 0..num_particles {
        for j in (i + 1)..num_particles {
            let (pos_i, _, vel_i, radius_i) = particles[i];
            let (pos_j, _, vel_j, radius_j) = particles[j];

            let dist_vec = pos_j - pos_i;
            let dist = dist_vec.length();
            let min_dist = radius_i + radius_j;

            if dist < min_dist {
                let normal = dist_vec.normalize();
                let relative_velocity = vel_j - vel_i;
                let separating_velocity = relative_velocity.dot(normal);

                if separating_velocity < 0.0 {
                    let new_sep_velocity = -separating_velocity * restitution;
                    let delta_velocity = new_sep_velocity - separating_velocity;

                    let total_mass = 2.0; // assuming equal mass for simplicity
                    let impulse = delta_velocity / total_mass;
                    let impulse_per_mass = impulse * normal;

                    particles[i].2 -= impulse_per_mass; // Adjust velocity of particle i
                    particles[j].2 += impulse_per_mass; // Adjust velocity of particle j

                    // Separate the particles to prevent overlap
                    let correction = normal * (min_dist - dist) / 2.0;
                    particles[i].0 -= correction;
                    particles[j].0 += correction;
                }
            }
        }
    }
}

fn check_and_place_within_bounds(
    transform: &mut Transform,
    particle: &mut Particle,
    boundary_radius: f32,
) -> bool {
    let current_distance = transform.translation.length();
    let max_distance = boundary_radius - particle.radius;

    if current_distance > max_distance {
        let direction_to_origin = -transform.translation;
        let normalized_direction = direction_to_origin.normalize();

        // Apply a small buffer to ensure the particle is slightly inside the boundary
        let buffer_distance = 0.01;
        transform.translation = -normalized_direction * (max_distance - buffer_distance);

        let restitution = 0.7;
        let velocity_normal_component = particle.velocity.dot(normalized_direction);
        particle.velocity -= normalized_direction * (1.0 + restitution) * velocity_normal_component;

        return true;
    }
    false
}
