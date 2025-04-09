use super::Pokemon;

pub struct Trainer {
    team : Vec<Pokemon>,
    active_pokemon: usize
}

impl Trainer {
    pub fn new() -> Trainer {
        Trainer {
            team: Vec::new(),
            active_pokemon: 0
        }
    }

    pub fn init_debug(&mut self) {
        let poke : Pokemon = Pokemon::new(0, "Rhydon".to_owned(), 0, None, 255, 255, 255, 255, 255, 255);
        self.team.push(poke);
    }

    pub fn get_active_pokemon(&self) -> &Pokemon {
        &self.team[self.active_pokemon]
    }
}