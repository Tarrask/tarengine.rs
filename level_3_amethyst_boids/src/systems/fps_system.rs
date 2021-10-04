use amethyst::{
    ecs::{Read, System, WriteStorage}, 
    shred::ReadExpect, 
    ui::UiText, 
    utils::fps_counter::FpsCounter
};

use crate::game::components::FpsText;

pub struct FpsSystem;

impl<'s> System<'s> for FpsSystem {
    type SystemData = (
        WriteStorage<'s, UiText>,
        ReadExpect<'s, FpsText>,
        Read<'s, FpsCounter>
    );

    fn run(&mut self, (mut ui_text, fps_text, fps_counter): Self::SystemData) {
        if let Some(text) = ui_text.get_mut(fps_text.entity) {
            text.text = format!("{:3.0}", fps_counter.sampled_fps());
        }
    }
}