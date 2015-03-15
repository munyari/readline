#![feature(io)]

extern crate "readline-sys" as readline_sys;
extern crate libc;

pub use common::Error;
pub use common::add_history;
pub use common::readline;
pub use common::readline_bare;

mod common;
mod linux;
mod other;
