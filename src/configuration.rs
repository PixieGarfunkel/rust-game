use macroquad::{
    color::{
        Color,
        GREEN,
    },
    text::{
        Font,
        load_ttf_font,
    },
};

#[derive(Clone)]
pub struct Configuration {
    pub theme: Color,
    pub font: Font,
}

impl Configuration {
    pub async fn new() -> Self {
        Self {
            theme: GREEN,
            font: load_ttf_font(
                    "C:/Users/jet84/Documents/rust-game/src/assets/Consolas.ttf"
                ).await.expect("Unable to load font"),
        }
    }
}