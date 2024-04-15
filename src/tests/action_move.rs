//! Tests of cursor movements.
//!
//! Generally, in every test case it is assumed, that after opening the editor,
//! the cursor position is top left corner of the decision table.

use super::*;
use crate::plane::Plane;

/// Moving up should not be possible, while there is a top first horizontal line.
/// Cursor position should stayed unchanged.
#[test]
fn _0001() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  plane.cursor_move_up();
  eq_cursor(1, 1, plane);
}

/// Cursor is moved down and up, cursor position should be the starting position.
#[test]
fn _0002() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  plane.cursor_move_down();
  eq_cursor(3, 1, plane);
  plane.cursor_move_up();
  eq_cursor(1, 1, plane);
}

/// Moving up inside the cell (2 lines below the top horizontal line).
#[test]
fn _0003() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  repeat!(3, plane, cursor_move_down);
  plane.cursor_move_up();
  eq_cursor(4, 1, plane);
}

/// Cursor is moved down and up, cursor position should be the starting position.
#[test]
fn _0004() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  plane.cursor_move_down();
  repeat!(34, plane, cursor_move_right);
  eq_cursor(3, 38, plane);
  plane.cursor_move_up();
  eq_cursor(3, 38, plane);
}

/// After opening the cursor is placed in the top-left corner of the information item name cell.
/// While moving down, the cursor should jump over the bottom horizontal line of the cell.  
/// Cursor position should be adjusted.
#[test]
fn _0005() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  plane.cursor_move_down();
  eq_cursor(3, 1, plane);
}

/// After opening the cursor is placed in the top-left corner of the information item name cell.
/// Cursor is moved down and right until it stays above `┬` character.
/// Now moving down should not be allowed. Cursor position should stay unchanged.
#[test]
fn _0006() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  plane.cursor_move_down();
  repeat!(31, plane, cursor_move_right);
  eq_cursor(3, 35, plane);
  plane.cursor_move_down();
  eq_cursor(3, 35, plane);
}

/// Moving left should not be possible, while there is a vertical line on the left.
/// Cursor position should stayed unchanged.
#[test]
fn _0007() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  plane.cursor_move_left();
  eq_cursor(1, 1, plane);
}

/// Moving right and left should return to the same starting position.
#[test]
fn _0008() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  plane.cursor_move_right();
  plane.cursor_move_left();
  eq_cursor(1, 1, plane);
}

/// Moving left should jump over the vertical lines.
#[test]
fn _0009() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  repeat!(2, plane, cursor_move_down);
  plane.cursor_move_table_end();
  repeat!(22, plane, cursor_move_left);
  eq_cursor(4, 48, plane);
}

/// Moving left should NOT jump over the horizontal lines or crossings.
#[test]
fn _0010() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  repeat!(2, plane, cursor_move_down);
  plane.cursor_move_table_end();
  repeat!(50, plane, cursor_move_left);
  eq_cursor(4, 47, plane);
}

/// Moving right should jump over the vertical lines.
#[test]
fn _0011() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  plane.cursor_move_down();
  repeat!(3, plane, cursor_move_right);
  eq_cursor(3, 5, plane);
}

/// Moving right should NOT jump over the horizontal lines or crossings.
#[test]
fn _0012() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  repeat!(2, plane, cursor_move_down);
  repeat!(50, plane, cursor_move_right);
  eq_cursor(4, 23, plane);
}

/// Moving to the right side of the cell.
#[test]
fn _0013() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  plane.cursor_move_cell_end();
  eq_cursor(1, 37, plane);
}

/// Moving to the left side of the cell.
#[test]
fn _0014() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  plane.cursor_move_cell_end();
  plane.cursor_move_cell_start();
  eq_cursor(1, 1, plane);
}

/// Moving to the beginning of the next cell.
#[test]
fn _0015() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  repeat!(1, plane, cursor_move_down);
  plane.cursor_move_cell_right();
  eq_cursor(3, 5, plane);
}

/// Moving to the next cell stopped at the cell end because of horizontal line.
#[test]
fn _0016() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  repeat!(4, plane, cursor_move_down);
  plane.cursor_move_cell_right();
  eq_cursor(6, 3, plane);
}

/// Moving to the end of the previous cell.
#[test]
fn _0017() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  repeat!(1, plane, cursor_move_down);
  plane.cursor_move_cell_right();
  plane.cursor_move_cell_left();
  eq_cursor(3, 3, plane);
}

/// Moving to the end of the table.
#[test]
fn _0018() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  repeat!(1, plane, cursor_move_down);
  plane.cursor_move_table_end();
  eq_cursor(3, 71, plane);
}

/// Moving to the end of the table but stopped at the cell end,
/// when moving to the end is not possible because of horizontal line.
#[test]
fn _0019() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  repeat!(4, plane, cursor_move_down);
  plane.cursor_move_table_end();
  eq_cursor(6, 3, plane);
}

/// Moving to the end of the table and to the start of the table
/// should not change the cursor position.
#[test]
fn _0020() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  repeat!(2, plane, cursor_move_down);
  plane.cursor_move_table_end();
  plane.cursor_move_table_start();
  eq_cursor(4, 1, plane);
}
