
pub struct Pokemon {
    num: i32,
    name: String,
    type_1: usize,
    type_2: Option<usize>,
    hp: i32,
    hp_max: i32,
    atk: i32,
    def: i32,
    sp_atk: i32,
    sp_def: i32,
    speed: i32
}

impl Pokemon {
    pub fn new(num: i32, name: String, type_1: usize, type_2: Option<usize>, hp: i32, atk: i32, def: i32, sp_atk: i32, sp_def: i32, speed: i32) -> Pokemon {
        Pokemon {
            num: num,
            name: name,
            type_1: type_1,
            type_2: type_2,
            hp: hp,
            hp_max: hp,
            atk: atk,
            def: def,
            sp_atk: sp_atk,
            sp_def: sp_def,
            speed: speed
        }
    }

    pub fn get_num(&self) -> i32 {
        self.num
    }

    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }
}