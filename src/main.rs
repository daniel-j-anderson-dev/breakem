use breakem::game::Game;
use macroquad::prelude::*;

#[macroquad::main("breakem")]
async fn main() {
    Game::seed_random();
    let mut game = Game::default();

    loop {
        clear_background(GRAY);

        game.update();

        next_frame().await;
    }
}
