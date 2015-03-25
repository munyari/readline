extern crate readline_sys;
extern crate libc;

pub use common::Error;
pub use common::ReadlineBytes;
pub use common::add_history;
pub use common::readline;
pub use common::readline_bare;

mod common;
mod linux;
mod other;
