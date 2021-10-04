
use amethyst::{
    core::{
        timing::Time,
        math::{Vector2, Vector3}, 
        transform::Transform
    }, 
    derive::SystemDesc, 
    ecs::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage}
};

use crate::{config::BoidConfig, game::components::{Boid, Physics}};

#[derive(SystemDesc)]
pub struct BoidSystem;

impl<'s> System<'s> for BoidSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Boid>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Physics>,
        Read<'s, Time>,
        Read<'s, BoidConfig>
        
    );

    fn run(&mut self, (entities, mut boids, transforms, physics, _time, config): Self::SystemData) {
        for (entity_a, physics_a, boid_a, transform_a) in (&entities, &physics, &mut boids, &transforms).join() {
            let mut attraction_position: Vector3<f32> = Vector3::zeros();
            let mut attraction_count: f32 = 0.0;
            let mut repulsion_force: Vector3<f32> = Vector3::zeros();
            let mut alignment_direction: Vector2<f32> = Vector2::zeros();
            let mut alignment_count: f32 = 0.0;
            
            for (entity_b, physics_b, transform_b) in (&entities, &physics, &transforms).join() {
                // avoid self
                if entity_a == entity_b { continue; }

                let distance: Vector3<f32> = transform_a.translation() - transform_b.translation();
                let square_distance = distance.magnitude_squared();

                // attraction
                if square_distance < config.attraction_radius * config.attraction_radius {
                    attraction_position += transform_b.translation();
                    attraction_count += 1.0;
                    // println!("magnitude between {:?} and {:?} = {}", entity_a, entity_b, square_distance);
                }

                // repulsion
                if square_distance < config.repulsion_radius * config.repulsion_radius {
                    // println!("distance={}, normedDistance={}", distance, distance / config.repulsion_radius);
                    repulsion_force += (distance / square_distance.sqrt()) * config.repulsion_factor;
                    // repulsion_count += 1.0;
                }

                // alignment
                if square_distance < config.alignment_radius * config.alignment_radius {
                    alignment_direction += physics_b.velocity;
                    alignment_count += 1.0;
                }
            }

            // additionne les facteurs de steering
            if attraction_count > 0.0 { 
                boid_a.attraction.x = (attraction_position.x / attraction_count - transform_a.translation().x) * config.attraction_factor;
                boid_a.attraction.y = (attraction_position.y / attraction_count - transform_a.translation().y) * config.attraction_factor;
            }

            // if repulsion_count > 0.0 {
            //     let repulsion_vect: Vector3<f32> = transform_a.translation() - repulsion_position / repulsion_count;
            //     let repulsion_value = repulsion_vect.magnitude_squared();
                boid_a.repulsion.x = repulsion_force.x; // (repulsion_vect.x / repulsion_value) * config.repulsion_factor;
                boid_a.repulsion.y = repulsion_force.y; // (repulsion_vect.y / repulsion_value) * config.repulsion_factor;
            // }

            if alignment_count > 0.0 {
                let avg: Vector2<f32> = alignment_direction / alignment_count;
                boid_a.alignment = (avg - physics_a.velocity) * config.alignment_factor;
            }
        }
    }
}
