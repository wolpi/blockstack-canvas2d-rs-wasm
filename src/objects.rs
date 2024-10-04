use crate::block_stack;
use crate::blocks;

#[derive(Copy, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(PartialEq)]
pub enum Movement {
    NONE,
    LEFT,
    RIGHT,
    RotateLeft,
    RotateRight,
}

pub struct DrawState<'draw_run> {
    pub current_block: &'draw_run blocks::Block,
    pub next_block: &'draw_run blocks::Block,
    pub block_stack: &'draw_run block_stack::BlockStack,
}
