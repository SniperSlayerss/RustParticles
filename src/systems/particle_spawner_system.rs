use crate::components::particle::Particle;
use crate::initialisation::simulation_parameters::SimulationParameters;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn particle_spawner_system(
    mut commands: Commands,
    time: Res<Time>,
    mut elapsed: Local<f32>,
    _parameters: Res<SimulationParameters>,
) {
    let spawn_interval = 0.05; // spawn a particle every second
    *elapsed += time.delta_seconds();

    if *elapsed >= spawn_interval {
        *elapsed = 0.0;

        let radius = 5.0;
        let particle_shape = shapes::Circle {
            radius,
            ..Default::default()
        };

        for i in 0..4 {
            // Particle 1 initial setup
            let x1: f32 = 0.0;
            let y1: f32 = 0.0 - (i as f32) * 10.0;
            let initial_velocity1 = Vec3::new(300.0, -100.0 - (i as f32) * 10.0, 0.0);
            let position1 = Vec3::new(x1, y1, 0.0);
            let position_old1 = position1 - initial_velocity1 * time.delta_seconds();

            commands.spawn((
                Particle {
                    position_old: position_old1,
                    velocity: initial_velocity1, // Correctly set initial velocity
                    acceleration: Vec3::ZERO,
                    radius,
                },
                ShapeBundle {
                    path: GeometryBuilder::build_as(&particle_shape),
                    spatial: SpatialBundle {
                        transform: Transform::from_translation(position1),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Fill::color(Color::WHITE),
                Stroke::new(Color::BLACK, 1.0),
            ));
        }
    }
}

pub fn particle_spawner_system_tests(
    mut commands: Commands,
    time: Res<Time>,
    mut elapsed: Local<f32>,
    _parameters: Res<SimulationParameters>,
) {
    let spawn_interval = 0.05; // spawn a particle every 0.05 seconds
    *elapsed += time.delta_seconds();

    if *elapsed >= spawn_interval {
        *elapsed = 0.0;

        let radius = 5.0;

        for i in 0..4 {
            // Particle initial setup
            let x: f32 = 0.0;
            let y: f32 = 0.0 - (i as f32) * 10.0;
            let initial_velocity = Vec3::new(300.0, -100.0 - (i as f32) * 10.0, 0.0);
            let position = Vec3::new(x, y, 0.0);
            let position_old = position - initial_velocity * time.delta_seconds();

            commands.spawn((
                Particle {
                    position_old,
                    velocity: initial_velocity, // Correctly set initial velocity
                    acceleration: Vec3::ZERO,
                    radius,
                },
                Transform::from_translation(position),
            ));
        }
    }
}
