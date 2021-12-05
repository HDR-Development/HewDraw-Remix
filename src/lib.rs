#![feature(proc_macro_hygiene)]

use skyline::{hook, install_hook};

extern "C" fn test() -> u32 {
    2
}

#[hook(replace = test)]
fn test_replacement() -> u32 {

    let original_test = original!();

    let val = original_test();

    println!("[override] original value: {}", val); // 2

    val + 1
}

#[skyline::main(name = "tmp")]
pub fn main() {
    println!("Hello from Skyline Rust!");

    install_hook!(test_replacement);

    let x = test();

    println!("[main] test returned: {}", x); // 3
}
