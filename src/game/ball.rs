use macroquad::prelude::*;

use super::{draw::DEFAULT_BORDER_COLOR, Paddle};

pub struct Ball {
    boundary: Rect,
    velocity: Vec2,
    interior_color: Color,
    border_color: Color,
}

// constructors and constants
impl Ball {
    pub const DEFAULT_SIZE: f32 = 10.0;
    pub fn on_paddle(paddle: &Paddle) -> Self {
        return Ball {
            boundary: Rect {
                x: paddle.boundary().x + (paddle.boundary().w / 2.0),
                y: paddle.boundary().top() - Ball::DEFAULT_SIZE,
                w: Ball::DEFAULT_SIZE,
                h: Ball::DEFAULT_SIZE,
            },
            velocity: Vec2::NEG_ONE.normalize() * 5.0,
            interior_color: paddle.interior_color(),
            border_color: DEFAULT_BORDER_COLOR,
        };
    }
}

// setters and getters
impl Ball {
    pub fn boundary(&self) -> &Rect {
        return &self.boundary;
    }
    pub fn position(&self) -> Vec2 {
        return self.boundary.point();
    }
    pub fn velocity(&self) -> Vec2 {
        return self.velocity;
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
    pub fn set_position(&mut self, position: Vec2) {
        self.boundary.move_to(position);
    }
    pub fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity;
    }
}

// actions
impl Ball {
    pub fn apply_velocity(&mut self) {
        self.boundary.x += self.velocity.x;
        self.boundary.y += self.velocity.y;
    }
    pub fn reflect_velocity_x(&mut self) {
        self.velocity.x = -self.velocity.x;
    }
    pub fn reflect_velocity_y(&mut self) {
        self.velocity.y = -self.velocity.y;
    }
}
