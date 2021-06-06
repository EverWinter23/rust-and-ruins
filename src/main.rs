mod components;
mod constants;
mod entity;
mod game;
mod managers;
mod timer;

use constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use game::Game;

fn main() {
    let mut game = Game::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    game.init();
    while game.is_running {
        game.process_input();
        game.update();
        game.render();
    }
}
