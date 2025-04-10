
pub struct Pokemon {
    num: i32,
    name: String,
    _type_1: usize,
    _type_2: Option<usize>,
    _hp: i32,
    _hp_max: i32,
    _atk: i32,
    _def: i32,
    _sp_atk: i32,
    _sp_def: i32,
    _speed: i32
}

impl Pokemon {
    pub fn new(num: i32, name: String, type_1: usize, type_2: Option<usize>, hp: i32, atk: i32, def: i32, sp_atk: i32, sp_def: i32, speed: i32) -> Pokemon {
        Pokemon {
            num: num,
            name: name,
            _type_1: type_1,
            _type_2: type_2,
            _hp: hp,
            _hp_max: hp,
            _atk: atk,
            _def: def,
            _sp_atk: sp_atk,
            _sp_def: sp_def,
            _speed: speed
        }
    }

    pub fn get_num(&self) -> i32 {
        self.num
    }

    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }
}