use amethyst::{
    core::cgmath::Vector2,
    core::Transform,
    ecs::{Entities, Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
    renderer::{SpriteRender, Transparent},
};
use crate::component::{Animation, Enemy, Motion, Player, Projectile};

pub struct Movement;

impl<'s> System<'s> for Movement {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<String, String>>,
        Option<Read<'s, crate::map::PassableTiles>>,
    );

    fn run(&mut self, (players, mut transforms, input, passable): Self::SystemData) {
        if let Some(passable) = passable {
            let x_move = input.axis_value("entity_x").unwrap();
            let y_move = input.axis_value("entity_y").unwrap();

            for (_, transform) in (&players, &mut transforms).join() {
                let goal_x = transform.translation.x + x_move as f32 * 5.0;
                let goal_y = transform.translation.y + y_move as f32 * 5.0;

                let tile_y = (goal_y as u32 / 32) as usize;
                let tile_x = (goal_x as u32 / 32) as usize;

                if passable.tile_matrix[tile_y][tile_x] {
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
            entities,
            input,
        ): Self::SystemData,
    ) {
        let mut bubble_transform = None;
        for (_, p_transform) in (&players, &transforms).join() {
            for (enemy, e_transform, enemy_entity) in (&mut enemies, &transforms, &*entities).join()
            {
                if input.action_is_down("jump") == Some(true) {
                    bubble_transform = Some(p_transform.clone());
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
                vel: Vector2 { x: 1.0, y: 1.0 },
                acc: Vector2 { x: 1.0, y: 1.0 },
                min_vel: None,
                max_vel: None,
            };

            entities
                .build_entity()
                .with(transform, &mut transforms)
                .with(Projectile, &mut projectiles)
                .with(motion, &mut motions)
                .with(sprite, &mut sprites)
                .with(Transparent, &mut transparent)
                .with(anim, &mut animations)
                .build();
        }
    }
}
