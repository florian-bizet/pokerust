use super::Trainer;

pub struct PlayerBattle {
    trainer : Trainer
}

impl PlayerBattle {
    pub fn new() -> PlayerBattle {
        let mut train = Trainer::new();
        train.init_debug();

        PlayerBattle { trainer: train }
    }
}

