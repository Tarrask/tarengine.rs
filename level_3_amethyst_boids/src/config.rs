use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BoidConfig {
    pub boid_count: i32,
    pub boid_size: f32,

    pub velocity_min: f32,
    pub velocity_max: f32,

    pub acceleration_max: f32,

    pub attraction_radius: f32,
    pub attraction_factor: f32,
    
    pub repulsion_radius: f32,
    pub repulsion_factor: f32,

    pub alignment_radius: f32,
    pub alignment_factor: f32
}

impl Default for BoidConfig {
    fn default() -> Self {
        BoidConfig {
            boid_count: 10,
            boid_size: 16.0,

            velocity_min: 10.0,
            velocity_max: 200.0,

            acceleration_max: 20000.0,

            attraction_radius: 250.0,
            attraction_factor: 0.0,
            
            repulsion_radius: 300.0,
            repulsion_factor: 0.0,

            alignment_radius: 250.0,
            alignment_factor: 1.0
        }
    }
}