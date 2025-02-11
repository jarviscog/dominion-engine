
use crate::pile::Pile;

#[derive(Debug)]
enum PlayerType {
    Bot,
    Terminal,
}


#[derive(Debug)]
pub struct Player {
    name: String,
    player_type: PlayerType,
    hand: Pile,
    deck: Pile,
}

impl Player {

    pub fn bot(name: String) -> Player {
        Player {
            name,
            player_type: PlayerType::Bot,
            hand: Pile::new(),
            deck: Pile::starter_deck(),
        }

    }

    pub fn terminal(name: String) -> Player {
        Player {
            name,
            player_type: PlayerType::Terminal,
            hand: Pile::new(),
            deck: Pile::starter_deck(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

}
