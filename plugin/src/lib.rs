#![feature(asm)]#![allow(unused)]#![allow(non_snake_case)]#![allow(unused_imports)]#![allow(unused_variables)]
#![feature(proc_macro_hygiene)]

#[smashline::installer]
pub fn install() {
    fighters::install();
}

#[skyline::main(name = "hdr")]
pub fn main() {
    #[cfg(not(feature = "runtime"))]
    { utils::init(); }
    fighters::install();
}