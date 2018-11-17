use amethyst::{
    core::Transform,
    ecs::{Entities, Join, Read, ReadStorage, System, WriteStorage},
    renderer::{SpriteRender, Transparent},
};
use crate::component::Ally;
use crate::component::Animation;
use crate::component::Player;
use rand::distributions::{Distribution, Uniform};

pub struct Movement;

impl<'s> System<'s> for Movement {
    type SystemData = (ReadStorage<'s, Ally>, WriteStorage<'s, Transform>);

    fn run(&mut self, (players, mut transforms): Self::SystemData) {
        for (_, transform) in (&players, &mut transforms).join() {
            println!("ally: {:?}", transform);
        }
    }
}

pub struct Spawner;

impl<'s> System<'s> for Spawner {
    type SystemData = (
        ReadStorage<'s, Player>,
        Read<'s, crate::load::LoadedTextures>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Ally>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Transparent>,
        Entities<'s>,
        WriteStorage<'s, Animation>,
    );

    fn run(
        &mut self,
        (
            players,
            textures,
            mut transforms,
            mut allies,
            mut sprites,
            mut transparent,
            entities,
            mut animation,
        ): Self::SystemData,
    ) {
        let count = (&allies).join().count();

        if count < 5 {
            let mut ally_positions = vec![];
            let range = Uniform::new_inclusive(-5.0 * 32.0, 5.0 * 32.0);
            let mut rng = rand::thread_rng();
            for (_, transform) in (&players, &mut transforms).join() {
                let mut pos = Transform::default();
                pos.scale.x = 0.5;
                pos.scale.y = 0.5;
                pos.translation.x = transform.translation.x + range.sample(&mut rng);
                pos.translation.y = transform.translation.y + range.sample(&mut rng);

                ally_positions.push(pos);
            }

            for pos in ally_positions {
                let sprite = SpriteRender {
                    sprite_sheet: textures.textures["FRONT.png"].clone(),
                    sprite_number: 1,
                    flip_horizontal: false,
                    flip_vertical: false,
                };

                let anim = Animation {
                    total_frames: 8,
                    max_count_till_next_frame: 0.1,
                    frame_life_time_count: 0.1,
                    current_frame: 0,
                };

                entities
                    .build_entity()
                    .with(pos, &mut transforms)
                    .with(Ally::default(), &mut allies)
                    .with(sprite, &mut sprites)
                    .with(Transparent, &mut transparent)
                    .with(anim, &mut animation)
                    .build();
            }
        }
    }
}
