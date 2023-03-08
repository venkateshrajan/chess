mod board;
mod engine;
mod player;
mod strategy;
mod ui;

use engine::*;

fn main() {
    let mut engine = ConsoleEngine::new();
    engine.game_loop();
}
