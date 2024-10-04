use crate::block_stack;
use crate::block_stack::BlockStack;
use crate::blocks;
use crate::colours;
use crate::draw::Draw;
use crate::highscore;
use crate::objects;
use crate::objects::DrawState;
use crate::textdisplay::set_background_colour;
use crate::textdisplay::update_duration_display;
use crate::textdisplay::update_text_display;
use crate::utils::log;
use crate::utils::random;

pub const DEFAULT_INPUT: char = '1';
pub const GAME_WIDTH: i32 = 16;
pub const GAME_HEIGHT: i32 = 30;
const ROWS_FOR_LEVEL_UP: i32 = 10;
const FRAME_RATE_SPEED_1: i32 = 1000 / 2;
const SPEED_INCREASE_MS: i32 = 50;

pub struct Game {
    score: i32,
    lines: i32,
    level: i32,
    name: String,
    draw: Option<Draw>,
    current_block: blocks::Block,
    next_block: blocks::Block,
    block_stack: Option<block_stack::BlockStack>,
    movement: objects::Movement,
    over: bool,
    pause: bool,
    input: char,
    input_processed: bool,
    pressed: bool,
    speed_increase: bool,
    timestamp_last_frame: u32,
    timestamp_game_start: u32,
    timestamp_pause_start: u32,
    pause_duration_sum: u32,
}

impl Game {
    pub const fn default() -> Self {
        Self {
            score: 0,
            lines: 0,
            level: 1,
            name: String::new(),
            draw: None,
            current_block: blocks::default_block(),
            next_block: blocks::default_block(),
            block_stack: None,
            movement: objects::Movement::NONE,
            over: true,
            pause: false,
            input: DEFAULT_INPUT,
            input_processed: false,
            pressed: false,
            speed_increase: false,
            timestamp_last_frame: 0,
            timestamp_game_start: 0,
            timestamp_pause_start: 0,
            pause_duration_sum: 0,
        }
    }
    pub fn set_state(&mut self, name: &str, start_level: i32, draw: Draw) {
        log!("  re-setting game state!");

        let level_to_use = if start_level < 0 || start_level > 9 {
            1
        } else {
            start_level
        };

        self.score = 0;
        self.lines = 0;
        self.level = level_to_use;
        self.name = name.to_string();
        self.draw = Some(draw);
        self.current_block = self.create_block();
        self.next_block = self.create_block();
        self.block_stack = Some(BlockStack::new());
        self.movement = objects::Movement::NONE;
        self.over = false;
        self.pause = false;
        self.input = DEFAULT_INPUT;
        self.input_processed = true;
        self.pressed = false;
        self.speed_increase = false;
        self.timestamp_last_frame = 0;
        self.timestamp_game_start = 0;
        self.timestamp_pause_start = 0;
        self.pause_duration_sum = 0;
        set_background_colour("#FFF");
        update_text_display(self.score, self.lines, self.level);
    }

    pub fn is_over(&self) -> bool {
        self.over
    }

    fn create_block(&self) -> blocks::Block {
        let rand = random(0, 6);
        blocks::new(rand, self.level)
    }

    pub fn world_loop_contents(&mut self, timestamp: u32) -> bool {
        let mut first_frame = false;
        if self.timestamp_game_start == 0 {
            self.timestamp_game_start = timestamp;
            first_frame = true;
        }
        if !self.over && self.enough_time_passed(timestamp) {
            self.process_input(timestamp);
            if self.pause {
                self.draw.as_ref().unwrap().draw_pause();
            } else {
                let continue_rendering = self.update_world(first_frame);
                if continue_rendering {
                    update_duration_display(self.calc_duration(timestamp));
                    self.draw.as_ref().unwrap().draw(DrawState {
                        current_block: &self.current_block,
                        next_block: &self.next_block,
                        block_stack: &self.block_stack.as_ref().unwrap(),
                    });
                } else {
                    self.game_over(timestamp);
                    return false;
                }
            }
        }
        true
    }

    fn enough_time_passed(&mut self, timestamp: u32) -> bool {
        if timestamp > self.timestamp_last_frame {
            let mut threshold = self.frame_time_threshold();
            if threshold < 0 {
                threshold = 0;
            }
            //log!("frame_time_threshold: {}", threshold);
            if (timestamp - self.timestamp_last_frame) as i32 > threshold {
                self.timestamp_last_frame = timestamp;
                return true;
            }
        }
        false
    }

    fn frame_time_threshold(&self) -> i32 {
        if self.speed_increase {
            FRAME_RATE_SPEED_1 - 8 * SPEED_INCREASE_MS
        } else {
            FRAME_RATE_SPEED_1 - self.level * SPEED_INCREASE_MS
        }
    }

    fn calc_duration(&self, timestamp: u32) -> u32 {
        timestamp - self.timestamp_game_start - self.pause_duration_sum
    }

    fn game_over(&mut self, timestamp: u32) {
        log!("game over");
        self.over = true;
        self.draw.as_ref().unwrap().draw_game_over();
        let duration = self.calc_duration(timestamp);
        let latest_timestamp =
            highscore::add_score(&self.name, self.level, self.lines, self.score, duration);
        highscore::print_highscores(latest_timestamp);
    }

    fn toggle_pause(&mut self, timestamp: u32) {
        log!("toggling pause");
        if !self.pause {
            self.timestamp_pause_start = timestamp;
        } else {
            self.pause_duration_sum += timestamp - self.timestamp_pause_start;
            self.timestamp_pause_start = 0;
        }
        self.pause = !self.pause;
        self.input = DEFAULT_INPUT;
    }

    pub fn set_input(&mut self, input: char) {
        self.input = input;
        self.input_processed = false;
        self.speed_increase = false;
    }

    pub fn set_pressed(&mut self, pressed: bool) {
        self.pressed = pressed;
        if pressed {
            if self.input == 's' {
                self.speed_increase = true;
            }
        } else {
            self.speed_increase = false;
        }
    }

    fn process_input(&mut self, timestamp: u32) {
        match &self.input {
            'a' => self.movement = objects::Movement::LEFT,
            'd' => self.movement = objects::Movement::RIGHT,
            'q' => self.movement = objects::Movement::RotateLeft,
            'e' => self.movement = objects::Movement::RotateRight,
            'p' => {
                if !self.input_processed {
                    self.print_debug_info();
                    self.input_processed = true;
                }
            }
            ' ' => {
                self.toggle_pause(timestamp);
                self.input_processed = true;
                self.movement = objects::Movement::NONE;
            }
            _ => self.movement = objects::Movement::NONE,
        }
    }

    fn print_debug_info(&self) {
        log!("********************************* Debug Info *********************************");
        log!("current block:");
        for piece in self.current_block.get_pieces() {
            log!("  x: {}, y: {}", piece.x, piece.y);
        }
        log!("block stack:");
        for (_index, row) in self.block_stack.as_ref().unwrap().get_stack() {
            for stack_block in row {
                for piece in stack_block.get_pieces() {
                    log!("  x: {}, y: {}", piece.x, piece.y);
                }
            }
        }
        log!("******************************************************************************");
    }

    fn update_world(&mut self, first_frame: bool) -> bool {
        let block_ok = self.move_block_and_check_collision(first_frame);
        if !block_ok {
            let mut block_still_on_first_row = false;
            for point in self.current_block.get_pieces() {
                if point.y == 0 {
                    block_still_on_first_row = true;
                }
            }
            if block_still_on_first_row {
                return false;
            } else {
                let block_stack = self.block_stack.as_mut().unwrap();
                block_stack.add_block_to_stack(&self.current_block);
                let num_of_rows = block_stack.reduce_stack(GAME_WIDTH);
                if num_of_rows > 0 {
                    self.handle_rows_removed(num_of_rows);
                }
                self.current_block = self.next_block.clone();
                self.next_block = self.create_block();
            }
        }
        true
    }

    fn handle_rows_removed(&mut self, num_of_rows: usize) {
        self.lines = self.lines + num_of_rows as i32;
        let lines_threshold = self.level * ROWS_FOR_LEVEL_UP;
        if self.lines > lines_threshold - 1 {
            self.level_up();
        }
        match num_of_rows {
            1 => self.score = self.score + 1 * self.level,
            2 => self.score = self.score + 3 * self.level,
            3 => self.score = self.score + 4 * self.level,
            4 => self.score = self.score + 8 * self.level,
            _ => (),
        }
        update_text_display(self.score, self.lines, self.level);
    }

    fn level_up(&mut self) {
        self.level = self.level + 1;
        set_background_colour(colours::colours_for_level(self.level).colour_bg);
        // re-create next_block with new color
        let rand = blocks::rand_for_block(&self.next_block);
        self.next_block = blocks::new(rand, self.level);
    }

    fn move_block_and_check_collision(&mut self, first_frame: bool) -> bool {
        let block_stack = self.block_stack.as_ref().unwrap();
        if !first_frame {
            let mut all_pieces_ok = true;
            for point in self.current_block.get_pieces_mut() {
                if point.y >= GAME_HEIGHT - 1 {
                    all_pieces_ok = false;
                }
                if all_pieces_ok {
                    all_pieces_ok = block_stack.check_collision(&objects::Point {
                        x: point.x,
                        y: point.y + 1,
                    });
                }
            }
            if all_pieces_ok {
                for point in self.current_block.get_pieces_mut() {
                    point.y += 1;
                }
            } else {
                return false;
            }
        }

        if self.input_processed {
            return true;
        }
        self.input_processed = true;

        if self.movement == objects::Movement::LEFT {
            let mut all_pieces_ok = true;
            for point in self.current_block.get_pieces() {
                if point.x <= 0 {
                    all_pieces_ok = false;
                }
                if all_pieces_ok {
                    all_pieces_ok = block_stack.check_collision(&objects::Point {
                        x: point.x - 1,
                        y: point.y,
                    });
                }
            }
            if all_pieces_ok {
                for point in self.current_block.get_pieces_mut() {
                    point.x -= 1;
                }
            }
        }
        if self.movement == objects::Movement::RIGHT {
            let mut all_pieces_ok = true;
            for point in self.current_block.get_pieces() {
                if point.x > GAME_WIDTH - 2 {
                    all_pieces_ok = false;
                }
                if all_pieces_ok {
                    all_pieces_ok = block_stack.check_collision(&objects::Point {
                        x: point.x + 1,
                        y: point.y,
                    });
                }
            }
            if all_pieces_ok {
                for point in self.current_block.get_pieces_mut() {
                    point.x += 1;
                }
            }
        }
        if self.movement == objects::Movement::RotateLeft
            || self.movement == objects::Movement::RotateRight
        {
            let rotated = if self.movement == objects::Movement::RotateLeft {
                &self.current_block.get_rotated_left()
            } else {
                &self.current_block.get_rotated_right()
            };
            let mut all_pieces_ok = true;
            for point in rotated {
                if !(point.x > 0 && point.x < GAME_WIDTH && point.y > 0 && point.y < GAME_HEIGHT) {
                    all_pieces_ok = false;
                }
                if all_pieces_ok {
                    all_pieces_ok = block_stack.check_collision(point);
                }
            }
            if all_pieces_ok {
                self.current_block.apply_rotated(rotated);
            }
        }
        return true;
    }
}
