pub mod block;

use block::Block;
use macroquad::prelude::*;

#[derive(Debug)]
pub struct Level {
    blocks: Vec<Block>,
    hitbox: Rect,
    id: usize,
}

impl Level {
    pub fn one() -> Self {
        include!("level/one.rs")
    }
}

// getters
impl Level {
    pub fn hitbox(&self) -> Rect {
        self.hitbox
    }
    pub fn id(&self) -> usize {
        self.id
    }
    pub fn blocks(&self) -> impl Iterator<Item = &Block> {
        self.blocks.iter()
    }
    pub fn blocks_mut(&mut self) -> impl Iterator<Item = &mut Block> {
        self.blocks.iter_mut()
    }
    pub fn is_complete(&self) -> bool {
        self.blocks().filter(|block| block.is_alive()).count() == 0
    }
}
