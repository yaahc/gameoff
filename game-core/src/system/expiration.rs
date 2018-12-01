use amethyst::{
    core::timing::Time,
    ecs::{Entities, Join, Read, System, WriteStorage},
};
use crate::component::Expiration;

pub struct Expirer;

impl<'s> System<'s> for Expirer {
    type SystemData = (WriteStorage<'s, Expiration>, Entities<'s>, Read<'s, Time>);

    fn run(&mut self, (mut expirations, entities, time): Self::SystemData) {
        for (entity, expiration) in (&entities, &mut expirations).join() {
            if let Some(diff) = expiration.seconds_left.checked_sub(time.delta_time()) {
                expiration.seconds_left = diff;
            } else {
                entities.delete(entity);
            }
        }
    }
}
