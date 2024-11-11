use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Block {
    hitbox: Rect,
    is_alive: bool,
    interior_color: Color,
    border_color: Color,
}

// constructors and constants
impl Block {
    pub fn new(hitbox: Rect, color: Color, border_color: Color) -> Self {
        Block {
            hitbox,
            interior_color: color,
            is_alive: true,
            border_color,
        }
    }
}

// setters and getters
impl Block {
    pub fn border_color(&self) -> Color {
        self.border_color
    }
    pub fn hitbox(&self) -> &Rect {
        &self.hitbox
    }
    pub fn interior_color(&self) -> Color {
        self.interior_color
    }
    pub fn is_alive(&self) -> bool {
        self.is_alive
    }
    pub fn set_is_alive(&mut self, is_alive: bool) {
        self.is_alive = is_alive
    }
}

fn random_color() -> Color {
    use macroquad::rand::gen_range;

    Color {
        r: gen_range(0.0, 1.0),
        g: gen_range(0.0, 1.0),
        b: gen_range(0.0, 1.0),
        a: gen_range(0.0, 1.0),
    }
}
