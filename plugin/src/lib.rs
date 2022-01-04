#![feature(proc_macro_hygiene)]

#[skyline::main(name = "hdr")]
pub fn main() {
    hdr_core::init();
}