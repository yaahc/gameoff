use amethyst::core::cgmath::InnerSpace;
use amethyst::core::cgmath::Rad;
use amethyst::{
    core::Transform,
    ecs::{Entities, Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
};
use crate::component::Enemy;
use crate::component::Player;

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

                    //                     transform.rotation.v.x = x_move as f32;
                    //                     transform.rotation.v.y = y_move as f32;
                }
            }
        }
    }
}

pub struct Attack;

const ATTACK_RANGE: f32 = 2.0 * 32.0;

impl<'s> System<'s> for Attack {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Enemy>,
        ReadStorage<'s, Transform>,
        Entities<'s>,
    );

    fn run(&mut self, (players, mut enemies, transforms, entities): Self::SystemData) {
        for (_, p_transform) in (&players, &transforms).join() {
            for (enemy, e_transform, enemy_entity) in (&mut enemies, &transforms, &*entities).join()
            {
                let to_enemy =
                    e_transform.translation.truncate() - p_transform.translation.truncate();

                if to_enemy.magnitude2() < ATTACK_RANGE.powf(4.0) {
                    if enemy.hp > 0 {
                        enemy.hp -= 1;
                    } else {
                        println!(
                            "{:?}, {:?}, {:?}",
                            to_enemy.angle(p_transform.rotation.v.truncate()),
                            to_enemy,
                            p_transform.rotation.v,
                        );
                        let _r = entities.delete(enemy_entity);
                    }
                }
            }
        }
    }
}
