#![cfg(not(test))]

extern crate readline;

use readline::readline;

use std::ffi::CString;
use std::io::Write;
use std::io;

fn main() {
    let prompt = CString::new("user> ").unwrap();
    let mut stdout = io::stdout();
    while let Ok(s) = readline(&prompt) {
        stdout.write_all(s.to_bytes()).unwrap();
        stdout.write_all(b"\n").unwrap();
    }
    stdout.write_all(b"\n").unwrap();
}
