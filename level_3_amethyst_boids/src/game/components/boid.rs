use amethyst::{core::math::Vector2, ecs::{Component, DenseVecStorage}};

pub struct Boid {
    pub alignment: Vector2<f32>,
    pub attraction: Vector2<f32>,
    pub repulsion: Vector2<f32>
}

impl Component for Boid {
    type Storage = DenseVecStorage<Self>;
}
