mod ball;
mod collision;
mod draw;
mod level;
mod menu;
mod paddle;

use crate::game::{ball::Ball, draw::screen_size, level::Level, menu::Menu, paddle::Paddle};
use macroquad::prelude::*;

pub enum GameState {
    NotPlaying(Menu),
    Playing,
    Quit,
}

pub struct Game {
    state: GameState,
    level: Level,
    paddle: Paddle,
    ball: Ball,
    play_field: Rect,
    play_field_offset: Vec2,
    lives: usize,
    score: usize,
}

// constructors
impl Game {
    pub fn new() -> Self {
        seed_random_with_current_time();

        let level = Level::one();
        let paddle = Paddle::in_level(&level);
        let ball = Ball::on_paddle(&paddle);

        let play_field_offset = (screen_size() - level.boundary().size()) / 2.0;

        let play_field = Rect {
            x: 0.0,
            y: 0.0,
            w: level.boundary().w,
            h: level.boundary().h + Paddle::LEVEL_BUFFER + Paddle::DEFAULT_HEIGHT,
        };

        return Game {
            state: GameState::NotPlaying(Menu::Main),
            level,
            paddle,
            ball,
            play_field,
            play_field_offset,
            lives: 10,
            score: 0,
        };
    }
}

// update functions
impl Game {
    pub fn update(&mut self) {
        match self.state {
            GameState::NotPlaying(menu) => self.display_menu(menu),
            GameState::Playing => self.tick(),
            GameState::Quit => std::process::exit(0),
        };
    }
    fn tick(&mut self) {
        if is_key_pressed(KeyCode::Escape) {
            self.state = GameState::NotPlaying(Menu::Pause);
            return;
        }

        if self.lives == 0 {
            self.state = GameState::NotPlaying(Menu::GameOver);
            return;
        }

        if self.level.is_complete() {
            self.state = GameState::NotPlaying(Menu::LevelComplete);
        }

        self.ball.apply_velocity();

        self.paddle.handle_input();
        self.paddle.apply_velocity();

        self.handle_collision();

        self.draw();
    }
}

// helpers
impl Game {
    pub fn full_reset(&mut self) {
        *self = Game::new();
    }
    fn level_reset(&mut self) {
        self.level = Level::one();
        self.paddle = Paddle::in_level(&self.level);
        self.ball = Ball::on_paddle(&self.paddle);
        self.score = 0;
        self.lives = 10;
    }
    fn next_level(&self) {
        todo!("change level");
    }
}

fn seed_random_with_current_time() {
    use macroquad::rand::srand;
    use std::time::{SystemTime, UNIX_EPOCH};

    srand(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs(),
    );
}
