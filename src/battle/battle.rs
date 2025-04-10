use macroquad::prelude::*;

use super::PlayerBattle;
use super::Pokemon;
use super::Trainer;

use super::super::overworld::Animation;

enum BattleState {
    Intro,
    AppearText,
    SendPokemon,
    SendEnemy,
    Idle
}
pub struct Battle {
    player : PlayerBattle,
    enemy: Trainer,
    is_wild: bool,
    player_tex : Option<Texture2D>,
    enemy_tex: Option<Texture2D>,
    state: BattleState,
    enemy_pos: f32,
    buffer : i32,
    menu_selection : i32
}

impl Battle {
    pub fn new() -> Battle {
        Battle { 
            player: PlayerBattle::new(), 
            enemy: Trainer::new(), 
            is_wild: true, 
            player_tex: None , 
            enemy_tex: None, 
            state: BattleState::Intro,
            enemy_pos: -160.0,
            buffer: 0,
            menu_selection : 0
        }
    }

    pub async fn load(&mut self) {
        self.enemy.init_debug();
        
        let mut path = "tex/pkmn/back/".to_owned();
        path.push_str(&self.get_active_pokemon().get_num().to_string());
        path.push_str(".png");

        self.player_tex = Some(load_texture(&path).await.unwrap());

        path = "tex/pkmn/front/".to_owned();
        path.push_str(&self.get_enemy_active_pokemon().get_num().to_string());
        path.push_str(".png");

        self.enemy_tex = Some(load_texture(&path).await.unwrap());
    }

    pub fn get_active_pokemon(&self) -> &Pokemon {
        self.player.get_active_pokemon()
    }

    pub fn get_enemy_active_pokemon(&self) -> &Pokemon {
        self.enemy.get_active_pokemon()
    }

    pub fn update(&mut self) {
        //self.draw();
    }

    pub fn draw(&mut self, font: &Font) {
        //Menu
        set_default_camera();
        draw_rectangle(0.0, screen_height()-180.0, screen_width(), 180.0, BLACK);
        draw_rectangle(5.0, screen_height()-175.0, screen_width()-10.0, 170.0, WHITE);
        set_camera(&Camera2D {
            zoom: vec2(0.008, 0.011),
            ..Default::default()
        });
        
        match self.state {
            BattleState::Intro => {
                if (self.enemy_pos == 20.0) {
                    self.state = BattleState::AppearText;
                } else {
                    draw_texture(&self.enemy_tex.clone().unwrap(), self.enemy_pos, -84.0, BLACK);
                    self.enemy_pos += 2.0;
                }
            }

            BattleState::AppearText => {
                draw_texture(&self.enemy_tex.clone().unwrap(), 20.0, -84.0, WHITE);
                set_default_camera();
                //TODO: adapt pokemon name
                draw_text_ex("A wild RHYDON appears !", 25.0, 480.0, TextParams { font: Some(font), font_size: 28,  color: BLACK, ..Default::default() });
                if is_key_down(KeyCode::Enter) {
                    self.buffer = 0;
                    self.state = BattleState::SendPokemon;
                }
            }

            BattleState::SendPokemon => {
                draw_texture(&self.enemy_tex.clone().unwrap(), 20.0, -84.0, WHITE);
                draw_texture(&self.player_tex.clone().unwrap(), -86.0, -20.0, WHITE);
                set_default_camera();
                //TODO: adapt pokemon name
                draw_text_ex("Go, RHYDON !", 25.0, 480.0, TextParams { font: Some(font), font_size: 28,  color: BLACK, ..Default::default() });
                //TODO: pokeball animation
                if self.buffer > 60 {
                    self.state = BattleState::Idle;
                }
                self.buffer += 1;
            }

            BattleState::Idle => {
                draw_texture(&self.enemy_tex.clone().unwrap(), 20.0, -84.0, WHITE);
                draw_texture(&self.player_tex.clone().unwrap(), -86.0, -20.0, WHITE);
                set_default_camera();
                draw_text_ex("What should RHYDON do ?", 25.0, 480.0, TextParams { font: Some(font), font_size: 28,  color: BLACK, ..Default::default() });

                //Menu
                draw_text_ex("ATTACK", 560.0, 480.0, TextParams { font: Some(font), font_size: 28,  color: BLACK, ..Default::default() });
                draw_text_ex("ITEM", 560.0, 510.0, TextParams { font: Some(font), font_size: 28,  color: BLACK, ..Default::default() });
                draw_text_ex("POKEMON", 560.0, 540.0, TextParams { font: Some(font), font_size: 28,  color: BLACK, ..Default::default() });
                draw_text_ex("RUN", 560.0, 570.0, TextParams { font: Some(font), font_size: 28,  color: BLACK, ..Default::default() });

                //TODO: Menu selection
            }

            _ => {
                draw_texture(&self.player_tex.clone().unwrap(), -86.0, -20.0, WHITE);
                draw_texture(&self.enemy_tex.clone().unwrap(), 20.0, -84.0, WHITE);

                //ui
                set_default_camera();
                draw_text(&self.get_active_pokemon().get_name().to_uppercase(), 12.0, 120.0, 32.0, BLACK);

            }
        }

        
    }
}