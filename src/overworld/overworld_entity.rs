use macroquad::prelude::*;

pub trait OverworldEntity {
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
    fn get_grid_x(&self) -> i32;
    fn get_grid_y(&self) -> i32;
    fn _get_orientation(&self) -> i32;
    fn get_texture(&mut self) -> Texture2D;
}