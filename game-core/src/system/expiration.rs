use amethyst::{
    core::timing::Time,
    ecs::{Entities, Join, Read, System, WriteStorage},
    renderer::SpriteRender,
};
use crate::component::Expiration;

pub struct Expirer;

impl<'s> System<'s> for Expirer {
    type SystemData = (WriteStorage<'s, Expiration>, Entities<'s>, Read<'s, Time>);

    fn run(&mut self, (mut expirations, entities, time): Self::SystemData) {
        for (entity, expiration) in (&entities, &mut expirations).join() {
            expiration.seconds_left -= time.delta_seconds();
            if expiration.seconds_left <= 0.0 {
                entities.delete(entity);
            }
        }
    }
}
