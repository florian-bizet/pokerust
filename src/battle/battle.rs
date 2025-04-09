use macroquad::prelude::*;

use super::PlayerBattle;
use super::Pokemon;
use super::Trainer;

use super::super::overworld::Animation;

enum BattleAnimationState {
    Intro,
    Idle
}
pub struct Battle {
    player : PlayerBattle,
    enemy: Trainer,
    is_wild: bool,
    player_tex : Option<Texture2D>,
    enemy_tex: Option<Texture2D>,
    state: BattleAnimationState,
    enemy_pos: f32
}

impl Battle {
    pub fn new() -> Battle {
        Battle { 
            player: PlayerBattle::new(), 
            enemy: Trainer::new(), 
            is_wild: false, 
            player_tex: None , 
            enemy_tex: None, 
            state: BattleAnimationState::Intro,
            enemy_pos: -160.0
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
        self.draw();
    }

    pub fn draw(&mut self) {
        match self.state {
            BattleAnimationState::Intro => {
                if (self.enemy_pos == 20.0) {
                    self.state = BattleAnimationState::Idle;
                } else {
                    draw_texture(&self.enemy_tex.clone().unwrap(), self.enemy_pos, -84.0, BLACK);
                    self.enemy_pos += 2.0;
                }
            }

            _ => {
                draw_texture(&self.player_tex.clone().unwrap(), -86.0, -20.0, WHITE);
                draw_texture(&self.enemy_tex.clone().unwrap(), 20.0, -84.0, WHITE);

                //ui
                set_default_camera();
                draw_text(&self.get_active_pokemon().get_name().to_uppercase(), 0.0, 120.0, 32.0, BLACK);

            }
        }

        //Menu
        set_default_camera();
        draw_rectangle(0.0, screen_height()-180.0, screen_width(), 180.0, BLACK);
        draw_rectangle(5.0, screen_height()-175.0, screen_width()-10.0, 170.0, WHITE);
    }
}