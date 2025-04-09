use super::Pokemon;

pub struct Trainer {
    team : Vec<Pokemon>
}

impl Trainer {
    pub fn new() -> Trainer {
        Trainer {
            team: Vec::new()
        }
    }

    pub fn init_debug(&mut self) {
        let poke : Pokemon = Pokemon::new(001, "Rhydon".to_owned(), 0, None, 255, 255, 255, 255, 255, 255);
        self.team.push(poke);
    }
}