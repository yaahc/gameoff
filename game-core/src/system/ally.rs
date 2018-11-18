use amethyst::{
    core::cgmath::InnerSpace,
    core::{Parent, Transform},
    ecs::{Entities, Join, Read, ReadStorage, System, WriteStorage},
    renderer::{SpriteRender, Transparent},
};
use crate::component::{Ally, Animation, Player};
use rand::distributions::{Distribution, Uniform};

pub struct Movement;

impl<'s> System<'s> for Movement {
    type SystemData = (ReadStorage<'s, Ally>, WriteStorage<'s, Transform>);

    fn run(&mut self, (allies, mut transforms): Self::SystemData) {
        for (_, _transform) in (&allies, &mut transforms).join() {}
    }
}

pub struct Grouper;

impl<'s> System<'s> for Grouper {
    type SystemData = (
        ReadStorage<'s, Ally>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Parent>,
        WriteStorage<'s, Player>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (allies, mut transforms, mut parents, mut players, entities): Self::SystemData,
    ) {
        let get_relative_position = |mut num_allies: u32| {
            num_allies %= 9;

            let ind = if num_allies == 4 {
                num_allies + 1 // skip player position
            } else {
                num_allies
            } as i32;

            let row = ind / 3 - 1;
            let col = ind % 3 - 1;

            (row as f32 * 32.0, col as f32 * 32.0)
        };

        let (p_transform, allies_count, p_entity) = {
            let (t, p, e) = (&transforms, &players, &entities)
                .join()
                .next()
                .expect("no player found");
            (t.clone(), p.num_allies, e)
        };

        let mut orphans = vec![];

        for (_, _, transform, entity) in (&allies, !&parents, &mut transforms, &entities).join() {
            let d = p_transform.translation - transform.translation;
            let merge_dist = f32::powf(32.0 * 1.0, 2.0);
            if d.truncate().magnitude2() < merge_dist {
                let (new_x, new_y) = get_relative_position(allies_count);

                transform.translation.x = new_x;
                transform.translation.y = new_y;

                orphans.push(entity);
            }
        }

        players.get_mut(p_entity).unwrap().num_allies += orphans.len() as u32;

        for entity in orphans {
            parents
                .insert(entity, Parent { entity: p_entity })
                .expect("the unexpected");
        }
    }
}

pub struct Spawner;

impl<'s> System<'s> for Spawner {
    type SystemData = (
        ReadStorage<'s, Player>,
        ReadStorage<'s, Parent>,
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
            parents,
            textures,
            mut transforms,
            mut allies,
            mut sprites,
            mut transparent,
            entities,
            mut animation,
        ): Self::SystemData,
    ) {
        let count = (&allies, !&parents).join().count();

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
