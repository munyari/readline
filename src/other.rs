use std::io;
use Error;

pub fn readline(prompt: &str) -> Result<String, Error> {
  use std::io::Write;

  print!("{}", prompt);
  io::stdout().flush().expect("flushing failed");

  let mut line = String::new();
  if let Err(e) = io::stdin().read_line(&mut line) {
    match e.kind() {
      io::ErrorKind::UnexpectedEof => Err(Error::EndOfFile),
      io::ErrorKind::InvalidData => Err(Error::InvalidUtf8),
      _ => Err(Error::IoError(e)),
    }
  } else {
    // TODO(ubsan): make sure this actually works
    line.pop(); // Take the last '\n' off the string.

    Ok(line)
  }
}

pub fn add_history(_line: &str) -> Result<(), Error> { Ok(()) }
