mod board;
mod engine;

use engine::*;

fn main() {
    let mut engine = Game::new();
    engine.game_loop();
}
