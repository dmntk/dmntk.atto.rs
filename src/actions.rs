//! Actions invoked by command-line arguments.

use crate::errors::Result;
use crate::{Editor, ATTO_DESCRIPTION, ATTO_NAME, ATTO_VERSION};
use clap::{arg, ArgMatches, Command};
use ncurses::*;

/// Available command-line actions.
enum CliAction {
  /// Edit decision table loaded from file with specified name.
  EditDecisionTable(String),
}

/// Parses command-line arguments.
fn get_matches() -> ArgMatches {
  Command::new(ATTO_NAME)
    .version(ATTO_VERSION)
    .about(ATTO_DESCRIPTION)
    .arg(arg!(<FILE>).help("File containing decision table to edit").required(true).index(1))
    .get_matches()
}

/// Checks arguments passed from the command line and returns a corresponding action.
fn get_cli_action() -> CliAction {
  let matches = get_matches();
  let file_name = matches.get_one::<String>("FILE").unwrap();
  CliAction::EditDecisionTable(file_name.to_string())
}

/// Executes command-line action.
pub fn do_action() -> Result<()> {
  match get_cli_action() {
    CliAction::EditDecisionTable(file_name) => {
      let mut editor = Editor::new(&file_name)?;
      editor.repaint_plane();
      editor.update_cursor();
      editor.update_cursor_coordinates();
      refresh();
      editor.process_keystrokes();
      editor.finalize()
    }
  }
}
