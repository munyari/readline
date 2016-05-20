extern crate readline_sys as sys;

use libc;

use Error;
use std::ffi::{CStr, CString};
use std::ptr;

pub fn readline(prompt: &str) -> Result<String, Error> {
  unsafe {
    let prompt = match CString::new(prompt.to_owned()) {
      Ok(p) => p,
      Err(_) => return Err(Error::PromptInternalNul),
    };
    let ptr = sys::readline(prompt.as_ptr());
    if ptr == ptr::null_mut() {
      Err(Error::EndOfFile)
    } else {
      let ret = match CStr::from_ptr(ptr).to_str() {
        Ok(s) => Ok(s.to_owned()),
        Err(_) => Err(Error::InvalidUtf8),
      };
      libc::free(ptr as *mut libc::c_void);
      ret
    }
  }
}

pub fn add_history(line: &str) -> Result<(), Error> {
  unsafe {
    let line = match CString::new(line.to_owned()) {
      Ok(l) => l,
      Err(_) => return Err(Error::PromptInternalNul),
    };
    Ok(sys::add_history(line.as_ptr()))
  }
}
