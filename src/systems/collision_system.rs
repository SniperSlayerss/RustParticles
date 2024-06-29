use crate::components::particle::{Particle, Velocity};
use bevy::prelude::*;

pub fn collision_system(
    mut query: Query<(&mut Transform, &mut Velocity, &Particle), With<Particle>>,
) {
    let mut particles: Vec<_> = query.iter_mut().collect();

    for i in 0..particles.len() {
        let (first, second) = particles.split_at_mut(i + 1);
        let (t_1, v_1, p_1) = &mut first[i];

        for j in 0..second.len() {
            let (t_2, v_2, p_2) = &mut second[j];

            let distance = t_1.translation.distance(t_2.translation);
            if distance > p_1.radius + p_2.radius {
                continue;
            }

            let normal = (t_1.translation - t_2.translation).normalize();
            let relative_v = v_1.0 - v_2.0;
            let normal_v = relative_v.dot(normal);

            // Particles moving apart do nothing
            if normal_v > 0.0 {
                continue;
            }

            let restitution = 0.7;
            let impluse_mag = -(1.0 + restitution) * normal_v / 2.0;

            let impulse = impluse_mag * normal;
            v_1.0 += impulse;
            v_2.0 -= impulse;

            let penetration_depth =
                p_1.radius + p_2.radius - t_1.translation.distance(t_2.translation);
            let correction = (penetration_depth / 2.0) * normal;
            t_1.translation += correction;
            t_2.translation -= correction;
        }
    }
}
