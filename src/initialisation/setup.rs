use crate::initialisation::simulation_parameters::SimulationParameters;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn setup_system(mut commands: Commands, parameters: Res<SimulationParameters>) {
    commands.spawn(Camera2dBundle::default());

    let bounding_circle = shapes::Circle {
        radius: parameters.boundary_radius,
        center: Vec2::ZERO,
    };
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&bounding_circle),
            spatial: SpatialBundle {
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                    ..Default::default()
                },
            ..default()
        },
        Stroke::new(Color::BLACK, 0.0),
    ));
}

