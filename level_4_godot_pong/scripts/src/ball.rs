use gdnative::prelude::*;

use crate::{
    paddle::{PADDLE_DEMI_HEIGHT, PADDLE_DEMI_WIDTH}, 
    score_board::ScoreBoard
};

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Ball {
    #[property]
    velocity: Vector2,
    #[property(default = 1.0)]
    acceleration: f32,
    player_1: Option<Ref<Node>>,
    player_2: Option<Ref<Node>>,
    score_board: Option<Ref<Node>>
}

const DEFAULT_WINDOW_HEIGHT: f32 = 600.0;
const DEFAULT_WINDOW_WIDTH: f32 = 800.0;
const DEFAULT_VELOCITY: Vector2 = Vector2::new(20.0, -15.0);
const DEFAULT_ACCELERATION: f32 = 1.0;
const BALL_RADIUS: f32 = 10.0;

#[methods]
impl Ball {
    fn new(_owner: &Node2D) -> Self {
        Ball { 
            velocity: DEFAULT_VELOCITY,
            acceleration: DEFAULT_ACCELERATION,
            player_1: None,
            player_2: None,
            score_board: None
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Node2D) {
        // place la balle au milieu de la fenêtre
        let viewport_size = match owner.get_viewport() {
            Some(viewport) => unsafe { viewport.assume_unique().size() },
            None => Vector2::new(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT)
        };
        owner.set_global_position(viewport_size / 2.0);

        self.player_1 = owner.get_node(NodePath::from_str("../Player_1"));
        self.player_2 = owner.get_node(NodePath::from_str("../Player_2"));
        self.score_board = owner.get_node(NodePath::from_str("/root/Level/ScoreBoard"));
        godot_print!("{:?}", self.score_board);

        self.velocity = owner.get_meta("starting_velocity").to_vector2();

        godot_print!("Ball ready!");
    }

    #[export]
    fn _process(&mut self, owner: &Node2D, dt: f32) {
        let viewport_size = match owner.get_viewport() {
            Some(viewport) => unsafe { viewport.assume_safe().size() },
            None => Vector2::new(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT)
        };

        // mouvement naturel de la balle
        let mut position = owner.global_position();
        position += self.velocity * dt;

        if position.y > viewport_size.y - BALL_RADIUS {
            self.velocity.y = -self.velocity.y;
            position.y += viewport_size.y - BALL_RADIUS - position.y 
        }

        if position.y < BALL_RADIUS {
            self.velocity.y = -self.velocity.y;
            position.y += BALL_RADIUS - position.y
        }

        // balle qui sort à droite ou à gauche
        if position.x > viewport_size.x + BALL_RADIUS {
            self.velocity = owner.get_meta("starting_velocity").to_vector2();
            self.velocity.x *= -1.0;
            position = viewport_size / 2.0;

            unsafe { 
                // let score_board = self.score_board.expect("bla bal");
                // let score_board = score_board.assume_safe().cast_instance::<ScoreBoard>().expect("bla bli");
                // score_board.map_mut(|x, o| x.score(&*o, 1)).unwrap();
                let score_board = owner.get_node_as_instance::<ScoreBoard>("/root/Level/ScoreBoard");
                godot_print!("scoreboard: {:?}", score_board);
                // owner.get_node_as_instance::<ScoreBoard>("/root/Level/ScoreBoard").expect("Can not get the scoreboard")
                //     .map_mut(|x, o| x.score(&*o, 1))
                //     .ok()
                //     .unwrap_or_else(|| godot_error!("Unable to get scoreBoard"));
            };
        } 
        if position.x < -BALL_RADIUS {
            self.velocity = owner.get_meta("starting_velocity").to_vector2();
            position = viewport_size / 2.0;

            // unsafe { 
            //     owner.get_node_as_instance::<ScoreBoard>("ScoreBoard").unwrap()
            //         .map_mut(|x, o| x.score(&*o, 2))
            //         .ok()
            //         .unwrap_or_else(|| godot_error!("Unable to get scoreBoard"));
            // };
        }

        // rebond sur les palettes
        let players = [self.player_1, self.player_2];
        for player in &players {
            if let Some(node) = player {
                match unsafe { node.assume_safe().cast::<Node2D>() } {
                    Some(player) => {
                        let paddle_position = player.global_position();
                        if point_in_rect(
                            position.x, position.y,
                            paddle_position.x - PADDLE_DEMI_WIDTH - BALL_RADIUS,
                            paddle_position.y - PADDLE_DEMI_HEIGHT - BALL_RADIUS,
                            paddle_position.x + PADDLE_DEMI_WIDTH + BALL_RADIUS,
                            paddle_position.y + PADDLE_DEMI_HEIGHT + BALL_RADIUS
                        ) {
                            if self.velocity.x < 0.0 {
                                position.x = position.x + (paddle_position.x + PADDLE_DEMI_WIDTH + BALL_RADIUS - position.x) * 2.0;
                            } 
                            else {
                                position.x = position.x + (paddle_position.x - PADDLE_DEMI_WIDTH - BALL_RADIUS - position.x) * 2.0;
                            }
                            self.velocity.x = -self.velocity.x;
                            
                            self.velocity *= self.acceleration;
                        }
                    }
                    None => {
                        godot_error!("Can not have a reference to the paddle");
                    }
                }
            }
        }

        

        
        owner.set_global_position(position);
        // godot_print!("{:?}", owner.scale());
    }
}

fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}