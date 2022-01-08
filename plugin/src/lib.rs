#![feature(asm)]
#![feature(proc_macro_hygiene)]

#[skyline::main(name = "hdr")]
pub fn main() {
    #[cfg(not(feature = "runtime"))]
    { utils::init(); }
    fighters::install();
}