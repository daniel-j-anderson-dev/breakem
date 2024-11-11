use macroquad::prelude::*;

use super::{draw::DEFAULT_BORDER_COLOR, Paddle};

pub struct Ball {
    hitbox: Rect,
    velocity: Vec2,
    interior_color: Color,
    border_color: Color,
}

// constructors and constants
impl Ball {
    pub const DEFAULT_SIZE: f32 = 10.0;
    pub fn on_paddle(paddle: &Paddle) -> Self {
        return Ball {
            hitbox: Rect {
                x: paddle.hitbox().x + (paddle.hitbox().w / 2.0),
                y: paddle.hitbox().top() - Ball::DEFAULT_SIZE,
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
    pub fn hitbox(&self) -> &Rect {
        return &self.hitbox;
    }
    pub fn next_hitbox(&self) -> Rect {
        let mut next_hitbox = self.hitbox;
        next_hitbox.x += self.velocity.x;
        next_hitbox.y += self.velocity.y;
        return next_hitbox;
    }
    pub fn next_position(&self) -> Vec2 {
        let mut next_position = self.position();
        next_position.x += self.velocity.x;
        next_position.y += self.velocity.y;
        return next_position;
    }
    pub fn position(&self) -> Vec2 {
        return self.hitbox.point();
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
    pub fn set_hitbox(&mut self, hitbox: Rect) {
        self.hitbox = hitbox;
    }
    pub fn set_position(&mut self, position: Vec2) {
        self.set_x_position(position.x);
        self.set_y_position(position.y);
    }
    pub fn set_x_position(&mut self, x: f32) {
        self.hitbox.x = x;
    }
    pub fn set_y_position(&mut self, y: f32) {
        self.hitbox.y = y;
    }
    pub fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity;
    }
}

// actions
impl Ball {
    pub fn apply_velocity(&mut self) {
        self.hitbox.x += self.velocity.x;
        self.hitbox.y += self.velocity.y;
    }
    pub fn reflect_x_velocity(&mut self) {
        self.velocity.x = -self.velocity.x;
    }
    pub fn reflect_y_velocity(&mut self) {
        self.velocity.y = -self.velocity.y;
    }
}
