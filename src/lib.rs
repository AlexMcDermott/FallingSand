use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

extern crate console_error_panic_hook;

#[wasm_bindgen(start)]
pub fn start() {
  console_error_panic_hook::set_once();
}

#[derive(Clone, Copy, PartialEq)]
enum CellKind {
  Empty,
  Sand,
}

#[derive(Clone, Copy, PartialEq)]
struct Cell {
  kind: CellKind,
  colour: &'static str,
}

#[wasm_bindgen]
pub struct Grid {
  width: u32,
  height: u32,
  state: Vec<Cell>,
}

static EMPTY_CELL: Cell = Cell {
  kind: CellKind::Empty,
  colour: &"FFF4E4",
};

static SAND_CELL: Cell = Cell {
  kind: CellKind::Sand,
  colour: &"F3C98B",
};

impl Cell {
  fn update_sand(&mut self, grid: &mut Grid, index: usize) {
    let target_index = grid.get_index_below(index);
    if grid.index_in_bounds(target_index) && grid.index_available(target_index) {
      grid.swap_cells(index, target_index)
    }
  }

  fn update(&mut self, grid: &mut Grid, index: usize) {
    match self.kind {
      CellKind::Sand => self.update_sand(grid, index),
      _ => (),
    }
  }
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

  fn get_index_below(&self, index: usize) -> usize {
    return index + self.width as usize;
  }

  fn index_in_bounds(&self, index: usize) -> bool {
    return index < (self.width * self.height) as usize;
  }

  fn index_available(&self, index: usize) -> bool {
    return self.state[index] == EMPTY_CELL;
  }

  fn swap_cells(&mut self, i1: usize, i2: usize) {
    let temp = self.state[i1];
    self.state[i1] = self.state[i2];
    self.state[i2] = temp;
  }

  pub fn set_cell(&mut self, x: u32, y: u32) {
    self.state[(y * self.width + x) as usize] = SAND_CELL;
  }

  pub fn tick(&mut self) {
    for i in (0..self.state.len()).rev() {
      let mut cell: Cell = self.state[i];
      cell.update(self, i);
    }
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
