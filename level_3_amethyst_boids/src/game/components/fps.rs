use amethyst::ecs::{Component, DenseVecStorage, Entity};

pub struct FpsText {
    pub entity: Entity
}

impl Component for FpsText {
    type Storage = DenseVecStorage<Self>;
}