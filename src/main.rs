mod game;

use game::Game;

use macroquad::prelude::*;

pub const BACKGROUND_COLOR: Color = GRAY;

#[macroquad::main("breakem")]
async fn main() {
    let mut game = Game::new();

    loop {
        clear_background(BACKGROUND_COLOR);

        game.update();

        next_frame().await;
    }
}
