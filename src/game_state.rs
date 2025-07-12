use crate::configuration::Configuration;

pub struct GameState {
    pub menu: String,
    pub state: usize,
    pub config: Configuration
}