use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::component::Enemy;

pub struct Movement;

impl<'s> System<'s> for Movement {
    type SystemData = (ReadStorage<'s, Enemy>, WriteStorage<'s, Transform>);

    fn run(&mut self, (players, mut transforms): Self::SystemData) {
        for (_, transform) in (&players, &mut transforms).join() {}
    }
}
