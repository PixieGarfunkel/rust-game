use crate::render::menu::start_menu::{
    draw_start_menu,
};
use crate::game_state::GameState;

pub fn draw_state(game_state: GameState) -> GameState {
    match game_state.menu.as_str() {
        "Start" => draw_start_menu(game_state),
        _ => {panic!()},
    }
}
