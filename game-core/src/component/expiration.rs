use amethyst::ecs::{Component, DenseVecStorage};
use std::time::Duration;

#[derive(Default, Debug)]
pub struct Expiration {
    pub seconds_left: Duration,
}

impl Component for Expiration {
    type Storage = DenseVecStorage<Self>;
}
