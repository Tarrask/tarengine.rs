
use gdnative::prelude::*;

#[derive(NativeClass, Debug)]
#[user_data(user_data::MutexData<ScoreBoard>)]
#[inherit(Node)]
pub struct ScoreBoard {
    player_1_score: i32,
    player_2_score: i32
}

#[methods]
impl ScoreBoard {
    fn new(_owner: &Node) -> Self {
        ScoreBoard { player_1_score: 0, player_2_score: 0 }
    }

    #[export]
    pub fn score(&mut self, owner: &Node, player: i32) {
        if player == 1 {
            self.player_1_score += 1;
            let text_node = owner.get_node(NodePath::from_str("../h/v_1/PlayerScore_1")).unwrap();
            unsafe { 
                let label = text_node.assume_safe().cast::<Label>().unwrap(); 
                label.set_text(format!("{}", self.player_1_score));
            }
        }
        else {
            self.player_2_score += 1;
            let text_node = owner.get_node(NodePath::from_str("../h/v_2/PlayerScore_2")).unwrap();
            unsafe { 
                let label = text_node.assume_safe().cast::<Label>().unwrap(); 
                label.set_text(format!("{}", self.player_2_score));
            }
        }
    }
}