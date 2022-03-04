use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

extern crate console_error_panic_hook;

#[wasm_bindgen(start)]
pub fn start() {
  console_error_panic_hook::set_once();
}

#[derive(Clone, Copy)]
enum CellKind {
  Empty,
  Sand,
}

#[derive(Clone, Copy)]
struct Cell {
  kind: CellKind,
  colour: &'static str,
}

static EMPTY_CELL: Cell = Cell {
  kind: CellKind::Empty,
  colour: &"FFF4E4",
};

static SAND_CELL: Cell = Cell {
  kind: CellKind::Sand,
  colour: &"F3C98B",
};

#[wasm_bindgen]
pub struct Grid {
  width: u32,
  height: u32,
  state: Vec<Cell>,
}

#[wasm_bindgen]
impl Grid {
  #[wasm_bindgen(constructor)]
  pub fn new(width: u32, height: u32) -> Self {
    return Grid {
      width,
      height,
      state: vec![EMPTY_CELL; (width * height) as usize],
    };
  }

  pub fn set(&mut self, x: u32, y: u32) {
    self.state[(y * self.width + x) as usize] = SAND_CELL;
  }

  pub fn render(&self, context: CanvasRenderingContext2d, cell_size: u32) {
    for i in 0..self.state.len() as u32 {
      let x: u32 = i % self.width;
      let y: u32 = i / self.width;
      let cell: Cell = self.state[i as usize];
      context.set_fill_style(&cell.colour.into());
      context.fill_rect(
        (x * cell_size) as f64,
        (y * cell_size) as f64,
        cell_size as f64,
        cell_size as f64,
      );
    }
  }
}
