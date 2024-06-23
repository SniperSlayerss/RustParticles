use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        .add_systems(Startup, setup_system)
        .add_systems(Update, update_system)
        .run();
}

#[derive(Component)]
struct Particle;

#[derive(Component)]
struct Velocity(Vec3);

fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    let particle = shapes::Circle {
        radius: 10.0,
        center: Vec2::ZERO,
    };

    (0..10).for_each(|i| {
        commands.spawn((
            Particle,
            ShapeBundle {
                path: GeometryBuilder::build_as(&particle),
                spatial: SpatialBundle {
                    transform: Transform::from_translation(Vec3::new(
                        //((i as f32) - 50.0) * 4.0,
                        0.0,
                        -i as f32,
                        0.0,
                    )),
                    ..Default::default()
                },
                ..Default::default()
            },
            Fill::color(Color::WHITE),
            Stroke::new(Color::BLACK, 1.0),
            Velocity(Vec3::new(0.0, 0.0, 0.0)),
        ));
    });
}

fn update_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Particle>>,
) {
    let delta_time = time.delta_seconds();
    let gravity = Vec3::new(200.4, -500.0, 0.0);
    let boundary_radius = 100.0;

    for (mut transform, mut velocity) in query.iter_mut() {
        velocity.0 += gravity * delta_time;

        transform.translation += velocity.0 * delta_time;

        check_and_place_within_bounds(&mut transform, &mut velocity, boundary_radius)
    }
}


fn check_and_place_within_bounds(transform: &mut Transform, velocity: &mut Velocity, boundary_radius: f32) {
    if transform.translation.length() > boundary_radius {
        let direction_to_origin = -transform.translation;
        let normalized_direction = direction_to_origin.normalize();

        transform.translation = -normalized_direction * boundary_radius;
        velocity.0 = velocity.0 * -0.98;

    }
}
