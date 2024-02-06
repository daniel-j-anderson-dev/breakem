use super::{draw::screen_size, Game, GameState};
use macroquad::{
    math::Vec2,
    ui::{root_ui, widgets::Window},
};

impl Game {
    pub(super) fn display_menu(&mut self, menu: Menu) {
        let screen_origin = screen_size() / 2.0;

        let buffer_space: f32 = (screen_origin.y / 10.0).max(21.0);

        let [label_position, play_button_position, exit_button_position] = [
            Vec2::new(screen_origin.x, screen_origin.y / 2.0 + buffer_space),
            Vec2::new(screen_origin.x, screen_origin.y / 2.0 + buffer_space * 2.0),
            Vec2::new(screen_origin.x, screen_origin.y / 2.0 + buffer_space * 3.0),
        ];

        let [label_text, play_button_text, exit_button_text] = match menu {
            Menu::Main => ["Breakem", "start game", "quit"],
            Menu::Pause => ["Paused", "continue game", "return to main menu"],
            Menu::GameOver => ["GAME OVER", "continue game", "return to main menu"],
            Menu::LevelComplete => ["Level Complete", "next level", "return to main menu"],
        };

        let window = Window::new(0, Vec2::ZERO, screen_origin);
        window.ui(&mut root_ui(), |ui| {
            ui.label(label_position, label_text);

            if ui.button(play_button_position, play_button_text) {
                match menu {
                    Menu::Main => self.full_reset(),
                    Menu::GameOver => self.level_reset(),
                    Menu::LevelComplete => self.next_level(),
                    Menu::Pause => {}
                };
                self.state = GameState::Playing
            }

            if ui.button(exit_button_position, exit_button_text) {
                self.state = match menu {
                    Menu::Main => GameState::Quit,
                    _ => GameState::NotPlaying(Menu::Main),
                }
            }
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Menu {
    Main,
    Pause,
    GameOver,
    LevelComplete,
}
