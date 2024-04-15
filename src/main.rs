//! Implementation of a minimalistic decision table editor.

extern crate clap;
extern crate ncurses;

mod actions;
mod editor;
mod errors;
mod keys;
mod plane;
#[cfg(test)]
mod tests;
mod utils;

use crate::actions::do_action;
use editor::Editor;
use errors::Result;
use std::env;

/// Name of this editor.
const ATTO_NAME: &str = "atto";

/// Current editor version.
const ATTO_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Short description.
const ATTO_DESCRIPTION: &str = "Decision table editor";

/// Main entrypoint.
fn main() -> Result<()> {
  do_action()
}
