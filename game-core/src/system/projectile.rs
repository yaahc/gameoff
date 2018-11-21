use amethyst::{
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
            motion.vel += motion.acc * delta; // vo = vi + a*t
            transform.translation += distance.extend(0.0);
        }
    }
}
