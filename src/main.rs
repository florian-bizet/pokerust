use battle::Battle;
use macroquad::prelude::*;

use overworld::{Map, OverworldEntity, Player, Tileset};
mod overworld;

mod battle;

#[macroquad::main("Pokémon sous frozen")]
async fn main() {
    set_default_filter_mode(FilterMode::Nearest);

    let font = load_ttf_font("PKMN RBYGSC.ttf").await.unwrap();
    
    let mut player : Player = Player::new();
    player.load().await;
    player.set_player_pos_grid(8, 8);

    let map = Map::new("maps/debug/testmap.pkmap".to_owned());
    let mut tileset = Tileset::new();
    tileset.load_tileset().await;

    let mut battle : Battle = Battle::new();
    battle.load().await;

    loop {
        

        if player.is_battling() { //TODO: DONT FORGET TO REMOVE THE !
            set_camera(&Camera2D {
                zoom: vec2(0.008, 0.011),
                ..Default::default()
            });
            clear_background(WHITE);
            battle.draw(&font);
            
            
        } else {
            set_camera(&Camera2D {
                zoom: vec2(0.01, 0.013),
                ..Default::default()
            });

            clear_background(WHITE);
            
            player.update(&map, &tileset);

            //drawing map
            map.draw(&tileset, &player);
            draw_texture(&player.get_texture(), 0.0, 0.0, WHITE);
            
        }
        next_frame().await
    }

}
