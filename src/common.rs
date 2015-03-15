use std::ffi::CStr;
use std::ops;

#[cfg(any(target_os = "linux", target_os = "macos"))]
use linux as sys;

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
use other as sys;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Error(());

pub fn error<T>() -> Result<T, Error> { Err(Error(())) }

pub struct ReadlineBytes {
    inner: sys::ReadlineBytes,
}

impl ops::Deref for ReadlineBytes {
    type Target = CStr;
    fn deref<'a>(&'a self) -> &'a CStr {
        self.inner.deref()
    }
}

pub fn readline_bare(prompt: &CStr) -> Result<ReadlineBytes, Error> {
    sys::readline_bare(prompt).map(|line| ReadlineBytes { inner: line })
}

pub fn readline(prompt: &CStr) -> Result<ReadlineBytes, Error> {
    let maybe_line = readline_bare(prompt);
    if let Ok(ref line) = maybe_line {
        add_history(line);
    }
    maybe_line
}

pub fn add_history(line: &CStr) {
    sys::add_history(line);
}
