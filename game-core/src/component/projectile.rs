use amethyst::ecs::{Component, NullStorage};

pub struct Projectile;

impl Default for Projectile {
    fn default() -> Self {
        Projectile
    }
}

impl Component for Projectile {
    type Storage = NullStorage<Self>;
}
