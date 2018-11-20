use amethyst::core::cgmath::InnerSpace;
use amethyst::core::cgmath::Vector2;
use amethyst::renderer::Camera;
use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};
use crate::component::Player;

pub struct Movement;

impl<'s> System<'s> for Movement {
    type SystemData = (
        ReadStorage<'s, Player>,
        ReadStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (players, cameras, mut transforms): Self::SystemData) {
        let mut player_translation = Vector2 { x: 0.0, y: 0.0 };

        for (_, transform) in (&players, &mut transforms).join() {
            player_translation = transform.translation.truncate();
        }

        for (_, transform) in (&cameras, &mut transforms).join() {
            let camera_translation = transform.translation.truncate();
            let camera_scale = transform.scale.truncate();
            let player_direction = player_translation - camera_translation - camera_scale / 2.0;
            let camera_safe_edge = camera_scale / 4.0;

            if player_direction.magnitude2() > camera_safe_edge.magnitude2() {
                let camera_shift =
                    player_direction - player_direction.normalize_to(camera_safe_edge.magnitude());
                transform.translation += camera_shift.extend(0.0);
            }
        }
    }
}
