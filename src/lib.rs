#![feature(core, std_misc, libc)]
extern crate "readline-sys" as ffi;

extern crate libc;
use self::libc::c_char;

use std::ffi::{CString, c_str_to_bytes};
use std::string::String;

pub fn readline(prompt: &str) -> Option<String> {
    unsafe {
        // It doesn't matter if there is an interior null
        // It just won't prompt all the way 
        let line_ptr: *const c_char = 
            ffi::readline(CString::from_slice(prompt.as_bytes()).as_ptr());

        if line_ptr.is_null() {
            return None;
        }

        Some(String::from_utf8_lossy(c_str_to_bytes(&line_ptr)).into_owned())
    }
}

pub fn add_history(line: &str) {
    let l = CString::from_slice(line.as_bytes()).as_ptr();
    unsafe { ffi::add_history(l as *const c_char) };
}
