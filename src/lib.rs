use wasm_bindgen::prelude::*;
use web_sys::{console, CanvasRenderingContext2d};

extern crate console_error_panic_hook;

#[wasm_bindgen(start)]
pub fn start() {
  console_error_panic_hook::set_once();
}

enum Cell {
  Empty { colour: &'static str },
  Sand { colour: &'static str },
}

static EMPTY_CELL: Cell = Cell::Empty { colour: &"FFF4E4" };

#[wasm_bindgen]
pub struct Grid {
  width: u32,
  height: u32,
  state: Vec<u8>,
}

#[wasm_bindgen]
impl Grid {
  #[wasm_bindgen(constructor)]
  pub fn new(width: u32, height: u32) -> Self {
    return Grid {
      width,
      height,
      state: vec![0; (width * height) as usize],
    };
  }

  pub fn set(&mut self, x: u32, y: u32, value: u8) {
    self.state[(y * self.width + x) as usize] = value
  }

  pub fn render(&self, context: CanvasRenderingContext2d, cell_size: u32) {
    for i in 0..self.state.len() as u32 {
      let x: u32 = i % self.width;
      let y: u32 = i / self.width;
      match self.state[i as usize] {
        1 => context.set_fill_style(&"#fff".into()),
        _ => context.set_fill_style(&"#000".into()),
      }
      context.fill_rect(
        (x * cell_size) as f64,
        (y * cell_size) as f64,
        cell_size as f64,
        cell_size as f64,
      );
    }
  }
}
