use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Default, Debug)]
pub struct Expiration {
    pub seconds_left: f32,
}

impl Component for Expiration {
    type Storage = DenseVecStorage<Self>;
}
