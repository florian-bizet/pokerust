use macroquad::prelude::*;
use macroquad::rand::rand;

use super::tileset::TileType;
use super::Map;
use super::OverworldEntity;
use super::Animation;
use super::Tileset;

pub struct Player {
    x : f32,
    y : f32,
    orientation: i32,
    animations: Vec<Animation>,
    current_anim : i32,
    current_anim_total : i32,
    anim_frames : i32,
    is_moving: bool,
    is_battling: bool
}

impl OverworldEntity for Player {
    fn get_x(&self) -> f32 {
        self.x
    }

    fn get_y(&self) -> f32 {
        self.y
    }

    fn get_grid_x(&self) -> i32 {
        (self.x / 16.0) as i32
    }

    fn get_grid_y(&self) -> i32 {
        (self.y / 16.0) as i32
    }

    fn _get_orientation(&self) -> i32 {
        self.orientation
    }

    fn get_texture(&mut self) -> Texture2D {
        let anim = self.animations.get_mut(usize::try_from(self.current_anim).unwrap());
        return anim.unwrap().get_texture(self.anim_frames);
    }
}

impl Player {
    pub fn new() -> Player {
        Player {
            x : 0.0,
            y : 0.0,
            orientation : 1,
            is_moving : false,
            animations : Vec::new(),
            current_anim : 0,
            current_anim_total : 1,
            anim_frames : 0,
            is_battling : false
        }
    }

    pub async fn load(&mut self) {
        //Idle Animations
        let mut idle_down = Animation::new("IdleDown".to_owned());
        idle_down.new_step(load_texture("tex/overworld/entities/red/redDown.png").await.unwrap(), 1);

        let mut idle_up = Animation::new("IdleUp".to_owned());
        idle_up.new_step(load_texture("tex/overworld/entities/red/redUp.png").await.unwrap(), 1);

        let mut idle_left = Animation::new("IdleLeft".to_owned());
        idle_left.new_step(load_texture("tex/overworld/entities/red/redLeft.png").await.unwrap(), 1);

        let mut idle_right = Animation::new("IdleRight".to_owned());
        idle_right.new_step(load_texture("tex/overworld/entities/red/redRight.png").await.unwrap(), 1);

        self.animations.push(idle_up);   //0
        self.animations.push(idle_left); //1
        self.animations.push(idle_down); //2
        self.animations.push(idle_right);//3

        //Walking animations
        let mut walk_up = Animation::new("WalkLeft".to_owned());
        walk_up.new_step(load_texture("tex/overworld/entities/red/redUp1.png").await.unwrap(), 8);
        walk_up.new_step(load_texture("tex/overworld/entities/red/redUp.png").await.unwrap(),  8);
        walk_up.new_step(load_texture("tex/overworld/entities/red/redUp2.png").await.unwrap(), 8);
        walk_up.new_step(load_texture("tex/overworld/entities/red/redUp.png").await.unwrap(),  8);

        let mut walk_left = Animation::new("WalkLeft".to_owned());
        walk_left.new_step(load_texture("tex/overworld/entities/red/redLeft1.png").await.unwrap(), 8);
        walk_left.new_step(load_texture("tex/overworld/entities/red/redLeft.png").await.unwrap(),  8);

        let mut walk_down = Animation::new("WalkLeft".to_owned());
        walk_down.new_step(load_texture("tex/overworld/entities/red/redDown1.png").await.unwrap(), 8);
        walk_down.new_step(load_texture("tex/overworld/entities/red/redDown.png").await.unwrap(),  8);
        walk_down.new_step(load_texture("tex/overworld/entities/red/redDown2.png").await.unwrap(), 8);
        walk_down.new_step(load_texture("tex/overworld/entities/red/redDown.png").await.unwrap(),  8);

        let mut walk_right = Animation::new("WalkLeft".to_owned());
        walk_right.new_step(load_texture("tex/overworld/entities/red/redRight1.png").await.unwrap(), 8);
        walk_right.new_step(load_texture("tex/overworld/entities/red/redRight.png").await.unwrap(),  8);

        self.animations.push(walk_up);
        self.animations.push(walk_left);
        self.animations.push(walk_down);
        self.animations.push(walk_right);
    }

    pub fn get_dx(&self) -> i32{
        match self.orientation {
            0 => {0}
            1 => {-1}
            2 => {0}
            3 => {1}
            _ => {0}
        }
    }

    pub fn get_dy(&self) -> i32{
        match self.orientation {
            0 => {-1}
            1 => {0}
            2 => {1}
            3 => {0}
            _ => {0}
        }
    }

    pub fn move_player(&mut self) {
        self.x += self.get_dx() as f32*2.0;
        self.y += self.get_dy() as f32*2.0;
    }

    pub fn can_move(&self, map : &Map, tileset : &Tileset) -> bool {
        let x = self.get_grid_x() + self.get_dx();
        let y = self.get_grid_y() + self.get_dy();

        let map_tile = map.get_tile(y, x);
        let tile_type = tileset.get_type(map_tile);

        match tile_type {
            TileType::Solid => {false}
            _ => {true}
        }
    }

    pub fn set_player_pos_grid(&mut self, x: i32, y: i32) {
        self.x = 16.0 * (x as f32);
        self.y = 16.0 * (y as f32);
    }

    pub fn _set_orientation(&mut self, ori : i32) {
        self.orientation = ori;
        self.current_anim = ori;
    }

    pub fn set_moving(&mut self, map: &Map, tileset: &Tileset) {
        if !self.can_move(map, tileset) {
            self.stop_moving();
            return;
        }
        
        self.is_moving = !self.is_moving;
        self.current_anim = 4 + self.orientation;
        let anim = self.animations.get_mut(usize::try_from(self.current_anim).unwrap());
        self.current_anim_total = anim.unwrap().get_total_frames();
    }

    pub fn tall_grass(&mut self, map : &Map, tileset: &Tileset) -> bool {
        let x = self.get_grid_x();
        let y = self.get_grid_y();
        let map_tile = map.get_tile(y, x);
        let tile_type = tileset.get_type(map_tile);

        match tile_type {
            TileType::TallGrass => {
                println!("HAUTES HERBES");
                let random = rand()%50;
                random == 3
            }
            _ => {false}
        }
    }

    pub fn stop_moving(&mut self) {
        self.is_moving = false;
        self.current_anim = self.orientation;
        self.current_anim_total = 1;
    }

    pub fn update(&mut self, map : &Map, tileset : &Tileset) {
        if self.x % 16.0 == 0.0 && self.y % 16.0 == 0.0 {
            if is_key_down(KeyCode::Up) {
                self.orientation = 0;
            }
    
            if is_key_down(KeyCode::Down) {
                self.orientation = 2;
            }
    
            if is_key_down(KeyCode::Left) {
                self.orientation = 1;
            }
    
            if is_key_down(KeyCode::Right) {
                self.orientation = 3;
            }

            if is_key_down(KeyCode::Right) | is_key_down(KeyCode::Down) | is_key_down(KeyCode::Left) | is_key_down(KeyCode::Up) {
                self.set_moving(&map, &tileset);
            } else {
                self.stop_moving();
            }
        }
        
        if self.is_moving {
            self.move_player();
             if self.x % 16.0 == 0.0 && self.y % 16.0 == 0.0 {
                if self.tall_grass(map, tileset) {
                    println!("POKEMON SAUVAGE");
                    self.stop_moving();
                    self.is_battling = true;
                    return;
                }
             }
        }



        self.anim_frames += 1;
        self.anim_frames = self.anim_frames%self.current_anim_total;
    }

    pub fn is_battling(&self) -> bool {self.is_battling}
}