use macroquad::{
    color::{Color, BLACK, GRAY},
    math::{Rect, Vec2},
    shapes::{draw_line, draw_rectangle},
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
        draw_border_offset(
            self.play_field_offset,
            self.play_field,
            DEFAULT_BORDER_COLOR,
        );
    }
    fn draw_level(&self) {
        for block in self.level.blocks() {
            if block.is_alive {
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
pub fn draw_border_offset(offset: Vec2, boundary: Rect, color: Color) {
    let top_left = Vec2::new(boundary.left(), boundary.top()) + offset;
    let top_right = Vec2::new(boundary.right(), boundary.top()) + offset;
    let bottom_left = Vec2::new(boundary.left(), boundary.bottom()) + offset;
    let bottom_right = Vec2::new(boundary.right(), boundary.bottom()) + offset;

    draw_line(
        top_left.x,
        top_left.y,
        top_right.x,
        top_right.y,
        BORDER_THICKNESS,
        color,
    );
    draw_line(
        top_right.x,
        top_right.y,
        bottom_right.x,
        bottom_right.y,
        BORDER_THICKNESS,
        color,
    );
    draw_line(
        bottom_right.x,
        bottom_right.y,
        bottom_left.x,
        bottom_left.y,
        BORDER_THICKNESS,
        color,
    );
    draw_line(
        bottom_left.x,
        bottom_left.y,
        top_left.x,
        top_left.y,
        BORDER_THICKNESS,
        color,
    );
}

pub fn screen_size() -> Vec2 {
    return Vec2::new(screen_width(), screen_height());
}

pub fn random_color() -> Color {
    use macroquad::rand::gen_range;

    return Color {
        r: gen_range(0.0, 1.0),
        g: gen_range(0.0, 1.0),
        b: gen_range(0.0, 1.0),
        a: gen_range(0.0, 1.0),
    };
}
