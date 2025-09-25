mod util;
mod sandbox;
mod cell;

use std::time::{Duration, Instant};

use raylib::prelude::*;

use crate::cell::Cell;
use crate::sandbox::{render_sandbox, window_to_world, Sandbox};

struct GameThread {
  handle: RaylibHandle,
  thread: RaylibThread,
  sandbox: Sandbox,
  last_tick: Instant,
  tick_rate: Duration
}

impl GameThread {
  fn render(&mut self) {
    let title = format!("{:?}; {:?}", (self.sandbox.get_hand_cell().material), (self.sandbox.get_size()));
    let width = self.handle.measure_text(&title, 20);

    let mut d = self.handle.begin_drawing(&self.thread);
    d.clear_background(Color::BLACK);

    render_sandbox(self.sandbox.get_buffer(), &mut d);

    //let title = "rhysix meow";
    d.draw_text(&title, 800 - width - 16, 600 - 32, 20, Color::WHITE);
  }

  fn tick(&mut self) {
    if self.handle.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
      let mouse_pos = self.handle.get_mouse_position();
      window_to_world(mouse_pos.x as i32, mouse_pos.y as i32).map(|pos| {
        self.sandbox.place(pos.0, pos.1);
      });
    }

    else if self.handle.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
      let mouse_pos = self.handle.get_mouse_position();
      window_to_world(mouse_pos.x as i32, mouse_pos.y as i32).map(|pos| {
        println!("{}", format!("maus: {:?}", (pos, self.sandbox.get(pos.0, pos.1))))
      });
    }

    match self.handle.get_key_pressed() {
      // Controls
      Some(KeyboardKey::KEY_SPACE) => {
        self.sandbox.toggle_pause();
      }
      Some(KeyboardKey::KEY_MINUS) => {
        self.sandbox.dec_size();
      }
      Some(KeyboardKey::KEY_EQUAL) => {
        self.sandbox.inc_size();
      }
      // Materials
      Some (KeyboardKey::KEY_ONE) => {
        self.sandbox.set_hand_cell(Cell::sand());
      }
      Some (KeyboardKey::KEY_TWO) => {
        self.sandbox.set_hand_cell(Cell::stone());
      }
      Some (KeyboardKey::KEY_THREE) => {
        self.sandbox.set_hand_cell(Cell::water());
      }
      Some (KeyboardKey::KEY_ZERO) => {
        self.sandbox.set_hand_cell(Cell::air());
      }
      _ => {}
    }

    // update
    if self.last_tick.elapsed() >= self.tick_rate {
      self.sandbox.tick();
      self.last_tick = std::time::Instant::now();
    }
  }
}

fn main() {
  let (rl, thread) = raylib::init()
    .size(800, 600)
    .title("Rhysix")
    .build();

  let sb =  Sandbox::new();
  let mut game_thread: GameThread = GameThread { 
    sandbox: sb,
    handle: rl, 
    thread: thread, 
    last_tick: std::time::Instant::now(),
    tick_rate: std::time::Duration::from_millis(10)
  };

  game_thread.handle.set_target_fps(60);

  while !game_thread.handle.window_should_close() {
    game_thread.tick();
    game_thread.render();
  }
}