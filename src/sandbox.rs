
use raylib::{color::Color, prelude::{RaylibDraw, RaylibDrawHandle}};

use crate::cell::Cell;

pub const SANDBOX_WIDTH: usize = 200;
pub const SANDBOX_HEIGHT: usize = 150;
pub const SANDBOX_SIZE: usize = SANDBOX_WIDTH * SANDBOX_HEIGHT;
pub const CELL_SIZE: usize = 4;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Material {
  OOB,
  AIR,
  SAND,
  WATER,
  STONE,
}



pub struct Sandbox {
  cells: Vec<Cell>,
  paused: bool
}

impl Sandbox {
  pub fn new() -> Self {
    Self {
      cells: vec![Cell{material: Material::AIR, temperature: 0}; SANDBOX_SIZE],
      paused: false
    }
  }

  fn pos(x: usize, y: usize) -> usize {
    y * SANDBOX_WIDTH + x
  }

  pub fn get(&self, x: usize, y: usize) -> Option<Cell> {
    let pos = Self::pos(x, y);
    if pos > SANDBOX_SIZE {
      None
    } else {
      Some(self.cells[pos])
    }
  }

  fn get_unchecked(&self, x: usize, y: usize) -> Cell {
    self.cells[Self::pos(x, y)]
  }

  pub fn set(&mut self, x: usize, y: usize, cel: Cell) {
    let pos = Self::pos(x, y);
    if pos > SANDBOX_SIZE {
      return;
    } else {
      self.cells[pos] = cel;
    }
  }

  pub fn get_buffer(&self) -> &Vec<Cell> {
    &self.cells
  }

  // === cell helpers
  
  fn get_cell_under(&self, x: usize, y: usize) -> Cell {
    self.get(x, y + 1).unwrap_or(Cell::oob())
  }

  fn get_cell_left(&self, x: usize, y: usize) -> Cell {
    if x == 0 {
      return Cell::oob();
    }
    self.get(x - 1, y + 1).unwrap_or(Cell::oob())
  }

  fn get_cell_right(&self, x: usize, y: usize) -> Cell {
    if x >= SANDBOX_WIDTH - 1 {
      return Cell::oob();
    }
    self.get(x + 1, y + 1).unwrap_or(Cell::oob())
  }


  // === logic

  pub fn tick(&mut self) {
    if self.paused {
      return;
    }
    
    for y in (0..SANDBOX_HEIGHT - 1).rev() {
      for x in 0..SANDBOX_WIDTH  {
        self.update_cell(x, y);
      }
    }
    //self.swap();
  }

  fn update_cell(&mut self, x: usize, y: usize) {
    let cel = self.get_unchecked(x, y);
    match cel.material {
      Material::SAND => self._update_powder(x, y),
      _ => {}
    }
  }

  pub fn toggle_pause(&mut self) {
    self.paused = !self.paused;
  }

  // logic by-material

  fn _update_powder(&mut self, x: usize, y: usize) {
    if self.get_cell_under(x, y).material == Material::AIR {
      self.set(x, y, Cell::air());
      self.set(x, y + 1, Cell::sand());
    } else if self.get_cell_left(x, y).material == Material::AIR {
      self.set(x, y, Cell::air());
      self.set(x - 1, y + 1, Cell::sand());
    } else if self.get_cell_right(x, y).material == Material::AIR {
      self.set(x, y, Cell::air());
      self.set(x + 1, y + 1, Cell::sand());
    }
  }
}

pub fn render_sandbox(cells: &Vec<Cell>, handle: &mut RaylibDrawHandle) {
  for y in 0..SANDBOX_HEIGHT {
    for x in 0..SANDBOX_WIDTH {
      let pos = y * SANDBOX_WIDTH + x;
      let mat = cells[pos];

      let color: Color = match mat.material {
        Material::SAND => Color::YELLOW,
        _ => Color::BLACK
      };

      if color == Color::BLACK {
        continue;
      }

      // actually draw it!
      handle.draw_rectangle((x * CELL_SIZE) as i32, (y * CELL_SIZE) as i32, (CELL_SIZE) as i32, (CELL_SIZE) as i32, color);
    }
  }
}

pub fn window_to_world(x: i32, y: i32) -> Option<(usize, usize)> {
  let sx = x / CELL_SIZE as i32;
  let sy = y / CELL_SIZE as i32;

  if sx > SANDBOX_WIDTH as i32 || sx < 0{
    return None;
  }

  if sy > SANDBOX_HEIGHT as i32 || sy < 0 {
    return None;
  }

  Some((sx as usize, sy as usize))
}