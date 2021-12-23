#[macro_use]
extern crate lazy_static;


mod offsets;
mod modules;

pub mod singletons;
pub mod utils;

pub use modules::*;

pub fn init() {
    modules::init();
}