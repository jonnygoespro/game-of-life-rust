mod utils;
mod board;

use wasm_bindgen::prelude::*;
use board::Board;
use js_sys::Array;

#[wasm_bindgen]
pub fn life(iteration: i32) -> Result<JsValue, JsValue> {
    let mut game_states: Vec<Vec<Vec<u8>>> = Vec::new();

    let mut board = Board::new(20);
    game_states.push(board.get_state());
    for _ in 0..iteration {
        board.play_round();
        game_states.push(board.get_state());
    }

    // Create a JavaScript Array to store game states
    let game_states_array = Array::new();

    for state in game_states {
        let state_array = Array::new();
        for row in state {
            let row_array = Array::new();
            for cell in row {
                row_array.push(&JsValue::from(cell));
            }
            state_array.push(&JsValue::from(row_array));
        }
        game_states_array.push(&JsValue::from(state_array));
    }

    Ok(JsValue::from(game_states_array))
}