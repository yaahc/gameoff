use amethyst::ecs::{Component, DenseVecStorage};

pub struct Ally {
    pub hp: u32,
}

impl Default for Ally {
    fn default() -> Self {
        Self { hp: 10 }
    }
}

impl Component for Ally {
    type Storage = DenseVecStorage<Self>;
}
