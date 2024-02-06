mod game;

use game::Game;
use macroquad::prelude::*;

#[macroquad::main("breakem")]
async fn main() {
    let mut game = Game::new();

    loop {
        clear_background(GRAY);

        game.update();

        next_frame().await;
    }
}
