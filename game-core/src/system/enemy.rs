use amethyst::{
    core::Transform,
    ecs::{Entities, Join, Read, ReadStorage, System, WriteStorage},
    renderer::{SpriteRender, Transparent},
};
use crate::component::{Animation, Enemy, Player};
use rand::distributions::{Distribution, Uniform};

pub struct Movement;

impl<'s> System<'s> for Movement {
    type SystemData = (ReadStorage<'s, Enemy>, WriteStorage<'s, Transform>);

    fn run(&mut self, (enemies, mut transforms): Self::SystemData) {
        for (_, _transform) in (&enemies, &mut transforms).join() {}
    }
}

pub struct Spawner;

impl<'s> System<'s> for Spawner {
    type SystemData = (
        ReadStorage<'s, Player>,
        Read<'s, crate::load::LoadedTextures>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Enemy>,
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
            mut enemies,
            mut sprites,
            mut transparent,
            entities,
            mut animation,
        ): Self::SystemData,
    ) {
        let count = (&enemies).join().count();

        if count < 5 {
            let mut enemy_positions = vec![];
            let range = Uniform::new_inclusive(-5.0 * 32.0, 5.0 * 32.0);
            let mut rng = rand::thread_rng();
            for (_, transform) in (&players, &mut transforms).join() {
                let mut pos = Transform::default();
                pos.translation.x = transform.translation.x + range.sample(&mut rng);
                pos.translation.y = transform.translation.y + range.sample(&mut rng);

                enemy_positions.push(pos);
            }

            for pos in enemy_positions {
                let sprite = SpriteRender {
                    sprite_sheet: textures.textures["penguinFront.png"].clone(),
                    sprite_number: 0,
                    flip_horizontal: false,
                    flip_vertical: false,
                };

                let anim = Animation {
                    total_frames: 2,
                    max_count_till_next_frame: 0.7,
                    frame_life_time_count: 0.7,
                    current_frame: 0,
                };

                entities
                    .build_entity()
                    .with(pos, &mut transforms)
                    .with(Enemy::default(), &mut enemies)
                    .with(sprite, &mut sprites)
                    .with(Transparent, &mut transparent)
                    .with(anim, &mut animation)
                    .build();
            }
        }
    }
}
