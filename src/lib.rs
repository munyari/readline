extern crate "readline-sys" as ffi;

pub fn readline(prompt: &str) -> Option<String> {
    ffi::readline(prompt)
}

pub fn add_history(line: &str) {
    ffi::add_history(line);
}
