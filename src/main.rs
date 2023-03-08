mod board;
mod engine;
mod player;
mod strategy;

use engine::*;

fn main() {
    let mut engine = Engine::new();
    engine.game_loop();
}
