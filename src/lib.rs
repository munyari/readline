extern crate libc;

use std::error::Error as ErrTrait;
use std::fmt::{self, Display, Formatter};

#[cfg(unix)]
#[path = "unix.rs"]
mod sys;

#[cfg(not(unix))]
#[path = "other.rs"]
mod sys;

pub use sys::{readline, add_history};

#[derive(Debug)]
pub enum Error {
  InvalidUtf8,
  EndOfFile,
  PromptInternalNul,
  IoError(std::io::Error),
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    write!(f, "{}", self.description())
  }
}

impl ErrTrait for Error {
  fn description(&self) -> &str {
    match *self {
      Error::InvalidUtf8 => "Invalid UTF-8",
      Error::EndOfFile => "End of file",
      Error::PromptInternalNul => "Internal nul inside prompt",
      Error::IoError(ref e) => e.description(),
    }
  }
}
