mod boid;
mod fps;

use amethyst::{core::math::Vector2, ecs::{Component, DenseVecStorage}};

pub use self::boid::Boid;
pub use self::fps::FpsText;

pub struct Physics {
    pub velocity: Vector2<f32>,
    pub acceleration: Vector2<f32>
}

impl Component for Physics {
    type Storage = DenseVecStorage<Self>;
}