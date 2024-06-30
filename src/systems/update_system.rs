
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

    // Collect particles for density and pressure calculations
    let particles: Vec<(Vec3, f32)> = query.iter().map(|(transform, particle)| {
        (transform.translation, particle.density)
    }).collect();

    // First loop: Update velocity and density
    for (transform, mut particle) in query.iter_mut() {
        particle.velocity += parameters.gravity * delta_time;
        particle.density = calculate_density(&transform, &particles, parameters.smoothing_radius);
    }

    // Second loop: Update velocity with pressure force
    //for (transform, mut particle) in query.iter_mut() {
     //   let pressure_force: Vec3 =
     //       calculate_pressure_force(transform.translation, &particles, &parameters);
     //   let pressure_accel: Vec3 = pressure_force / particle.density;
     //   particle.velocity += pressure_accel * delta_time;
   // }

    // Third loop: Update position and check boundaries
    for (mut transform, mut particle) in query.iter_mut() {
        transform.translation += particle.velocity * delta_time;
        check_and_place_within_bounds(&mut transform, &mut particle, parameters.boundary_radius);
    }
}

fn calculate_density(
    current_particle: &Transform,
    particles: &[(Vec3, f32)],
    smoothing_radius: f32,
) -> f32 {
    let mut density: f32 = 0.0;
    let mass: f32 = 1.0;

    for (position, _) in particles.iter() {
        let dist: f32 = (*position - current_particle.translation).length();
        let influence: f32 = smoothing_kernel(smoothing_radius, dist);
        density += mass * influence;
    }

    density
}

fn calculate_pressure_force(
    particle_pos: Vec3,
    particles: &[(Vec3, f32)],
    parameters: &SimulationParameters,
) -> Vec3 {
    let mut pressure_force: Vec3 = Vec3::ZERO;

    for (position, density) in particles.iter() {
        let dist: f32 = (*position - particle_pos).length();
        if dist > 0.0 {
            let dir: Vec3 = (*position - particle_pos) / dist;
            let slope: f32 = smoothing_kernel_derivative(parameters.smoothing_radius, dist);
            let density: f32 = *density;
            pressure_force +=
                -density_to_pressure(density, &parameters) * dir * slope * 1.0 / density;
        }
    }
    pressure_force
}

fn density_to_pressure(density: f32, parameters: &SimulationParameters) -> f32 {
    let density_error = density - parameters.target_density;
    let pressure = density_error * parameters.pressure_mult;
    pressure
}

fn smoothing_kernel(radius: f32, dist: f32) -> f32 {
    let volume: f32 = PI * radius.powi(8) / 4.0;
    let value: f32 = (radius * radius - dist).max(0.0);
    (value * value * value) / volume
}

fn smoothing_kernel_derivative(radius: f32, dist: f32) -> f32 {
    if dist >= radius {
        return 0.0;
    }
    let f: f32 = radius * radius - dist * dist;
    let scale: f32 = -24.0 / (PI * radius.powi(8));
    scale * dist * f * f
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

        let restitution = 0.8;
        let velocity_normal_component = particle.velocity.dot(normalized_direction);
        particle.velocity -= normalized_direction * (1.0 + restitution) * velocity_normal_component;

        // Optionally cap the velocity to prevent particles from gaining too much speed
//        let max_velocity = 10.0; // Adjust this value as needed
  //      if particle.velocity.length() > max_velocity {
    //        particle.velocity = particle.velocity.normalize() * max_velocity;
      //  }

        return true;
    }
    false
}


