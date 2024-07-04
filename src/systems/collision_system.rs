use bevy::prelude::*;

pub fn check_collisions(
    particles: &mut Vec<(Vec3, Vec3, f32)>,
    substeps: i32,
    restitution: f32,
) {
    for _ in 0..substeps {
        resolve_collisions(particles, restitution);
    }
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
