use amethyst::{
    core::cgmath::Vector2,
    ecs::{Component, DenseVecStorage},
};

pub struct Motion {
    pub vel: Vector2<f32>,
    pub acc: Vector2<f32>,
    pub max_vel: Option<f32>,
    pub min_vel: Option<f32>,
}

impl Default for Motion {
    fn default() -> Self {
        Self {
            vel: Vector2 { x: 0.0, y: 0.0 },
            acc: Vector2 { x: 0.0, y: 0.0 },
            max_vel: None,
            min_vel: None,
        }
    }
}

impl Component for Motion {
    type Storage = DenseVecStorage<Self>;
}
