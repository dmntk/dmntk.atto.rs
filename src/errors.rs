//! Definition of the common error type.

use std::fmt;
use std::fmt::{Display};

/// Common result type.
pub type Result<T, E = AttoError> = std::result::Result<T, E>;

/// Common error definition.
#[derive(Debug)]
pub struct AttoError(String);

impl Display for AttoError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

/// Creates and error indicating that loading input file failed.
pub fn err_load_file(file_name: &str, reason: &str) -> AttoError {
  AttoError(format!("loading input file '{}' failed with reason: '{}'", file_name, reason))
}
