use std::ops::Sub;
use wasm_bindgen::prelude::*;
use web_sys::{console, CanvasRenderingContext2d};

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
  outdated: bool,
}

struct Coordinates {
  x: u32,
  y: u32,
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
  outdated: true,
};

static SAND_CELL: Cell = Cell {
  kind: CellKind::Sand,
  colour: &"F3C98B",
  outdated: true,
};

impl Cell {
  fn update_sand(&mut self, grid: &mut Grid, current_index: usize) {
    let below_index = grid.get_index_below(current_index);
    if grid.index_available(current_index, below_index) {
      grid.swap_cells(current_index, below_index)
    }
  }

  fn update(&mut self, grid: &mut Grid, index: usize) {
    match self.kind {
      CellKind::Sand => self.update_sand(grid, index),
      _ => (),
    }
  }
}

impl Sub for Coordinates {
  type Output = Self;

  fn sub(self, other: Self) -> Self::Output {
    Self {
      x: (self.x as i32 - other.x as i32).abs() as u32,
      y: (self.y as i32 - other.y as i32).abs() as u32,
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

  fn coords_from_index(&self, index: usize) -> Coordinates {
    let x: u32 = index as u32 % self.width;
    let y: u32 = index as u32 / self.width;
    return Coordinates { x: x, y: y };
  }

  fn get_index_below(&self, index: usize) -> usize {
    return index + self.width as usize;
  }

  fn index_in_bounds(&self, index: usize) -> bool {
    return index < (self.width * self.height) as usize;
  }

  fn index_is_empty(&self, index: usize) -> bool {
    return self.state[index] == EMPTY_CELL;
  }

  fn index_is_nearby(&self, current_index: usize, test_index: usize) -> bool {
    let current_coords: Coordinates = self.coords_from_index(current_index);
    let test_coords: Coordinates = self.coords_from_index(test_index);
    let delta: Coordinates = current_coords - test_coords;
    return delta.x <= 1 && delta.y <= 1;
  }

  fn index_available(&self, current_index: usize, test_index: usize) -> bool {
    return self.index_in_bounds(test_index)
      && self.index_is_empty(test_index)
      && self.index_is_nearby(current_index, test_index);
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
      let coords: Coordinates = self.coords_from_index(i as usize);
      let cell: Cell = self.state[i as usize];
      context.set_fill_style(&cell.colour.into());
      context.fill_rect(
        (coords.x * cell_size) as f64,
        (coords.y * cell_size) as f64,
        cell_size as f64,
        cell_size as f64,
      );
    }
  }
}
