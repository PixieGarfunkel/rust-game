use macroquad::{
    prelude::*
};

mod render;
use render::menu::menu::{
    draw_state
};

mod configuration;
use configuration::Configuration;

mod game_state;
use game_state::GameState;

fn window_conf() -> Conf {
    Conf {
        window_title: "My Game".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // let font = load_ttf_font("assets/Consolas.ttf")
    //     .await
    //     .expect("Failed to load font");
    let mut game_state: GameState = GameState {
        menu: "Start".to_owned(),
        state: 0,
        config: Configuration::new().await
    };

    loop {
        clear_background(BLACK);
        game_state = draw_state(game_state);
        next_frame().await;
    }
}