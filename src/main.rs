mod util;
mod sandbox;

use std::time::{Duration, Instant};

use raylib::ffi::SCROLLBAR_RIGHT_SIDE;
use raylib::prelude::*;

use crate::util::{text_to_width};
use crate::sandbox::{render_sandbox, window_to_world, Sandbox, Material};

struct GameThread {
  handle: RaylibHandle,
  thread: RaylibThread,
  sandbox: Sandbox,
  last_tick: Instant,
  tick_rate: Duration
}

impl GameThread {
  fn render(&mut self) {
    let mut d = self.handle.begin_drawing(&self.thread);
    d.clear_background(Color::BLACK);

    render_sandbox(self.sandbox.get_buffer(), &mut d);

    //let title = "rhysix meow";
    let title = format!("rhysix");
    d.draw_text(&title, 800 - text_to_width(&title, 20), 600 - 32, 20, Color::WHITE);
  }

  fn tick(&mut self) {
    if self.handle.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
      let mouse_pos = self.handle.get_mouse_position();
      window_to_world(mouse_pos.x as i32, mouse_pos.y as i32).map(|pos| {
        self.sandbox.set(pos.0, pos.1, Material::SAND);
      });
    }

    else if self.handle.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
      let mouse_pos = self.handle.get_mouse_position();
      window_to_world(mouse_pos.x as i32, mouse_pos.y as i32).map(|pos| {
        println!("{}", format!("maus: {:?}", (pos, self.sandbox.get(pos.0, pos.1))))
      });
    }

    match self.handle.get_key_pressed() {
      Some(KeyboardKey::KEY_SPACE) => {
        self.sandbox.toggle_pause();
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