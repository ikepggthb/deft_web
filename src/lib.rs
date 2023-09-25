use wasm_bindgen::prelude::*;

// wasm console.log()を実行
// https://rustwasm.github.io/wasm-bindgen/examples/console-log.html

// First up let's take a look of binding `console.log` manually, without the
// help of `web_sys`. Here we're writing the `#[wasm_bindgen]` annotations
// manually ourselves, and the correctness of our program relies on the
// correctness of these annotations!

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

// Next let's define a macro that's like `println!`, only it works for
// `console.log`. Note that `println!` doesn't actually work on the wasm target
// because the standard library currently just eats all output. To get
// `println!`-like behavior in your app you'll likely want a macro like this.

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    console_log!("{}", a+b);
    a + b
}

use deft_reversi_engine::*;
use serde::{Serialize, Deserialize};

// Rust から Javascript に、structを渡す。
//https://rustwasm.github.io/docs/wasm-bindgen/reference/arbitrary-data-with-serde.html
// https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html#serializing-and-deserializing-arbitrary-data-into-and-from-jsvalue-with-serde
#[derive(Serialize, Deserialize)]
struct JsBoard {
    board: Vec<Vec<i32>>,
    next_turn: i32
}

#[wasm_bindgen]
pub struct App {
    bm: BoardManager

}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            bm: BoardManager::new()
        }
    }

    #[wasm_bindgen]
    pub fn get_board(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        let b = self.bm.current_board();
        let put_able = b.put_able();
        let mut js_b = JsBoard{
            board: vec![vec![0;8]; 8],
            next_turn: 0
        };
        for y in 0..8 {
            for x in 0..8 {
                let mask = 1u64 << y * 8 + x;
                if mask & b.bit_board[Board::BLACK] != 0 {
                    js_b.board[y][x] = 1;
                } else if mask & b.bit_board[Board::WHITE] != 0{
                    js_b.board[y][x] = 2;
                } else if put_able & mask != 0 {
                    js_b.board[y][x] = 3;
                }
            }
        }

        //https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html#serializing-and-deserializing-arbitrary-data-into-and-from-jsvalue-with-serde
        serde_wasm_bindgen::to_value(&js_b)
    }

    #[wasm_bindgen]
    pub fn put(&mut self, y: i32, x: i32) {
        let mut b = self.bm.current_board();
        let re = b.put_piece_from_coord(y, x);
        if re.is_ok() {
            self.bm.add(b);
        }
    }

}