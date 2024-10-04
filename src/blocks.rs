use crate::colours;
use crate::game;
use crate::objects::Point;

#[derive(Clone)]
pub enum BlockType {
    DefaultBlock,
    StackBlock,
    Long,
    Tee,
    Quad,
    StepLeft,
    StepRight,
    LLeft,
    LRight,
}

#[derive(Clone)]
pub struct Block {
    pieces: Vec<Point>,
    colour_1: &'static str,
    colour_2: &'static str,
    block_type: BlockType,
}

impl Block {
    pub fn get_pieces(&self) -> &Vec<Point> {
        &self.pieces
    }

    pub fn get_pieces_mut(&mut self) -> &mut Vec<Point> {
        &mut self.pieces
    }

    pub fn get_colour_1(&self) -> &'static str {
        self.colour_1
    }

    pub fn get_colour_2(&self) -> &'static str {
        self.colour_2
    }

    pub fn apply_rotated(&mut self, rotated: &Vec<Point>) {
        self.pieces.clear();
        for point in rotated {
            self.pieces.push(*point)
        }
    }

    pub fn get_rotated_left(&self) -> Vec<Point> {
        match &self.block_type {
            BlockType::DefaultBlock => self.pieces.clone(),
            BlockType::StackBlock => self.pieces.clone(),
            BlockType::Long => self.get_rotated_left_long(),
            BlockType::Tee => self.get_rotated_left_tee(),
            BlockType::Quad => self.get_rotated_left_quad(),
            BlockType::StepLeft => self.get_rotated_left_step_left(),
            BlockType::StepRight => self.get_rotated_left_step_right(),
            BlockType::LLeft => self.get_rotated_left_l_left(),
            BlockType::LRight => self.get_rotated_left_l_right(),
        }
    }
    pub fn get_rotated_right(&self) -> Vec<Point> {
        match &self.block_type {
            BlockType::DefaultBlock => self.pieces.clone(),
            BlockType::StackBlock => self.pieces.clone(),
            BlockType::Long => self.get_rotated_right_long(),
            BlockType::Tee => self.get_rotated_right_tee(),
            BlockType::Quad => self.get_rotated_right_quad(),
            BlockType::StepLeft => self.get_rotated_right_step_left(),
            BlockType::StepRight => self.get_rotated_right_step_right(),
            BlockType::LLeft => self.get_rotated_right_l_left(),
            BlockType::LRight => self.get_rotated_right_l_right(),
        }
    }

    fn get_rotated_left_long(&self) -> Vec<Point> {
        let mut rotated = Vec::new();
        let rotation_point = self.pieces[1];
        let horizontal = self.pieces[0].x == rotation_point.x;
        if horizontal {
            rotated.push(Point {
                x: rotation_point.x - 1,
                y: rotation_point.y,
            });
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y,
            });
            rotated.push(Point {
                x: rotation_point.x + 1,
                y: rotation_point.y,
            });
            rotated.push(Point {
                x: rotation_point.x + 2,
                y: rotation_point.y,
            });
        } else {
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y - 1,
            });
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y,
            });
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y + 1,
            });
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y + 2,
            });
        }
        rotated
    }
    fn get_rotated_right_long(&self) -> Vec<Point> {
        self.get_rotated_left_long()
    }
    fn get_rotated_left_tee(&self) -> Vec<Point> {
        let mut rotated = Vec::new();
        let rotation_point = self.pieces[1];
        let extension = self.pieces[3];
        let horizontal = self.pieces[0].y == rotation_point.y;
        if horizontal {
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y - 1,
            });
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y,
            });
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y + 1,
            });
            if extension.y > rotation_point.y {
                rotated.push(Point {
                    x: rotation_point.x + 1,
                    y: rotation_point.y,
                });
            } else {
                rotated.push(Point {
                    x: rotation_point.x - 1,
                    y: rotation_point.y,
                });
            }
        } else {
            rotated.push(Point {
                x: rotation_point.x - 1,
                y: rotation_point.y,
            });
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y,
            });
            rotated.push(Point {
                x: rotation_point.x + 1,
                y: rotation_point.y,
            });
            if extension.x > rotation_point.x {
                rotated.push(Point {
                    x: rotation_point.x,
                    y: rotation_point.y - 1,
                });
            } else {
                rotated.push(Point {
                    x: rotation_point.x,
                    y: rotation_point.y + 1,
                });
            }
        }
        rotated
    }
    fn get_rotated_right_tee(&self) -> Vec<Point> {
        let mut rotated = Vec::new();
        let rotation_point = self.pieces[1];
        let extension = self.pieces[3];
        let horizontal = self.pieces[0].y == rotation_point.y;
        if horizontal {
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y - 1,
            });
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y,
            });
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y + 1,
            });
            if extension.y > rotation_point.y {
                rotated.push(Point {
                    x: rotation_point.x - 1,
                    y: rotation_point.y,
                });
            } else {
                rotated.push(Point {
                    x: rotation_point.x + 1,
                    y: rotation_point.y,
                });
            }
        } else {
            rotated.push(Point {
                x: rotation_point.x - 1,
                y: rotation_point.y,
            });
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y,
            });
            rotated.push(Point {
                x: rotation_point.x + 1,
                y: rotation_point.y,
            });
            if extension.x > rotation_point.x {
                rotated.push(Point {
                    x: rotation_point.x,
                    y: rotation_point.y + 1,
                });
            } else {
                rotated.push(Point {
                    x: rotation_point.x,
                    y: rotation_point.y - 1,
                });
            }
        }
        rotated
    }
    fn get_rotated_left_quad(&self) -> Vec<Point> {
        self.pieces.clone()
    }
    fn get_rotated_right_quad(&self) -> Vec<Point> {
        self.get_rotated_left_quad()
    }
    fn get_rotated_left_step_left(&self) -> Vec<Point> {
        let mut rotated = Vec::new();
        let rotation_point = self.pieces[1];
        let horizontal = self.pieces[0].y == rotation_point.y;
        if horizontal {
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y + 1,
            });
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y,
            });
            rotated.push(Point {
                x: rotation_point.x + 1,
                y: rotation_point.y,
            });
            rotated.push(Point {
                x: rotation_point.x + 1,
                y: rotation_point.y - 1,
            });
        } else {
            rotated.push(Point {
                x: rotation_point.x + 1,
                y: rotation_point.y,
            });
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y,
            });
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y - 1,
            });
            rotated.push(Point {
                x: rotation_point.x - 1,
                y: rotation_point.y - 1,
            });
        }
        rotated
    }
    fn get_rotated_right_step_left(&self) -> Vec<Point> {
        self.get_rotated_left_step_left()
    }
    fn get_rotated_left_step_right(&self) -> Vec<Point> {
        let mut rotated = Vec::new();
        let rotation_point = self.pieces[1];
        let horizontal = self.pieces[0].y == rotation_point.y;
        if horizontal {
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y + 1,
            });
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y,
            });
            rotated.push(Point {
                x: rotation_point.x - 1,
                y: rotation_point.y,
            });
            rotated.push(Point {
                x: rotation_point.x - 1,
                y: rotation_point.y - 1,
            });
        } else {
            rotated.push(Point {
                x: rotation_point.x + 1,
                y: rotation_point.y,
            });
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y,
            });
            rotated.push(Point {
                x: rotation_point.x + 1,
                y: rotation_point.y - 1,
            });
            rotated.push(Point {
                x: rotation_point.x + 2,
                y: rotation_point.y - 1,
            });
        }
        rotated
    }
    fn get_rotated_right_step_right(&self) -> Vec<Point> {
        self.get_rotated_left_step_right()
    }
    fn get_rotated_left_l_left(&self) -> Vec<Point> {
        let mut rotated = Vec::new();
        let rotation_point = self.pieces[1];
        let extension = self.pieces[3];
        let horizontal = self.pieces[0].y == rotation_point.y;
        if horizontal {
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y - 1,
            });
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y,
            });
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y + 1,
            });
            if extension.y > rotation_point.y {
                rotated.push(Point {
                    x: rotation_point.x + 1,
                    y: rotation_point.y + 1,
                });
            } else {
                rotated.push(Point {
                    x: rotation_point.x - 1,
                    y: rotation_point.y - 1,
                });
            }
        } else {
            rotated.push(Point {
                x: rotation_point.x - 1,
                y: rotation_point.y,
            });
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y,
            });
            rotated.push(Point {
                x: rotation_point.x + 1,
                y: rotation_point.y,
            });
            if extension.x > rotation_point.x {
                rotated.push(Point {
                    x: rotation_point.x + 1,
                    y: rotation_point.y - 1,
                });
            } else {
                rotated.push(Point {
                    x: rotation_point.x - 1,
                    y: rotation_point.y + 1,
                });
            }
        }
        rotated
    }
    fn get_rotated_right_l_left(&self) -> Vec<Point> {
        let mut rotated = Vec::new();
        let rotation_point = self.pieces[1];
        let extension = self.pieces[3];
        let horizontal = self.pieces[0].y == rotation_point.y;
        if horizontal {
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y - 1,
            });
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y,
            });
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y + 1,
            });
            if extension.y > rotation_point.y {
                rotated.push(Point {
                    x: rotation_point.x - 1,
                    y: rotation_point.y - 1,
                });
            } else {
                rotated.push(Point {
                    x: rotation_point.x + 1,
                    y: rotation_point.y + 1,
                });
            }
        } else {
            rotated.push(Point {
                x: rotation_point.x - 1,
                y: rotation_point.y,
            });
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y,
            });
            rotated.push(Point {
                x: rotation_point.x + 1,
                y: rotation_point.y,
            });
            if extension.x > rotation_point.x {
                rotated.push(Point {
                    x: rotation_point.x - 1,
                    y: rotation_point.y + 1,
                });
            } else {
                rotated.push(Point {
                    x: rotation_point.x + 1,
                    y: rotation_point.y - 1,
                });
            }
        }
        rotated
    }
    fn get_rotated_left_l_right(&self) -> Vec<Point> {
        let mut rotated = Vec::new();
        let rotation_point = self.pieces[1];
        let extension = self.pieces[3];
        let horizontal = self.pieces[0].y == rotation_point.y;
        if horizontal {
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y - 1,
            });
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y,
            });
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y + 1,
            });
            if extension.y > rotation_point.y {
                rotated.push(Point {
                    x: rotation_point.x + 1,
                    y: rotation_point.y - 1,
                });
            } else {
                rotated.push(Point {
                    x: rotation_point.x - 1,
                    y: rotation_point.y + 1,
                });
            }
        } else {
            rotated.push(Point {
                x: rotation_point.x - 1,
                y: rotation_point.y,
            });
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y,
            });
            rotated.push(Point {
                x: rotation_point.x + 1,
                y: rotation_point.y,
            });
            if extension.x > rotation_point.x {
                rotated.push(Point {
                    x: rotation_point.x - 1,
                    y: rotation_point.y - 1,
                });
            } else {
                rotated.push(Point {
                    x: rotation_point.x + 1,
                    y: rotation_point.y + 1,
                });
            }
        }
        rotated
    }
    fn get_rotated_right_l_right(&self) -> Vec<Point> {
        let mut rotated = Vec::new();
        let rotation_point = self.pieces[1];
        let extension = self.pieces[3];
        let horizontal = self.pieces[0].y == rotation_point.y;
        if horizontal {
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y - 1,
            });
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y,
            });
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y + 1,
            });
            if extension.y > rotation_point.y {
                rotated.push(Point {
                    x: rotation_point.x - 1,
                    y: rotation_point.y + 1,
                });
            } else {
                rotated.push(Point {
                    x: rotation_point.x + 1,
                    y: rotation_point.y - 1,
                });
            }
        } else {
            rotated.push(Point {
                x: rotation_point.x - 1,
                y: rotation_point.y,
            });
            rotated.push(Point {
                x: rotation_point.x,
                y: rotation_point.y,
            });
            rotated.push(Point {
                x: rotation_point.x + 1,
                y: rotation_point.y,
            });
            if extension.x > rotation_point.x {
                rotated.push(Point {
                    x: rotation_point.x + 1,
                    y: rotation_point.y + 1,
                });
            } else {
                rotated.push(Point {
                    x: rotation_point.x - 1,
                    y: rotation_point.y - 1,
                });
            }
        }
        rotated
    }
}

pub const fn default_block() -> Block {
    Block {
        pieces: Vec::new(),
        colour_1: "#000;",
        colour_2: "#000;",
        block_type: BlockType::DefaultBlock,
    }
}

pub fn stack_blocks(block: &Block) -> Vec<Block> {
    let mut blocks = Vec::new();
    for point in block.get_pieces() {
        blocks.push(Block {
            pieces: vec![Point {
                x: point.x,
                y: point.y,
            }],
            colour_1: block.get_colour_1(),
            colour_2: block.get_colour_2(),
            block_type: BlockType::StackBlock,
        });
    }
    blocks
}

const HAVE_WIDTH: i32 = game::GAME_WIDTH / 2;

pub fn rand_for_block(block: &Block) -> i32 {
    match block.block_type {
        BlockType::Long => 0,
        BlockType::Tee => 1,
        BlockType::Quad => 2,
        BlockType::StepLeft => 3,
        BlockType::StepRight => 4,
        BlockType::LLeft => 5,
        BlockType::LRight => 6,
        BlockType::DefaultBlock => -1,
        BlockType::StackBlock => -1,
    }
}

pub fn new(rand: i32, level: i32) -> Block {
    let colours = colours::colours_for_level(level);
    match rand {
        0 => Block {
            pieces: vec![
                Point {
                    x: HAVE_WIDTH - 1,
                    y: 0,
                },
                Point {
                    x: HAVE_WIDTH,
                    y: 0,
                },
                Point {
                    x: HAVE_WIDTH + 1,
                    y: 0,
                },
                Point {
                    x: HAVE_WIDTH + 2,
                    y: 0,
                },
            ],
            colour_1: colours.colour_1,
            colour_2: colours.colour_2,
            block_type: BlockType::Long,
        },
        1 => Block {
            pieces: vec![
                Point {
                    x: HAVE_WIDTH - 1,
                    y: 0,
                },
                Point {
                    x: HAVE_WIDTH,
                    y: 0,
                },
                Point {
                    x: HAVE_WIDTH + 1,
                    y: 0,
                },
                Point {
                    x: HAVE_WIDTH,
                    y: 1,
                },
            ],
            colour_1: colours.colour_1,
            colour_2: colours.colour_2,
            block_type: BlockType::Tee,
        },
        2 => Block {
            pieces: vec![
                Point {
                    x: HAVE_WIDTH - 1,
                    y: 0,
                },
                Point {
                    x: HAVE_WIDTH,
                    y: 0,
                },
                Point {
                    x: HAVE_WIDTH - 1,
                    y: 1,
                },
                Point {
                    x: HAVE_WIDTH,
                    y: 1,
                },
            ],
            colour_1: colours.colour_2,
            colour_2: colours.colour_1,
            block_type: BlockType::Quad,
        },
        3 => Block {
            pieces: vec![
                Point {
                    x: HAVE_WIDTH - 1,
                    y: 0,
                },
                Point {
                    x: HAVE_WIDTH,
                    y: 0,
                },
                Point {
                    x: HAVE_WIDTH,
                    y: 1,
                },
                Point {
                    x: HAVE_WIDTH + 1,
                    y: 1,
                },
            ],
            colour_1: colours.colour_1,
            colour_2: colours.colour_1,
            block_type: BlockType::StepLeft,
        },
        4 => Block {
            pieces: vec![
                Point {
                    x: HAVE_WIDTH - 1,
                    y: 1,
                },
                Point {
                    x: HAVE_WIDTH,
                    y: 1,
                },
                Point {
                    x: HAVE_WIDTH,
                    y: 0,
                },
                Point {
                    x: HAVE_WIDTH + 1,
                    y: 0,
                },
            ],
            colour_1: colours.colour_2,
            colour_2: colours.colour_2,
            block_type: BlockType::StepRight,
        },
        5 => Block {
            pieces: vec![
                Point {
                    x: HAVE_WIDTH - 1,
                    y: 0,
                },
                Point {
                    x: HAVE_WIDTH,
                    y: 0,
                },
                Point {
                    x: HAVE_WIDTH + 1,
                    y: 0,
                },
                Point {
                    x: HAVE_WIDTH - 1,
                    y: 1,
                },
            ],
            colour_1: colours.colour_1,
            colour_2: colours.colour_1,
            block_type: BlockType::LLeft,
        },
        _ => Block {
            pieces: vec![
                Point {
                    x: HAVE_WIDTH - 1,
                    y: 0,
                },
                Point {
                    x: HAVE_WIDTH,
                    y: 0,
                },
                Point {
                    x: HAVE_WIDTH + 1,
                    y: 0,
                },
                Point {
                    x: HAVE_WIDTH + 1,
                    y: 1,
                },
            ],
            colour_1: colours.colour_2,
            colour_2: colours.colour_2,
            block_type: BlockType::LRight,
        },
    }
}
