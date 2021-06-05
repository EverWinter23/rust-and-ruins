mod constants;
mod game;
mod timer;

use constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use game::Game;

fn main() {
    let mut game = Game::new();
    game.init(WINDOW_WIDTH, WINDOW_HEIGHT);
    while game.is_running {
        game.process_input();
        game.update();
        game.render();
    }
}
