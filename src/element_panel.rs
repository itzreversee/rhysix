use raylib::{color::Color, math::{Rectangle, Vector2}, prelude::{RaylibDraw, RaylibDrawHandle}, RaylibHandle};

use crate::{cell::Cell, sandbox::Sandbox, SCREEN_HEIGHT, SCREEN_WIDTH};

const PANEL_WIDTH: i32 = 200;
const PANEL_HEIGHT: i32 = SCREEN_HEIGHT - 16;

const PANEL_X: i32 = SCREEN_WIDTH - PANEL_WIDTH - 8;
const PANEL_Y: i32 = 8;

const BUTTON_WIDTH: i32 = 80;
const BUTTON_HEIGHT: i32 = 40;

struct PanelButton {
  x: i32,
  y: i32,
  text: String,
  color: Color,
  text_color: Color,
  action: String
}

impl PanelButton {
  fn render(&self, handle: &mut RaylibDrawHandle) {
    handle.draw_rectangle(self.x + PANEL_X, self.y + PANEL_Y, BUTTON_WIDTH, BUTTON_HEIGHT, self.color);
    handle.draw_text(&self.text, self.x + PANEL_X + 8, self.y + 8 + PANEL_Y, 24, self.text_color);
  }

  fn mouse_inside(&self, mouse_pos: Vector2) -> bool {
    let rect: Rectangle = Rectangle { x: (self.x + PANEL_X) as f32, y: (self.y + PANEL_Y) as f32, width: BUTTON_WIDTH as f32, height: BUTTON_HEIGHT as f32 };
    if rect.check_collision_point_rec(mouse_pos) {
      return true;
    }
    return false;
  }
}

pub struct ElementPanel {
  visible: bool,
  buttons: Vec<PanelButton>,
  lock_visibility: bool,
}

impl ElementPanel {
  pub fn new() -> Self {
    let buttons: Vec<PanelButton> = vec![
      PanelButton { x: 4, y: 4, text: String::from("sand"), color: Color::GOLD, text_color: Color::BLACK, action: String::from("switch/sand")},
      PanelButton { x: 90, y: 4, text: String::from("stone"), color: Color::GRAY, text_color: Color::WHITE, action: String::from("switch/stone")},
      PanelButton { x: 4, y: 48, text: String::from("water"), color: Color::BLUE, text_color: Color::WHITE, action: String::from("switch/water")},
      PanelButton { x: 90, y: 48, text: String::from("air"), color: Color::WHITE, text_color: Color::BLACK, action: String::from("switch/air")},
    ];
    Self {
      visible: false,
      buttons: buttons,
      lock_visibility: false
    }
  }

  pub fn render(&self, handle: &mut RaylibDrawHandle) {
    if !self.visible {
      return;
    }

    handle.draw_rectangle(PANEL_X, PANEL_Y, PANEL_WIDTH, PANEL_HEIGHT, Color::from_hex("404040").unwrap());
    for button in &self.buttons {
      button.render(handle);
    }
  }

  pub fn tick(&mut self, handle: &mut RaylibHandle, sandbox: &mut Sandbox) -> bool{
    let mouse_pos = handle.get_mouse_position();
    let mouse_left = handle.is_mouse_button_pressed(raylib::ffi::MouseButton::MOUSE_BUTTON_LEFT);
    let mouse_right = handle.is_mouse_button_pressed(raylib::ffi::MouseButton::MOUSE_BUTTON_RIGHT);
    
    if self.visible && mouse_right {
      self.visible = false;
      self.lock_visibility = true;
    }

    if !self.visible && !self.lock_visibility {
      if mouse_pos.x > (SCREEN_WIDTH - 45) as f32 {
        self.visible = true;
      }
    } else {
      if mouse_pos.x < ((SCREEN_WIDTH - PANEL_WIDTH - 20)) as f32 {
        self.visible = false;
        self.lock_visibility = false;
      }
    }

    if !self.visible {
      return false;
    }

    for button in &self.buttons {
      if button.mouse_inside(mouse_pos) && mouse_left {
        self.handle_action(&button.action, sandbox);
      }
    }

    return true;
  }

  fn handle_action(&self, action: &str, sandbox: &mut Sandbox) {
    let action_data: Vec<&str> = action.split("/").collect();

    match action_data[0] {
      "switch" => {
        match action_data[1] {
          "sand" => {
            sandbox.set_hand_cell(Cell::sand());
          }
          "stone" => {
            sandbox.set_hand_cell(Cell::stone());
          }
          "water" => {
            sandbox.set_hand_cell(Cell::water());
          }
          "air" => {
            sandbox.set_hand_cell(Cell::air());
          }
          _ => {}
        }
      }
      _ => {}
    }
  }
}