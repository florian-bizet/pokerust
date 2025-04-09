use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Map {
    tiles : Vec<usize>,
    height : i32,
    _width : i32
}

impl Map {
    pub fn new(file_path : String) -> Map {
        let mut vec = Vec::new();

        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            for elt in line.split(" ") {
                vec.push(elt.parse().unwrap());
            }
        }

        Map {tiles : vec, height : 16, _width : 16} //TODO: set custom
    }

    pub fn get_tile(&self, i : i32, j : i32) -> usize {
        let index = self.height*i + j;
        self.tiles[usize::try_from(index).unwrap()]
    }
}