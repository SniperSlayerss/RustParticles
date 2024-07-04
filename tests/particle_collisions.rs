use bevy::prelude::*;
use RustParticles::components::particle::Particle;
use RustParticles::initialisation::simulation_parameters::SimulationParameters;
use RustParticles::systems::{check_collisions, particle_spawner_system_tests, update_system};

fn calculate_kinetic_energy(particles: &Vec<(Vec3, Vec3, f32)>) -> f32 {
    particles
        .iter()
        .map(|(_, vel, radius)| 0.5 * radius * vel.length_squared())
        .sum()
}

#[test]
fn test_energy_conservation() {
    let mut app = App::new();

    let substeps = 1;
    let restitution = 0.9;

    app.add_plugins(MinimalPlugins)
        .add_plugins(bevy::transform::TransformPlugin);
    app.insert_resource(SimulationParameters::new());
    app.add_systems(FixedUpdate, update_system::update_system);
    app.add_systems(FixedUpdate, particle_spawner_system_tests);

    app.update();

    // Simulate the passage of time by running the update multiple times
    let seconds = 1001;
    let steps = seconds * 64;
    for _ in 0..steps {
        app.update();
    }

    let mut query = app.world.query::<(&mut Transform, &mut Particle)>();
    let mut particles: Vec<(Vec3, Vec3, f32)> = query
        .iter(&app.world)
        .map(|(transform, particle)| (transform.translation, particle.velocity, particle.radius))
        .collect();

    println!("Particles:{}",particles.len());

    // Print particle information
    //for (i, (position, velocity, radius)) in particles.iter().enumerate() {
    //    println!(
    //        "Particle {}: Position: {:?}, Velocity: {:?}, Radius: {}",
    //        i, position, velocity, radius
    //    );
    //}

    // Check if particles are spawned correctly
    assert!(!particles.is_empty(), "No particles were spawned!");

    // Calculate initial kinetic energy
    let initial_energy = calculate_kinetic_energy(&particles);
    println!("Initial kinetic energy: {}", initial_energy);

    // Check collisions and update particles
    check_collisions(&mut particles, substeps, restitution);

    // Calculate final kinetic energy
    let final_energy = calculate_kinetic_energy(&particles);
    println!("Final kinetic energy: {}", final_energy);

    // Print final state of particles
    //for (i, (position, velocity, radius)) in particles.iter().enumerate() {
    //    println!(
    //        "Particle {}: Position: {:?}, Velocity: {:?}, Radius: {}",
    //        i, position, velocity, radius
    //    );
    //}

    // Assert energy conservation
    assert!(
        final_energy <= initial_energy,
        "Energy after collision should not exceed energy before collision"
    );
}
