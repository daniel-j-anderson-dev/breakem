use macroquad::prelude::*;

use super::{draw::DEFAULT_BORDER_COLOR, Level};

pub struct Paddle {
    boundary: Rect,
    velocity: Vec2,
    interior_color: Color,
    border_color: Color,
}

// constructors
impl Paddle {
    pub const DEFAULT_WIDTH: f32 = 50.0;
    pub const DEFAULT_HEIGHT: f32 = 10.0;
    pub const LEVEL_BUFFER: f32 = Paddle::DEFAULT_HEIGHT * 8.0;

    pub fn in_level(level: &Level) -> Self {
        return Paddle {
            boundary: Rect {
                x: level.boundary().left() + level.boundary().w / 2.0,
                y: level.boundary().bottom() + Paddle::LEVEL_BUFFER,
                w: Paddle::DEFAULT_WIDTH,
                h: Paddle::DEFAULT_HEIGHT,
            },
            velocity: Vec2::ZERO,
            interior_color: SKYBLUE,
            border_color: DEFAULT_BORDER_COLOR,
        };
    }
}

// getters and setters
impl Paddle {
    pub fn boundary(&self) -> &Rect {
        return &self.boundary;
    }
    pub fn velocity(&self) -> Vec2 {
        return self.velocity;
    }
    pub fn position(&self) -> Vec2 {
        return self.boundary.point();
    }
    pub fn interior_color(&self) -> Color {
        return self.interior_color;
    }
    pub fn border_color(&self) -> Color {
        return self.border_color;
    }
    pub fn set_border_color(&mut self, border_color: Color) {
        self.border_color = border_color;
    }
    pub fn set_boundary(&mut self, boundary: Rect) {
        self.boundary = boundary;
    }
    pub fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity;
    }
    pub fn set_color(&mut self, color: Color) {
        self.interior_color = color;
    }
    pub fn set_position(&mut self, position: Vec2) {
        self.boundary.move_to(position);
    }
}

// actions
impl Paddle {
    pub fn handle_input(&mut self) {
        self.velocity = Vec2::ZERO;

        if is_key_down(KeyCode::Left) {
            self.velocity.x -= 1.0;
        }
        if is_key_down(KeyCode::Right) {
            self.velocity.x += 1.0;
        }

        self.velocity = self.velocity.normalize_or_zero() * 10.0;
    }
    pub fn apply_velocity(&mut self) {
        self.boundary.x += self.velocity.x;
        self.boundary.y += self.velocity.y;
    }
}
