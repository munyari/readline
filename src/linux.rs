use common::Error;
use common::error;
use libc::c_char;
use libc::c_void;
use libc;
use readline_sys;
use std::ffi::CStr;
use std::ops;
use std::ptr;

#[allow(dead_code)]
pub struct ReadlineBytes {
    // The `'static` is a lie, don't expose it.
    bytes: &'static CStr,
}

impl ops::Deref for ReadlineBytes {
    type Target = CStr;
    fn deref<'a>(&'a self) -> &'a CStr {
        self.bytes
    }
}

impl Drop for ReadlineBytes {
    fn drop(&mut self) {
        unsafe { libc::free(self.bytes.as_ptr() as *mut c_void); }
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
#[allow(dead_code)]
pub fn readline_bare(prompt: &CStr) -> Result<ReadlineBytes, Error> {
    unsafe {
        let bytes: *const c_char =
            readline_sys::readline(prompt.as_ptr() as *const c_char);
        if bytes == ptr::null() {
            return error();
        }
        Ok(ReadlineBytes {
            bytes: CStr::from_ptr(bytes),
        })
    }
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
#[allow(dead_code)]
pub fn readline_bare(prompt: &CStr) -> Result<ReadlineBytes, EndOfFile> {
    unimplemented!("on this platform");
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
#[allow(dead_code)]
pub fn add_history(line: &CStr) {
    unsafe { readline_sys::add_history(line.as_ptr()) };
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
#[allow(dead_code)]
pub fn add_history(line: &CStr) {
    unimplemented!("on this platform");
}
