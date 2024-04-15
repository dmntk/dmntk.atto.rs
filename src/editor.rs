use crate::errors::*;
use crate::keys::*;
use crate::plane::*;
use crate::utils::debug;
use ncurses::*;
use std::fs;
use std::time::Instant;

/// Editor actions.
enum EditorAction {
  CursorMoveCellStart,
  CursorMoveCellEnd,
  CursorMoveCellLeft,
  CursorMoveCellRight,
  CursorMoveDown,
  CursorMoveLeft,
  CursorMoveRight,
  CursorMoveTableStart,
  CursorMoveTableEnd,
  CursorMoveUp,
  DebugKeystroke(i32, String),
  DeleteChar,
  DeleteCharBefore,
  InsertChar(char),
  Nop,
  ResizeWindow,
  SplitLine,
  Quit,
}

/// Implementation of the decision table editor.
pub struct Editor {
  /// Handle of the main window of the terminal.
  window: WINDOW,
  /// Plane where the edited text is stored and manipulated.
  plane: Plane,
}

impl Editor {
  /// Creates a new editor initialized with the content loaded from file.
  pub fn new(file_name: &str) -> Result<Self, AttoError> {
    let content = fs::read_to_string(file_name).map_err(|e| err_load_file(file_name, &e.to_string()))?;
    let plane = Plane::new(&content);
    let window = Self::initialize();
    Ok(Self { window, plane })
  }

  /// Initializes terminal via ncurses.
  pub fn initialize() -> WINDOW {
    let locale_conf = LcCategory::all;
    let _ = setlocale(locale_conf, "en_US.UTF-8");
    let window = initscr();
    raw();
    keypad(window, true);
    noecho();
    window
  }

  /// Terminates terminal via ncurses.
  pub fn finalize(&self) -> Result<()> {
    endwin();
    Ok(())
  }

  /// Updates cursor position.
  pub fn update_cursor(&self) {
    mv(self.plane.cursor_row() as i32, self.plane.cursor_col() as i32);
  }

  /// Updates cursor coordinates in status bar.
  pub fn update_cursor_coordinates(&self) {
    let mut cur_x = 0;
    let mut cur_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(self.window, &mut max_y, &mut max_x);
    getyx(self.window, &mut cur_y, &mut cur_x);
    let _ = mvaddstr(
      max_y - 1,
      max_x - 20,
      &format!("{:>20}", format!("{}:{} ", self.plane.cursor_col(), self.plane.cursor_row())),
    );
    mv(cur_y, cur_x);
  }

  /// Repaints the content of a plane.
  pub fn repaint_plane(&self) {
    for (r, row) in self.plane.chars.iter().enumerate() {
      mv(r as i32, 0);
      let _ = addstr(&row.iter().collect::<String>());
      let _ = addstr("  ");
    }
  }

  /// Maps a key-stroke to editor action.
  fn map_key_to_action(&self, key: i32) -> EditorAction {
    if let Some(key_name) = keyname(key) {
      match key_name.as_str() {
        KN_CTRL_Q => EditorAction::Quit,
        KN_UP => EditorAction::CursorMoveUp,
        KN_DOWN => EditorAction::CursorMoveDown,
        KN_LEFT => EditorAction::CursorMoveLeft,
        KN_RIGHT => EditorAction::CursorMoveRight,
        KN_BACKSPACE => EditorAction::DeleteCharBefore,
        KN_DELETE => EditorAction::DeleteChar,
        KN_HOME => EditorAction::CursorMoveCellStart,
        KN_END => EditorAction::CursorMoveCellEnd,
        KN_SHIFT_HOME => EditorAction::CursorMoveTableStart,
        KN_SHIFT_END => EditorAction::CursorMoveTableEnd,
        KN_TAB => EditorAction::CursorMoveCellRight,
        KN_SHIFT_TAB => EditorAction::CursorMoveCellLeft,
        KN_RESIZE => EditorAction::ResizeWindow,
        _ => match key {
          10 => EditorAction::SplitLine,
          32..=126 => EditorAction::InsertChar(char::from_u32(key as u32).unwrap()),
          127 => EditorAction::DeleteChar,
          _ => EditorAction::DebugKeystroke(key, key_name),
        },
      }
    } else {
      EditorAction::Nop
    }
  }

  /// Processes input key-strokes.
  pub fn process_keystrokes(&mut self) {
    loop {
      match self.map_key_to_action(getch()) {
        EditorAction::CursorMoveCellStart => {
          if self.plane.cursor_move_cell_start() {
            self.update_cursor();
            self.update_cursor_coordinates();
            refresh();
          }
        }
        EditorAction::CursorMoveCellEnd => {
          if self.plane.cursor_move_cell_end() {
            self.update_cursor();
            self.update_cursor_coordinates();
            refresh();
          }
        }
        EditorAction::CursorMoveCellLeft => {
          if self.plane.cursor_move_cell_left() {
            self.update_cursor();
            self.update_cursor_coordinates();
            refresh();
          }
        }
        EditorAction::CursorMoveCellRight => {
          if self.plane.cursor_move_cell_right() {
            self.update_cursor();
            self.update_cursor_coordinates();
            refresh();
          }
        }
        EditorAction::CursorMoveDown => {
          if self.plane.cursor_move_down() {
            self.update_cursor();
            self.update_cursor_coordinates();
            refresh();
          }
        }
        EditorAction::CursorMoveLeft => {
          if self.plane.cursor_move_left() {
            self.update_cursor();
            self.update_cursor_coordinates();
            refresh();
          }
        }
        EditorAction::CursorMoveRight => {
          if self.plane.cursor_move_right() {
            self.update_cursor();
            self.update_cursor_coordinates();
            refresh();
          }
        }
        EditorAction::CursorMoveTableStart => {
          if self.plane.cursor_move_table_start() {
            self.update_cursor();
            self.update_cursor_coordinates();
            refresh();
          }
        }
        EditorAction::CursorMoveTableEnd => {
          if self.plane.cursor_move_table_end() {
            self.update_cursor();
            self.update_cursor_coordinates();
            refresh();
          }
        }
        EditorAction::CursorMoveUp => {
          if self.plane.cursor_move_up() {
            self.update_cursor();
            self.update_cursor_coordinates();
            refresh();
          }
        }
        EditorAction::DebugKeystroke(key, key_name) => {
          debug(&format!("KEY: {} | {}", key, key_name));
        }
        EditorAction::DeleteChar => {
          self.plane.delete_char();
          self.repaint_plane();
          self.update_cursor();
          self.update_cursor_coordinates();
        }
        EditorAction::DeleteCharBefore => {
          self.plane.delete_char_before();
          self.repaint_plane();
          self.update_cursor();
          self.update_cursor_coordinates();
          refresh();
        }
        EditorAction::Nop => {}
        EditorAction::InsertChar(ch) => {
          let now = Instant::now();
          self.plane.insert_char(ch);
          let elapsed = now.elapsed();
          debug(&format!("{:.2?}", elapsed));
          self.repaint_plane();
          self.update_cursor();
          self.update_cursor_coordinates();
          refresh();
        }
        EditorAction::ResizeWindow => {
          // getmaxyx(self.window, &mut max_y, &mut max_x);
          // getyx(window, &mut cur_y, &mut cur_x);
          // attron(A_REVERSE());
          // mvaddstr(43, 1, &format!("{}:{}", max_x, max_y));
          // attroff(A_REVERSE());
          // mv(cur_y, cur_x);
          // refresh();
        }
        EditorAction::SplitLine => {
          self.plane.split_line();
          self.repaint_plane();
          self.update_cursor();
          self.update_cursor_coordinates();
          refresh();
        }
        EditorAction::Quit => break,
      }
    }
  }
}
