use crate::blocks;
use crate::objects::Point;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct BlockStack {
    block_stack: HashMap<i32, Vec<blocks::Block>>,
}

impl BlockStack {
    pub fn new() -> Self {
        Self {
            block_stack: HashMap::new(),
        }
    }

    pub fn get_stack(&self) -> &HashMap<i32, Vec<blocks::Block>> {
        &self.block_stack
    }

    pub fn check_collision(&self, piece: &Point) -> bool {
        let mut piece_ok = true;
        'outer: for (_index, row) in &self.block_stack {
            for stack_block in row {
                for stack_point in stack_block.get_pieces() {
                    if piece.x == stack_point.x && piece.y == stack_point.y {
                        piece_ok = false;
                        break 'outer;
                    }
                }
            }
        }
        return piece_ok;
    }

    pub fn add_block_to_stack(&mut self, block: &blocks::Block) {
        let block_stack = &mut self.block_stack;
        let stack_blocks = blocks::stack_blocks(&block);
        for block in stack_blocks {
            let index = block.get_pieces().get(0).unwrap().y;
            let row_opt = block_stack.get_mut(&index);
            let row: &mut Vec<blocks::Block>;
            if row_opt.is_none() {
                let tmp_row = Vec::new();
                block_stack.insert(index, tmp_row);
                // get row back from map directly after inserting it
                // to get lifetime right
                row = block_stack.get_mut(&index).unwrap();
            } else {
                row = row_opt.unwrap();
            }
            row.push(block);
        }
    }

    pub fn reduce_stack(&mut self, game_width: i32) -> usize {
        let complete_row_indexes = self.find_complete_row_indexes(game_width);
        self.remove_completed_rows(&complete_row_indexes);
        complete_row_indexes.len()
    }

    fn find_complete_row_indexes(&self, game_width: i32) -> HashSet<i32> {
        let mut complete_row_indexes: HashSet<i32> = HashSet::new();
        for (index, row) in &self.block_stack {
            let mut xs_in_row = HashSet::new();
            for block in row {
                let x = block.get_pieces().get(0).unwrap().x;
                xs_in_row.insert(x);
            }
            let mut row_complete = true;
            let mut i = 0;
            while i < game_width {
                let present = xs_in_row.get(&i);
                i = i + 1;
                if present.is_none() {
                    row_complete = false;
                    break;
                }
            }
            if row_complete {
                complete_row_indexes.insert(*index);
            }
        }
        complete_row_indexes
    }

    fn remove_completed_rows(&mut self, complete_row_indexes: &HashSet<i32>) {
        if !complete_row_indexes.is_empty() {
            // find out which rows need to be shifed by which amount
            let mut row_idx_to_be_shifted_by_count: HashMap<i32, i32> = HashMap::new();
            for old_index in self.block_stack.keys() {
                if complete_row_indexes.contains(old_index) {
                    // row will be removed
                    continue;
                }
                let mut to_be_shifted_by_rows = 0;
                for idx_to_be_removed in complete_row_indexes {
                    if *old_index < *idx_to_be_removed {
                        to_be_shifted_by_rows = to_be_shifted_by_rows + 1;
                    }
                }
                row_idx_to_be_shifted_by_count.insert(*old_index, to_be_shifted_by_rows);
            }
            // build map of old to new index
            let mut old_idx_to_new: HashMap<i32, i32> = HashMap::new();
            for old_index in self.block_stack.keys() {
                let to_by_shifted_by_opt = row_idx_to_be_shifted_by_count.get(old_index);
                if to_by_shifted_by_opt.is_some() {
                    let to_by_shifted_by = to_by_shifted_by_opt.unwrap();
                    let new_index = old_index + to_by_shifted_by;
                    old_idx_to_new.insert(*old_index, new_index);
                }
            }
            // build new stack
            let mut reduced_stack: HashMap<i32, Vec<blocks::Block>> = HashMap::new();
            for old_idx in old_idx_to_new.keys() {
                let new_idx = old_idx_to_new.get(old_idx).unwrap();
                let row_opt = self.block_stack.remove(old_idx);
                let row = row_opt.unwrap();
                reduced_stack.insert(*new_idx, row);
                // adjust y of all points
                let row_opt2 = reduced_stack.get_mut(new_idx);
                let row_mut = row_opt2.unwrap();
                for block in row_mut {
                    for point in block.get_pieces_mut() {
                        point.y = *new_idx;
                    }
                }
            }
            self.block_stack = reduced_stack;
        }
    }
}
