mod ball;
mod draw;
mod level;
mod paddle;

use crate::game::{ball::Ball, draw::screen_size, level::Level, paddle::Paddle};
use macroquad::{
    prelude::*,
    rand::gen_range,
    ui::{root_ui, widgets::Window},
};

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

// constructors and constants
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
    fn display_menu(&mut self, menu: Menu) {
        let screen_size = screen_size();

        let buffer_space: f32 = (screen_size.y / 20.0).max(21.0);

        let label_position = Vec2::new(screen_size.x / 2.0, screen_size.y / 4.0 + buffer_space);
        let play_button_position =
            Vec2::new(label_position.x, screen_size.y / 4.0 + buffer_space * 2.0);
        let exit_button_position =
            Vec2::new(label_position.x, screen_size.y / 4.0 + buffer_space * 3.0);

        let (label_text, play_button_text, exit_button_text) = match menu {
            Menu::Main => ("Breakem", "start game", "quit"),
            Menu::Pause => ("Paused", "continue game", "return to main menu"),
            Menu::GameOver => ("GAME OVER", "continue game", "return to main menu"),
            Menu::LevelComplete => ("Level Complete", "next level", "return to main menu"),
        };

        let window = Window::new(0, Vec2::ZERO, screen_size);
        window.ui(&mut root_ui(), |ui| {
            ui.label(label_position, label_text);

            if ui.button(play_button_position, play_button_text) {
                match menu {
                    Menu::Main => self.full_reset(),
                    Menu::GameOver => self.level_reset(),
                    Menu::Pause => {}
                    Menu::LevelComplete => todo!("change level"),
                };
                self.state = GameState::Playing
            }

            if ui.button(exit_button_position, exit_button_text) {
                self.state = match menu {
                    Menu::Main => GameState::Quit,
                    Menu::Pause | Menu::GameOver | Menu::LevelComplete => {
                        GameState::NotPlaying(Menu::Main)
                    }
                }
            }
        });
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

        if self
            .level
            .blocks()
            .iter()
            .filter(|block| block.is_alive)
            .count()
            == 0
        {
            self.state = GameState::NotPlaying(Menu::LevelComplete);
        }

        self.ball.apply_velocity();

        self.paddle.handle_input();
        self.paddle.apply_velocity();

        self.handle_collision();

        self.draw();
    }
}

// Game::Menu helpers
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
}

// Game::tick helpers
impl Game {
    fn handle_collision(&mut self) {
        self.keep_in_play_field();

        // ball and blocks
        for block in self.level.blocks_mut() {
            if block.boundary().overlaps(self.ball.boundary()) && block.is_alive {
                let offset = self.ball.boundary().center().x - block.boundary().center().x;
                let normalized_offset = offset / (block.boundary().w / 2.0);

                self.ball.set_velocity(Vec2::new(
                    normalized_offset * gen_range(2.0, 5.0),
                    -self.ball.velocity().y,
                ));

                block.is_alive = false;
            }
        }

        // ball and paddle
        if self.paddle.boundary().overlaps(self.ball.boundary()) {
            let offset = self.ball.boundary().center().x - self.paddle.boundary().center().x;
            let normalized_offset = offset / (self.paddle.boundary().w / 2.0);

            self.ball.set_velocity(Vec2::new(
                normalized_offset * gen_range(2.0, 5.0),
                -self.ball.velocity().y,
            ));
        }
    }
    fn keep_in_play_field(&mut self) {
        // keep paddle in bounds
        if self.paddle.next_boundary().left() < self.play_field.left() {
            self.paddle.set_position(Vec2::new(
                self.play_field.left(),
                self.paddle.next_boundary().y,
            ));
        }
        if self.paddle.next_boundary().right() > self.play_field.right() {
            self.paddle.set_position(Vec2::new(
                self.play_field.right() - self.paddle.next_boundary().w,
                self.paddle.next_boundary().y,
            ));
        }

        // keep ball in bounds. simple reflection on walls
        if self.ball.next_boundary().left() <= self.play_field.left()
            || self.ball.next_boundary().right() >= self.play_field.right()
        {
            self.ball.reflect_x_velocity();
        }
        if self.ball.next_boundary().top() <= self.play_field.top() {
            self.ball.reflect_y_velocity();
        }
        if self.ball.next_boundary().bottom() >= self.play_field.bottom() {
            self.ball.reflect_y_velocity();

            self.lives -= 1;
        }
    }
}

pub enum GameState {
    NotPlaying(Menu),
    Playing,
    Quit,
}

#[derive(Debug, Clone, Copy)]
pub enum Menu {
    Main,
    Pause,
    GameOver,
    LevelComplete,
}

fn seed_random_with_current_time() {
    
}
