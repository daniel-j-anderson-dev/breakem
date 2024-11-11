use macroquad::{
    color::{Color, BLACK},
    math::{Rect, Vec2},
    shapes::{draw_rectangle, draw_rectangle_lines},
    window::{screen_height, screen_width},
};

use super::Game;

const BORDER_THICKNESS: f32 = 2.5;
pub const DEFAULT_BORDER_COLOR: Color = BLACK;

impl Game {
    pub(super) fn draw(&self) {
        self.draw_play_field();
        self.draw_level();
        self.draw_ball();
        self.draw_paddle();
    }
    fn draw_play_field(&self) {
        draw_rectangle_lines(
            self.play_field_hitbox.x + self.play_field_offset.x,
            self.play_field_hitbox.y + self.play_field_offset.y,
            self.play_field_hitbox.w,
            self.play_field_hitbox.h,
            BORDER_THICKNESS,
            DEFAULT_BORDER_COLOR,
        );
    }
    fn draw_level(&self) {
        for block in self.level.blocks() {
            if block.is_alive() {
                draw_bordered_offset(
                    self.play_field_offset,
                    block.hitbox(),
                    block.interior_color(),
                    block.border_color(),
                );
            }
        }
    }
    fn draw_ball(&self) {
        draw_bordered_offset(
            self.play_field_offset,
            self.ball.hitbox(),
            self.ball.interior_color(),
            self.ball.border_color(),
        );
    }
    fn draw_paddle(&self) {
        draw_bordered_offset(
            self.play_field_offset,
            self.paddle.hitbox(),
            self.paddle.interior_color(),
            self.paddle.border_color(),
        );
    }
}

pub fn draw_bordered_offset(offset: Vec2, rect: &Rect, interior_color: Color, border_color: Color) {
    draw_rectangle(
        offset.x + rect.x,
        offset.y + rect.y,
        rect.w,
        rect.h,
        border_color,
    );
    draw_rectangle(
        offset.x + rect.x + BORDER_THICKNESS,
        offset.y + rect.y + BORDER_THICKNESS,
        rect.w - (BORDER_THICKNESS * (5.0 / 3.0)),
        rect.h - (BORDER_THICKNESS * (5.0 / 3.0)),
        interior_color,
    );
}

pub fn screen_size() -> Vec2 {
    Vec2::new(screen_width(), screen_height())
}
