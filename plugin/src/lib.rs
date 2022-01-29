#![feature(asm)]#![allow(unused)]#![allow(non_snake_case)]#![allow(unused_imports)]#![allow(unused_variables)]
#![feature(proc_macro_hygiene)]

use skyline::libc::c_char;

#[smashline::installer]
pub fn install() {
    fighters::install();
}

extern "C" {
    fn change_version_string(arg: u64, string: *const c_char);
}

#[skyline::hook(replace = change_version_string)]
fn change_version_string_hook(arg: u64, string: *const c_char) {
    let original_str = unsafe { skyline::from_c_str(string) };
    if original_str.contains("Ver.") {
        let new_str = format!(
            "{}\nHDR Ver. {}\0",
            original_str,
            env!("CARGO_PKG_VERSION")
        );

        call_original!(arg, skyline::c_str(&new_str))
    } else {
        call_original!(arg, string)
    }
}

#[skyline::main(name = "hdr")]
pub fn main() {
    #[cfg(not(feature = "runtime"))]
    { utils::init(); }
    fighters::install();

    skyline::install_hook!(change_version_string_hook);
}