
#[allow(unused)]
use log::{debug, info, warn, error};

use amethyst::{core::timing::Time, core::transform::Transform, derive::SystemDesc, ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage}, shred::ReadExpect, window::ScreenDimensions};

use crate::{config::BoidConfig, game::{components::{Boid, Physics}}};

#[derive(SystemDesc)]
pub struct MoveSystem;

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Physics>,
        ReadStorage<'s, Boid>,
        ReadExpect<'s, ScreenDimensions>,
        Read<'s, Time>,
        Read<'s, BoidConfig>
    );

    fn run(&mut self, (mut transforms, mut physics, boids, screen_dimensions, time, config): Self::SystemData) {
        let width = screen_dimensions.width() + 40.0;
        let height = screen_dimensions.height() + 40.0;

        for (transform, physic, boid) in (&mut transforms, &mut physics, &boids).join() {
            // calculate the new acceleration
            physic.acceleration = boid.alignment + boid.attraction + boid.repulsion;
            let a = physic.acceleration.magnitude();
            if a > config.acceleration_max {
                physic.acceleration = physic.acceleration / a * config.acceleration_max;
            }

            // calculate new velocity with applied acceleration
            physic.velocity += physic.acceleration * time.delta_seconds();

            // avoid over speed and under speed
            let v = physic.velocity.magnitude();
            if v < config.velocity_min {
                physic.velocity = physic.velocity / v * config.velocity_min;
            }
            if v > config.velocity_max {
                physic.velocity = physic.velocity / v * config.velocity_max;
            }

            // move entity based on velocity
            transform.prepend_translation_x(physic.velocity.x * time.delta_seconds());
            transform.prepend_translation_y(physic.velocity.y * time.delta_seconds());

            debug!("translation: {} {}", transform.translation().x, transform.translation().y);

            // wrap entity arround screen
            if transform.translation().x < -width * 0.5 {
                transform.prepend_translation_x(width);
            }
            else if transform.translation().x > width * 0.5 {
                transform.prepend_translation_x(-width);
            }

            if transform.translation().y < -height * 0.5 {
                transform.prepend_translation_y(height);
            }
            else if transform.translation().y > height * 0.5 {
                transform.prepend_translation_y(-height);
            }

            transform.set_rotation_z_axis(f32::atan2(physic.velocity.y, physic.velocity.x));
        }
    }
}