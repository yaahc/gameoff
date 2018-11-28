use amethyst::{
    core::cgmath::{InnerSpace, Vector2},
    core::Transform,
    ecs::{Entities, Join, Read, ReadStorage, System, WriteStorage},
    renderer::{SpriteRender, Transparent},
};
use config::GameoffConfig;
use crate::component::{Ally, Animation, Motion, Player};
use rand::distributions::{Distribution, Uniform};

pub struct Movement;

impl<'s> System<'s> for Movement {
    type SystemData = (
        ReadStorage<'s, Ally>,
        WriteStorage<'s, Motion>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, GameoffConfig>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (allies, mut motions, transforms, players, config, entities): Self::SystemData,
    ) {
        let mut rng = rand::thread_rng();
        let zero_distance_dist = Uniform::new(0.5, 1.0);

        let p_transform = {
            let (t, _) = (&transforms, &players)
                .join()
                .next()
                .expect("no player found");
            (t.clone())
        };

        for (_, motion, transform1, entity1) in
            (&allies, &mut motions, &transforms, &entities).join()
        {
            let d = (p_transform.translation - transform1.translation).truncate();
            let m = d.magnitude().abs();

            let mut pv = d.normalize(); //unit vector
                                        //less than follow distance, do nothing
            if m < config.ally.follow_distance {
                pv *= 0.0;
            } else if (m > config.ally.follow_distance) && (m < config.ally.max_distance) {
                //at or more than the follow distance, play a bit of catchup
                pv *= (1.0
                    + (m - config.ally.follow_distance)
                        / (config.ally.max_distance - config.ally.follow_distance))
                    * config.speed;
            } else if m > config.ally.max_distance {
                //max catchup is 2.0
                pv *= 2.0 * config.speed;
            }

            let mut av = Vector2::new(0.0, 0.0);

            for (_, transform2, entity2) in (&allies, &transforms, &entities).join() {
                if entity1 != entity2 {
                    let d = (transform1.translation - transform2.translation).truncate();
                    let m = d.magnitude().abs();
                    let mut v = if m == 0.0 {
                        //make sure the vector always points somewhere
                        Vector2::new(
                            zero_distance_dist.sample(&mut rng),
                            zero_distance_dist.sample(&mut rng),
                        ).normalize()
                    } else {
                        d.normalize()
                    };

                    if m < config.ally.min_distance {
                        // repell if less than a minimum distance
                        v *= (config.ally.min_distance - m) / config.ally.min_distance
                            * 5.0
                            * config.speed;
                    } else if (m > config.ally.follow_distance) && (m < config.ally.max_distance) {
                        // attract to other allies as well as the player.
                        v *= ((m - config.ally.follow_distance)
                            / (config.ally.max_distance - config.ally.follow_distance))
                            * config.speed;
                    }
                    //The player 2x speed up should dominate if longer than max distance

                    av += v;
                }
            }
            motion.vel = pv + av;
        }
    }
}

pub struct Grouper;

impl<'s> System<'s> for Grouper {
    type SystemData = (
        ReadStorage<'s, Ally>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Motion>,
        ReadStorage<'s, Player>,
        Entities<'s>,
    );

    fn run(&mut self, (allies, transforms, mut motions, players, entities): Self::SystemData) {
        let p_transform = {
            let (t, _) = (&transforms, &players)
                .join()
                .next()
                .expect("no player found");
            (t.clone())
        };

        let mut merged = vec![];
        for (_ally, transform, e, _) in (&allies, &transforms, &*entities, !&motions).join() {
            let d = p_transform.translation - transform.translation;
            let m = d.truncate().magnitude();

            let merge_dist = 32.0 * 1.0;
            if m < merge_dist {
                merged.push(e);
            }
        }

        for entity in merged {
            let _ = motions.insert(entity, Motion::default());
        }
    }
}

pub struct Spawner;

impl<'s> System<'s> for Spawner {
    type SystemData = (
        ReadStorage<'s, Player>,
        ReadStorage<'s, Motion>,
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
            motions,
            textures,
            mut transforms,
            mut allies,
            mut sprites,
            mut transparent,
            entities,
            mut animation,
        ): Self::SystemData,
    ) {
        let count = (&allies, !&motions).join().count();

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
