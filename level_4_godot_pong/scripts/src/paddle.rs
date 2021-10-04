use gdnative::prelude::*;

const DEFAULT_WINDOW_HEIGHT: f32 = 600.0;
const DEFAULT_WINDOW_WIDTH: f32 = 800.0;
const PADDLE_VELOCITY: f32 = 150.0;
pub const PADDLE_DEMI_HEIGHT: f32 = 40.0;
pub const PADDLE_DEMI_WIDTH: f32 = 10.0;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Paddle;


#[methods]
impl Paddle {
    fn new(_owner: &Node2D) -> Self {
        Paddle
    }

    #[export]
    fn _ready(&self, _owner: &Node2D) {
    }

    #[export]
    fn _process(&mut self, owner: &Node2D, dt: f32) {
        let viewport_size = match owner.get_viewport() {
            Some(viewport) => unsafe { viewport.assume_unique().get_size_override() },
            None => Vector2::new(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT)
        };
        let player = owner.get_meta("player").to_i64();
        let mut position = owner.global_position();

        if Input::godot_singleton().is_action_pressed(format!("player_{}_up", player)) {
            position.y -= PADDLE_VELOCITY * dt;
        }
        if Input::godot_singleton().is_action_pressed(format!("player_{}_down", player)) {
            position.y += PADDLE_VELOCITY * dt;
        }

        if position.y < PADDLE_DEMI_HEIGHT {
            position.y = PADDLE_DEMI_HEIGHT;
        }
        if position.y > viewport_size.y - PADDLE_DEMI_HEIGHT {
            position.y = viewport_size.y - PADDLE_DEMI_HEIGHT;
        }
        
        owner.set_global_position(position);
    }
}
