use macroquad::prelude::*;

use overworld::{Map, OverworldEntity, Player, Tileset};
mod overworld;

#[macroquad::main("Pokémon sous frozen")]
async fn main() {
    set_default_filter_mode(FilterMode::Nearest);
    
    let mut player : Player = Player::new();
    player.load().await;
    player.set_player_pos_grid(8, 8);

    let map = Map::new("maps/debug/testmap.pkmap".to_owned());
    let mut tileset = Tileset::new();
    tileset.load_tileset().await;

    loop {
        clear_background(WHITE);

        if player.get_x() % 16.0 == 0.0 && player.get_y() % 16.0 == 0.0 {
            if is_key_down(KeyCode::Up) {
                player.set_orientation(0);
            }
    
            if is_key_down(KeyCode::Down) {
                player.set_orientation(2);
            }
    
            if is_key_down(KeyCode::Left) {
                player.set_orientation(1);
            }
    
            if is_key_down(KeyCode::Right) {
                player.set_orientation(3);
            }

            if is_key_down(KeyCode::Right) | is_key_down(KeyCode::Down) | is_key_down(KeyCode::Left) | is_key_down(KeyCode::Up) {
                player.set_moving(&map, &tileset);
            } else {
                player.stop_moving();
            }
        }

        player.update();
        
        set_camera(&Camera2D {
            zoom: vec2(0.01, 0.013),
            ..Default::default()
        });

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
        
        next_frame().await
    }

}
