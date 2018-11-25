use amethyst::{
    core::cgmath::{InnerSpace, Vector2},
    core::{timing::Time, Transform},
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
};
use crate::component::{Motion, Projectile};

pub struct Movement;

impl<'s> System<'s> for Movement {
    type SystemData = (
        ReadStorage<'s, Projectile>,
        WriteStorage<'s, Motion>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (projectiles, mut motions, mut transforms, time): Self::SystemData) {
        for (_, motion, transform) in (&projectiles, &mut motions, &mut transforms).join() {
            let delta = time.delta_seconds();
            let distance = motion.vel * delta + 0.5 * motion.acc * delta.powf(2.0); // d = v*t + (a*t^2)/2
            if let Some(min_vel) = motion.min_vel {
                if distance.magnitude2() < (min_vel * delta).powf(2.0)
                    || motion.vel.dot(motion.vel + motion.acc * delta) < 0.0
                {
                    distance.normalize_to(min_vel * delta);
                    motion.vel = motion.vel.normalize_to(min_vel);
                    motion.acc = Vector2 { x: 0.0, y: 0.0 };
                    motion.min_vel = None;
                }
            } else if let Some(max_vel) = motion.max_vel {
                if distance.magnitude2() > (max_vel * delta).powf(2.0) {
                    distance.normalize_to(max_vel * delta);
                    motion.vel = motion.vel.normalize_to(max_vel);
                    motion.acc = Vector2 { x: 0.0, y: 0.0 };
                    motion.max_vel = None;
                }
            }
            motion.vel += motion.acc * delta; // vo = vi + a*t
            transform.translation += distance.extend(0.0);
        }
    }
}
