pub mod block;

use block::Block;
use macroquad::prelude::*;

#[derive(Debug)]
pub struct Level {
    blocks: Vec<Block>,
    boundary: Rect,
    id: usize,
}

impl Level {
    pub fn one() -> Self {
        return include!("level/one.rs");
    }
}

// getters
impl Level {
    pub fn blocks(&self) -> &[Block] {
        return &self.blocks;
    }
    pub fn boundary(&self) -> Rect {
        return self.boundary;
    }
    pub fn id(&self) -> usize {
        return self.id;
    }

    pub fn blocks_mut(&mut self) -> &mut [Block] {
        return &mut self.blocks;
    }
}