mod board;
mod engine;

use engine::*;

fn main() {
    let engine = Game::new();
    engine.game_loop();
}
