use crate::components::particle::Particle;
use crate::simulation_parameters::SimulationParameters;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::Rng;

pub fn setup_system(mut commands: Commands, parameters: Res<SimulationParameters>) {
    commands.spawn(Camera2dBundle::default());

    let bounding_circle = shapes::Circle {
        radius: parameters.boundary_radius,
        center: Vec2::ZERO,
    };
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&bounding_circle),
            ..default()
        },
        Stroke::new(Color::BLACK, 4.0),
    ));

    let radius = 15.0;

    let particle_shape = shapes::Circle {
        radius,
        center: Vec2::ZERO,
    };

    let mut rng = rand::thread_rng();
    let boundary_radius = parameters.boundary_radius;

    (0..20).for_each(|_| {
        // Generate a random position within the boundary radius
        let angle = rng.gen_range(0.0..2.0 * std::f32::consts::PI);
        let distance = rng.gen_range(0.0..boundary_radius - radius);
        let x = distance * angle.cos();
        let y = distance * angle.sin();

        commands.spawn((
            Particle {
                radius,
                velocity: Vec3::ZERO,
                density: 0.0,
            },
            ShapeBundle {
                path: GeometryBuilder::build_as(&particle_shape),
                spatial: SpatialBundle {
                    transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
            Fill::color(Color::WHITE),
            Stroke::new(Color::BLACK, 1.0),
        ));
    });
}

