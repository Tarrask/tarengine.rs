#[allow(unused)]
use log::{debug, info, warn, error};

use amethyst::{
    derive::SystemDesc, 
    ecs::{Join, System, SystemData, WriteStorage}, 
    renderer::Camera, 
    shred::ReadExpect, 
    window::ScreenDimensions
};

#[derive(SystemDesc)]
pub struct CameraSystem;


impl<'s> System<'s> for CameraSystem {
    type SystemData = (
        WriteStorage<'s, Camera>,
        ReadExpect<'s, ScreenDimensions>
    );

    fn run(&mut self, (mut cameras, screen_dimensions): Self::SystemData) {
        for camera in (&mut cameras).join() {
            *camera = Camera::standard_2d(screen_dimensions.width(), screen_dimensions.height());
        }
    }
}