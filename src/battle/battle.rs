use super::PlayerBattle;
use super::Trainer;

pub struct Battle {
    player : PlayerBattle,
    enemy: Trainer,
    is_wild: bool
}

impl Battle {
    pub fn new() -> Battle {
        Battle { player: PlayerBattle::new(), enemy: Trainer::new(), is_wild: false }
    }
}