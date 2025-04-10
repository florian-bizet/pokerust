use std::fs::File;
use std::io::{BufRead, BufReader};

use macroquad::prelude::*;

use super::{Player, Tileset, OverworldEntity};

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

    pub fn draw(&self, tileset: &Tileset, player: &Player) {
        let mut texture : Texture2D = tileset.get_tile(1).clone();
        let mut previous_tile : usize = 1;
        let mut tile : usize;
        for i in 0..16 {
            for j in 0..16 {
                tile = self.get_tile(i, j);
                if tile != 0 {
                    if tile != previous_tile {
                        texture = tileset.get_tile(tile).clone();
                    }
                    draw_texture(&texture, 16.0*(j as f32)-player.get_x(), 16.0*(i as f32)-player.get_y(), WHITE);
                }
                previous_tile = tile;
            }
        }
    }
}