mod ball;
mod paddle;
mod score_board;

use gdnative::prelude::*;

use crate::ball::Ball;
use crate::paddle::Paddle;
use crate::score_board::ScoreBoard;

fn init(handle: InitHandle) {
    handle.add_class::<Ball>();
    handle.add_class::<Paddle>();
    handle.add_class::<ScoreBoard>();
}

godot_init!(init);

