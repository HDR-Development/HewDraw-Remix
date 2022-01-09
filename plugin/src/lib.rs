#![feature(asm)]#![allow(unused_imports)]#![allow(unused_variables)]
#![feature(proc_macro_hygiene)]

#[skyline::main(name = "hdr")]
pub fn main() {
    #[cfg(not(feature = "runtime"))]
    { utils::init(); }
    fighters::install();
}