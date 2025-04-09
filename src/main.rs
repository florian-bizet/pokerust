use battle::Battle;
use macroquad::prelude::*;

use overworld::{Map, OverworldEntity, Player, Tileset};
mod overworld;

mod battle;

#[macroquad::main("Pokémon sous frozen")]
async fn main() {
    set_default_filter_mode(FilterMode::Nearest);
    
    let mut player : Player = Player::new();
    player.load().await;
    player.set_player_pos_grid(8, 8);

    let map = Map::new("maps/debug/testmap.pkmap".to_owned());
    let mut tileset = Tileset::new();
    tileset.load_tileset().await;

    let mut battle : Battle = Battle::new();

    loop {
        set_camera(&Camera2D {
            zoom: vec2(0.01, 0.013),
            ..Default::default()
        });

        if !player.is_battling() { //TODO: DONT FORGET TO REMOVE THE !
            clear_background(WHITE);
            //Enemy Poke
            //Your Poke
            //Menu
            set_default_camera();
            draw_rectangle(0.0, screen_height()-180.0, screen_width(), 180.0, BLACK);
            draw_rectangle(5.0, screen_height()-175.0, screen_width()-10.0, 170.0, WHITE);
        } else {
            clear_background(WHITE);
            
            player.update(&map, &tileset);
            //drawing map
            let mut texture : Texture2D = tileset.get_tile(1).clone();
            let mut previous_tile : usize = 1;
            let mut tile : usize;
            for i in 0..16 {
                for j in 0..16 {
                    tile = map.get_tile(i, j);
                    if tile != 0 {
                        if tile != previous_tile {
                            texture = tileset.get_tile(tile).clone();
                        }
                        draw_texture(&texture, 16.0*(j as f32)-player.get_x(), 16.0*(i as f32)-player.get_y(), WHITE);
                    }
                    previous_tile = tile;
                }
            }
        
            draw_texture(&player.get_texture(), 0.0, 0.0, WHITE);
            
        }
        next_frame().await
    }

}
