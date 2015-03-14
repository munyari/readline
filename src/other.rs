use common::Error;
use common::error;

use std::ffi::CStr;
use std::ffi::CString;
use std::io::BufRead;
use std::io::Write;
use std::io;
use std::ops;

#[allow(dead_code)]
pub struct ReadlineBytes {
    bytes: CString,
}

impl ops::Deref for ReadlineBytes {
    type Target = CStr;
    fn deref<'a>(&'a self) -> &'a CStr {
        &self.bytes
    }
}

#[allow(dead_code)]
pub fn add_history(line: &CStr) {
    let _ = line;
}

#[allow(dead_code)]
pub fn readline_bare(prompt: &CStr) -> Result<ReadlineBytes, Error> {
    let stdin_u = io::stdin();
    let mut stdin = stdin_u.lock();

    if let Err(_) = io::stdout().write_all(prompt.to_bytes()) {
        return error();
    }

    let mut line = vec![];
    if let Err(_) = stdin.read_until(b'\n', &mut line) {
        return error();
    }
    line.pop(); // Take the last '\n' off the string.

    Ok(ReadlineBytes { bytes: match CString::new(line) {
        Ok(s) => s,
        Err(_) => return error(),
    }})
}
