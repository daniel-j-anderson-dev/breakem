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
    pub fn next_boundary(&self) -> Rect {
        let mut next_boundary = self.boundary;
        next_boundary.x += self.velocity.x;
        next_boundary.y += self.velocity.y;
        return next_boundary;
    }
    pub fn next_position(&self) -> Vec2 {
        let mut next_position = self.position();
        next_position.x += self.velocity.x;
        next_position.y += self.velocity.y;
        return next_position;
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
        self.set_x_position(position.x);
        self.set_y_position(position.y);
    }
    pub fn set_x_position(&mut self, x: f32) {
        self.boundary.x = x;
    }
    pub fn set_y_position(&mut self, y: f32) {
        self.boundary.y = y;
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
    pub fn reflect_x_velocity(&mut self) {
        self.velocity.x = -self.velocity.x;
    }
    pub fn reflect_y_velocity(&mut self) {
        self.velocity.y = -self.velocity.y;
    }
}
