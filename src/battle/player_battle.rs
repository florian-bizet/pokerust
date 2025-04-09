use super::{Pokemon, Trainer};

pub struct PlayerBattle {
    trainer : Trainer
}

impl PlayerBattle {
    pub fn new() -> PlayerBattle {
        let mut train = Trainer::new();
        train.init_debug();

        PlayerBattle { trainer: train }
    }

    pub fn get_active_pokemon(&self) -> &Pokemon {
        self.trainer.get_active_pokemon()
    }
}

