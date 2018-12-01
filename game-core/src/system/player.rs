use amethyst::{
    core::cgmath::{InnerSpace, Vector2},
    core::Transform,
    ecs::{Entities, Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
    renderer::{SpriteRender, Transparent},
};
use crate::component::{Animation, Enemy, Expiration, Motion, Player, Projectile};
use rand::distributions::{Distribution, Uniform};
use std::time::Duration;

pub struct Movement;

impl<'s> System<'s> for Movement {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<String, String>>,
        Option<Read<'s, crate::map::PassableTiles>>,
    );

    fn run(&mut self, (mut players, mut transforms, input, passable): Self::SystemData) {
        if let Some(passable) = passable {
            let x_move = input.axis_value("entity_x").unwrap();
            let y_move = input.axis_value("entity_y").unwrap();

            for (player, transform) in (&mut players, &mut transforms).join() {
                if x_move != 0.0 || y_move != 0.0 {
                    player.last_direction = Vector2 {
                        x: x_move as f32,
                        y: y_move as f32,
                    };
                }

                let goal_x = transform.translation.x + x_move as f32 * 5.0;
                let goal_y = transform.translation.y + y_move as f32 * 5.0;

                let tile_y = (goal_y as u32 / 32) as usize;
                let tile_x = (goal_x as u32 / 32) as usize;

                if *passable
                    .tile_matrix
                    .get(tile_y)
                    .and_then(|row| row.get(tile_x))
                    .unwrap_or(&false)
                {
                    transform.translation.x = goal_x;
                    transform.translation.y = goal_y;
                }
            }
        }
    }
}

pub struct Attack;

impl<'s> System<'s> for Attack {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, Transform>,
        Read<'s, crate::load::LoadedTextures>,
        WriteStorage<'s, Projectile>,
        WriteStorage<'s, Motion>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Transparent>,
        WriteStorage<'s, Animation>,
        WriteStorage<'s, Expiration>,
        Entities<'s>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(
        &mut self,
        (
            players,
            mut enemies,
            mut transforms,
            textures,
            mut projectiles,
            mut motions,
            mut sprites,
            mut transparent,
            mut animations,
            mut expirations,
            entities,
            input,
        ): Self::SystemData,
    ) {
        let mut bubble_transform = None;
        let mut bubble_dir = None;
        for (player, p_transform) in (&players, &transforms).join() {
            for (enemy, e_transform, enemy_entity) in (&mut enemies, &transforms, &*entities).join()
            {
                if input.action_is_down("jump") == Some(true) {
                    bubble_transform = Some(p_transform.clone());

                    let range = Uniform::new_inclusive(-5.0 * 32.0, 5.0 * 32.0);
                    let mut rng = rand::thread_rng();
                    let perp = Vector2 {
                        x: player.last_direction.y,
                        y: -player.last_direction.x,
                    };
                    let perp = perp.normalize_to(range.sample(&mut rng));

                    bubble_dir = Some(player.last_direction.normalize_to(32.0 * 23.0) + perp);
                }

                if e_transform.translation.x < p_transform.translation.x
                    && e_transform.translation.y < p_transform.translation.y
                {
                    if enemy.hp > 0 {
                        enemy.hp -= 1;
                    } else {
                        let _r = entities.delete(enemy_entity);
                    }
                }
            }
        }

        if let Some(transform) = bubble_transform {
            let sprite = SpriteRender {
                sprite_sheet: textures.textures["bubble.png"].clone(),
                sprite_number: 0,
                flip_horizontal: false,
                flip_vertical: false,
            };

            let anim = Animation {
                total_frames: 2,
                max_count_till_next_frame: 0.5,
                frame_life_time_count: 0.5,
                current_frame: 0,
            };

            let motion = Motion {
                vel: bubble_dir.unwrap(),
                acc: bubble_dir.unwrap() * -2.0,
                min_vel: Some(32.0),
                max_vel: None,
            };

            let expiration = Expiration {
                seconds_left: Duration::from_secs(1),
            };

            entities
                .build_entity()
                .with(transform, &mut transforms)
                .with(Projectile, &mut projectiles)
                .with(motion, &mut motions)
                .with(sprite, &mut sprites)
                .with(Transparent, &mut transparent)
                .with(anim, &mut animations)
                .with(expiration, &mut expirations)
                .build();
        }
    }
}
