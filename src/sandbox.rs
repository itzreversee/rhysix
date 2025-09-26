
use rand::prelude::SliceRandom;
use raylib::{color::Color, math::Vector2, prelude::{RaylibDraw, RaylibDrawHandle}};

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
  paused: bool,
  hand_cell: Cell,
  hand_size: usize
}

impl Sandbox {
  pub fn new() -> Self {
    Self {
      cells: vec![Cell::air(); SANDBOX_SIZE],
      paused: false,
      hand_cell: Cell::sand(),
      hand_size: 4
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

  fn set(&mut self, x: usize, y: usize, cel: Cell) {
    let pos = Self::pos(x, y);
    if pos > SANDBOX_SIZE {
      return;
    } else {
      self.cells[pos] = cel;
    }
  }

  pub fn place(&mut self, x: usize, y: usize, ovr: Option<Cell>) {
    for oy in 0..self.hand_size {
      for ox in 0..self.hand_size {
        self.set(ox + x, oy + y, ovr.unwrap_or(self.hand_cell));
      }
    }
  }

  pub fn set_hand_cell(&mut self, cell: Cell) {
    self.hand_cell = cell;
  }
  pub fn get_hand_cell(&mut self) -> Cell {
    self.hand_cell
  }

  pub fn inc_size(&mut self) {
    self.hand_size += 1;
    if self.hand_size > 10 {
      self.hand_size = 10;
    }
  }
  pub fn dec_size(&mut self) {
    if self.hand_size == 1 {
      self.hand_size = 1;
      return;
    }
    self.hand_size -= 1;
  }
  pub fn get_size(&mut self) -> usize{
    self.hand_size
  }

  pub fn get_buffer(&self) -> &Vec<Cell> {
    &self.cells
  }

  pub fn reset(&mut self) {
    self.cells.fill(Cell::air());
  }

  // === cell helpers
  
  fn get_cell_under(&self, x: usize, y: usize) -> Cell {
    self.get(x, y + 1).unwrap_or(Cell::oob())
  }

  fn get_pos_left(&self, x: usize, y: usize) -> Option<(usize, usize)> {
    if x == 0 {
      return None;
    }
    Some((x - 1, y))
  }

  fn get_pos_right(&self, x: usize, y: usize) -> Option<(usize, usize)> {
    if x >= SANDBOX_WIDTH - 1 {
      return None;
    }
    Some((x + 1, y))
  }
  
  
  fn get_pos_bot_left(&self, x: usize, y: usize) -> Option<(usize, usize)> {
    if x == 0 || y >= SANDBOX_HEIGHT - 1  {
      return None;
    }
    Some((x - 1, y + 1))
  }
  
  fn get_pos_bot_right(&self, x: usize, y: usize) -> Option<(usize, usize)> {
    if x >= SANDBOX_WIDTH - 1 || y >= SANDBOX_HEIGHT - 1  {
      return None;
    }
    Some((x + 1, y + 1))
  }
  

  fn get_cell_from_pos_or_oob(&self, pos: Option<(usize, usize)>) -> Cell {
    if pos == None {
      return Cell::oob();
    }
    let pos = pos.unwrap();
    self.get(pos.0, pos.1).unwrap_or(Cell::oob())
  }

  fn swap_cell(&mut self, ax: usize, ay: usize, bx: usize, by: usize) {
    let a = self.get_unchecked(ax, ay);
    let b = self.get_unchecked(bx, by);
    self.set(ax, ay, b);
    self.set(bx, by, a);
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
      Material::WATER => self._update_liquid(x, y),
      _ => {}
    }
  }

  pub fn toggle_pause(&mut self) {
    self.paused = !self.paused;
  }

  // logic by-material

  fn _update_powder(&mut self, x: usize, y: usize) {
    if self.get_cell_under(x, y).weight < 2 {
      self.swap_cell(x, y, x, y + 1);
    } else if self.get_cell_from_pos_or_oob(self.get_pos_bot_left(x, y)).weight < 2 {
      self.swap_cell(x, y, x - 1, y + 1);
    } else if self.get_cell_from_pos_or_oob(self.get_pos_bot_right(x, y)).weight < 2 {
      self.swap_cell(x, y, x + 1, y + 1);
    }
  }

  fn _update_liquid(&mut self, x: usize, y: usize) {
    if self.get_cell_under(x, y).weight < 1 {
      self.swap_cell(x, y, x, y + 1);
      return;
    }
    
    let mut rng = rand::rng();
    let mut dirs_diag = [(-1, 1), (1, 1)];
    let mut dirs_flat = [(-1, 0), (1, 0)];

    dirs_diag.shuffle(&mut rng);
    dirs_flat.shuffle(&mut rng);

    // let lb_free: bool = self.get_cell_from_pos_or_oob(self.get_pos_bot_left(x, y)).weight < 1;
    // let rb_free: bool = self.get_cell_from_pos_or_oob(self.get_pos_bot_right(x, y)).weight < 1;
    // let l_free: bool = self.get_cell_from_pos_or_oob(self.get_pos_left(x, y)).weight < 1;
    // let r_free: bool = self.get_cell_from_pos_or_oob(self.get_pos_right(x, y)).weight < 1;

    self._update_liquid_balance(x, y, dirs_diag, 4);
    self._update_liquid_balance(x, y, dirs_flat, 8);
  }

  fn _update_liquid_balance(&mut self, x: usize, y: usize, dirs: [(isize, isize); 2], dist:  isize) {
    for dist in 1..dist {
      for (dx, dy) in dirs {
        let nx = x as isize + (dx * dist);
        let ny = y as isize + dy;

        if nx < 0 || nx >= (SANDBOX_WIDTH - 1) as isize || ny > (SANDBOX_HEIGHT - 1) as isize {
          break;
        }

        if let Some(cel) = self.get(nx as usize, ny as usize) {
          if cel.weight < 1 {
            self.swap_cell(x, y, nx as usize, ny as usize);
            break;
          } else {
            return;
          }
        }
      
      }
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
        Material::STONE => Color::GRAY,
        Material::WATER => Color::BLUE,
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

pub fn render_mouse_overlay(handle: &mut RaylibDrawHandle, mouse_pos: Vector2, size: usize) {
  handle.draw_rectangle_lines((mouse_pos.x - 1.0) as i32, (mouse_pos.y + 1.0) as i32, (size * CELL_SIZE) as i32, (size * CELL_SIZE) as i32, Color::WHITE);
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

