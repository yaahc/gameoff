use amethyst::{
    core::timing::Time,
    ecs::{Entities, Join, Read, System, WriteStorage},
    renderer::SpriteRender,
};
use crate::component::Animation;

pub struct Frame;

impl<'s> System<'s> for Frame {
    type SystemData = (
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Animation>,
        Entities<'s>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut sprite_render, mut animation, entities, time): Self::SystemData) {
        for (_, animation, sprite_render) in (&entities, &mut animation, &mut sprite_render).join()
        {
            animation.frame_update(sprite_render, time.delta_seconds());
        }
    }
}
