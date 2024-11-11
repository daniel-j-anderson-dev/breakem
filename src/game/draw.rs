use macroquad::{
    color::{Color, BLACK, GRAY},
    math::{Rect, Vec2},
    shapes::{draw_line, draw_rectangle, draw_rectangle_lines},
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
            self.play_field.x + self.play_field_offset.x,
            self.play_field.y + self.play_field_offset.y,
            self.play_field.w,
            self.play_field.h,
            BORDER_THICKNESS,
            DEFAULT_BORDER_COLOR,
        );
    }
    fn draw_level(&self) {
        for block in self.level.blocks() {
            if block.is_alive() {
                draw_bordered_offset(
                    self.play_field_offset,
                    block.boundary(),
                    block.interior_color(),
                    block.border_color(),
                );
            }
        }
    }
    fn draw_ball(&self) {
        draw_bordered_offset(
            self.play_field_offset,
            self.ball.boundary(),
            self.ball.interior_color(),
            self.ball.border_color(),
        );
    }
    fn draw_paddle(&self) {
        draw_bordered_offset(
            self.play_field_offset,
            self.paddle.boundary(),
            self.paddle.interior_color(),
            self.paddle.border_color(),
        );
    }
}

pub fn draw_bordered_offset(
    offset: Vec2,
    boundary: &Rect,
    interior_color: Color,
    border_color: Color,
) {
    draw_rectangle(
        offset.x + boundary.x,
        offset.y + boundary.y,
        boundary.w,
        boundary.h,
        border_color,
    );
    draw_rectangle(
        offset.x + boundary.x + BORDER_THICKNESS,
        offset.y + boundary.y + BORDER_THICKNESS,
        boundary.w - (BORDER_THICKNESS * (5.0 / 3.0)),
        boundary.h - (BORDER_THICKNESS * (5.0 / 3.0)),
        interior_color,
    );
}

pub fn screen_size() -> Vec2 {
    return Vec2::new(screen_width(), screen_height());
}
