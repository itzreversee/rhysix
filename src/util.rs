// DEPRECATED: use ?.handle.measure_text() instead
pub fn text_to_width(s: &str, font_size: i32) -> i32{
  (s.len() as i32) * (font_size - 6)
}