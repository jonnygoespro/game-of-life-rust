mod utils;
mod board;

use wasm_bindgen::prelude::*;
use board::Board;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, console};
use crate::utils::log_vec;

#[wasm_bindgen]
pub fn life(iteration: i32, board_size: usize) -> Result<(), JsValue> {
    let window = window().expect("Cannot access global 'window'");
    let document = window.document().expect("Cannot access document");
    let canvas = document.get_element_by_id("canvas").expect("No canvas found");

    let canvas = canvas.dyn_into::<HtmlCanvasElement>()?;
    let context = canvas.get_context("2d")
        .expect("No context").expect("No context")
        .dyn_into::<CanvasRenderingContext2d>()?;

    let cell_size = canvas.width() / board_size as u32;

    let mut board = Board::new(board_size);
    for _ in 0..iteration {
        board.play_round();
    }

    let latest_state = board.get_state();
    // log_vec(&latest_state);

    for (y, row) in latest_state.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let color = match cell {
                0 => "#44035c",
                1 => "#ffffff",
                _ => "#7684b9"
            };

            context.set_fill_style(&JsValue::from_str(color));
            context.fill_rect((x as u32 * cell_size) as f64, (y as u32 * cell_size) as f64, cell_size as f64, cell_size as f64);
        }
    }

    Ok(())
}