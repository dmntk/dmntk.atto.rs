use super::*;
use crate::plane::*;

#[test]
fn _0001() {
  let plane = &mut Plane::new(TEST_INPUT_X);
  eq_cursor(1, 1, plane);
  repeat!(6, plane, cursor_move_right);
  eq_cursor(1, 7, plane);
  plane.split_line();
}
