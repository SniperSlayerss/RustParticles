use crate::components::particle::Particle;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let bounding_cirlce = shapes::Circle {
        radius: 500.0,
        center: Vec2::ZERO,
    };
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&bounding_cirlce),
            ..default()
        },
        Stroke::new(Color::BLACK, 4.0),
    ));

    let radius = 15.0;

    let particle = shapes::Circle {
        radius,
        center: Vec2::ZERO,
    };

    (0..10).for_each(|i| {
        commands.spawn((
            Particle {
                radius,
                velocity: Vec3::ZERO,
                density: 0.0,
            },
            ShapeBundle {
                path: GeometryBuilder::build_as(&particle),
                spatial: SpatialBundle {
                    transform: Transform::from_translation(Vec3::new(
                        ((i - 5) * 100) as f32,
                        //-i as f32,
                        (-i - 100) as f32,
                        0.0,
                    )),
                    ..Default::default()
                },
                ..Default::default()
            },
            Fill::color(Color::WHITE),
            Stroke::new(Color::BLACK, 1.0),
        ));
    });
}
