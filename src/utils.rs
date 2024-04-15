//! Utilities used during development and debugging.

use ncurses::*;

/// Displays a message in the left-bottom corner of the terminal.
pub fn debug(msg: &str) {
  let mut x = 0;
  let mut y = 0;
  let mut mx = 0;
  let mut my = 0;
  getyx(stdscr(), &mut y, &mut x);
  getmaxyx(stdscr(), &mut my, &mut mx);
  let _ = mvaddstr(my - 1, 1, &format!("{:40}", msg));
  mv(y, x);
  refresh();
}
