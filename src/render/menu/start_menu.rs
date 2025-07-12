use macroquad::{
    input::{
        is_key_pressed,
        KeyCode::{
            Down, Up
        }
    }, shapes::draw_rectangle_lines, text::{
        draw_text_ex,
        TextParams,
    }, window::{
        screen_height,
        screen_width,
    }
};

use crate::{configuration::Configuration, game_state::GameState};

const MENU_FONT_SIZE: f32 = 30.0;
const MENU_ITEMS: [&str; 3] = [
    "New World",
    "Select Character",
    "Options",
];
pub fn draw_start_menu(mut game_state: GameState) -> GameState {
    
    draw_rectangle_lines(
        (screen_width() * 3.0) / 10.0,
        40.0,
        (screen_width() * 4.0) / 10.0,
        screen_height() - 100.0, 5.0,
        game_state.config.theme,
    );
    for (index, item) in MENU_ITEMS.iter().enumerate() {
        let text = if index == game_state.state {
                                    format!("[{}]", item)
                                } else {
                                    item.to_owned().to_owned()
                                };
        draw_menu_text(
            text,
            &game_state.config,
            index
        );
    }
    
    if is_key_pressed(Down) {
        if (game_state.state + 1) >= MENU_ITEMS.len() {
            game_state.state = 0;
            return game_state
        } else {
            game_state.state += 1;
            return game_state
        }
    } else if is_key_pressed(Up) {
        if game_state.state == 0 {
            game_state.state = MENU_ITEMS.len()-1;
            return game_state
        } else {
            game_state.state -= 1;
            return game_state
        }
    } else {
        return game_state
    };
}

fn get_text_width(text: &str, font_size: f32) -> f32 {
    text.len() as f32 * (font_size / 2.0)
}

fn draw_menu_text(text: String, config: &Configuration, menu_position: usize) {
    draw_text_ex(
        text.as_str(),
        ((screen_width() * 5.0) / 10.0) - (get_text_width(text.as_str(), screen_height() / MENU_FONT_SIZE) / 2.0),
        ((menu_position * 40) + 100) as f32,
        TextParams {
            font: Some(&config.font),
            font_size: (screen_height() / MENU_FONT_SIZE).round() as u16,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            rotation: 0.0,
            color: config.theme,
        }
    );
}