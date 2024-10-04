use crate::blocks::Block;
use crate::objects::DrawState;
use crate::objects::Point;

use wasm_bindgen::prelude::*;

pub const BLOCK_SIZE: i32 = 15;
const SUB_BLOCK_SIZE: i32 = 3;

#[derive(PartialEq)]
enum DrawMode {
    GAME,
    PREVIEW,
}

pub struct Draw {
    context: Option<web_sys::CanvasRenderingContext2d>,
    context_next: Option<web_sys::CanvasRenderingContext2d>,
    width: i32,
    height: i32,
}
impl Draw {
    pub const fn create(
        context: Option<web_sys::CanvasRenderingContext2d>,
        context_next: Option<web_sys::CanvasRenderingContext2d>,
        width: i32,
        height: i32,
    ) -> Self {
        Self {
            context: context,
            context_next: context_next,
            width: width * BLOCK_SIZE,
            height: height * BLOCK_SIZE,
        }
    }

    fn calc_coord(&self, point: &Point, mode: &DrawMode) -> Point {
        match mode {
            DrawMode::GAME => Point {
                x: point.x * BLOCK_SIZE,
                y: point.y * BLOCK_SIZE,
            },
            DrawMode::PREVIEW => Point {
                x: (point.x - 4) * BLOCK_SIZE,
                y: (point.y + 4) * BLOCK_SIZE,
            },
        }
    }

    pub fn draw_pause(&self) {
        let context: &web_sys::CanvasRenderingContext2d = self.context.as_ref().unwrap();
        context.set_font("bold 30px serif");
        context.set_text_align("center");
        context.set_fill_style(&JsValue::from_str("#00F"));
        let result = context.fill_text("PAUSE", (self.width / 2).into(), (self.height / 2).into());
        crate::utils::handle_js_error(result);
    }

    pub fn draw_game_over(&self) {
        let context: &web_sys::CanvasRenderingContext2d = self.context.as_ref().unwrap();
        context.set_font("bold 30px serif");
        context.set_text_align("center");
        context.set_fill_style(&JsValue::from_str("#F00"));
        let result = context.fill_text(
            "GAME OVER",
            (self.width / 2).into(),
            (self.height / 2).into(),
        );
        crate::utils::handle_js_error(result);
    }

    pub fn draw(&self, draw_state: DrawState) {
        let context = self.context.as_ref().unwrap();
        let context_next = self.context_next.as_ref().unwrap();
        self.draw_clear(context);
        self.draw_clear(context_next);
        self.draw_block(context, &DrawMode::GAME, draw_state.current_block);
        self.draw_block(context_next, &DrawMode::PREVIEW, draw_state.next_block);
        for (_index, row) in draw_state.block_stack.get_stack() {
            for block in row {
                self.draw_block(context, &DrawMode::GAME, block);
            }
        }
    }

    fn draw_clear(&self, context: &web_sys::CanvasRenderingContext2d) {
        context.set_fill_style(&JsValue::from_str("#FFF"));
        context.fill_rect(0.0, 0.0, self.width.into(), self.height.into());
    }

    fn draw_block(
        &self,
        context: &web_sys::CanvasRenderingContext2d,
        mode: &DrawMode,
        current_block: &Block,
    ) {
        for point in current_block.get_pieces() {
            self.draw_piece(
                context,
                mode,
                point,
                current_block.get_colour_1(),
                current_block.get_colour_2(),
            );
        }
    }

    fn draw_piece(
        &self,
        context: &web_sys::CanvasRenderingContext2d,
        mode: &DrawMode,
        point: &Point,
        colour_1: &str,
        colour_2: &str,
    ) {
        let coord = self.calc_coord(&point, mode);
        if colour_1 == colour_2 {
            context.set_fill_style(&JsValue::from_str(colour_1));
            context.fill_rect(
                coord.x.into(),
                coord.y.into(),
                BLOCK_SIZE.into(),
                BLOCK_SIZE.into(),
            );
        } else {
            // draw whole block with colour 2 as background
            context.set_fill_style(&JsValue::from_str(colour_2));
            context.fill_rect(
                coord.x.into(),
                coord.y.into(),
                BLOCK_SIZE.into(),
                BLOCK_SIZE.into(),
            );

            // draw with colour 1 as foreground
            context.set_fill_style(&JsValue::from_str(colour_1));
            // upper row
            context.fill_rect(
                coord.x.into(),
                coord.y.into(),
                BLOCK_SIZE.into(),
                SUB_BLOCK_SIZE.into(),
            );
            // lower row
            context.fill_rect(
                (coord.x).into(),
                (coord.y + BLOCK_SIZE - SUB_BLOCK_SIZE).into(),
                BLOCK_SIZE.into(),
                SUB_BLOCK_SIZE.into(),
            );
            // left column
            context.fill_rect(
                coord.x.into(),
                coord.y.into(),
                SUB_BLOCK_SIZE.into(),
                BLOCK_SIZE.into(),
            );
            // right column
            context.fill_rect(
                (coord.x + BLOCK_SIZE - SUB_BLOCK_SIZE).into(),
                (coord.y).into(),
                SUB_BLOCK_SIZE.into(),
                BLOCK_SIZE.into(),
            );
            // middle
            context.fill_rect(
                (coord.x + 2 * SUB_BLOCK_SIZE).into(),
                (coord.y + 2 * SUB_BLOCK_SIZE).into(),
                SUB_BLOCK_SIZE.into(),
                SUB_BLOCK_SIZE.into(),
            );
        }
    }
}
