#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]#![allow(unused_imports)]#![allow(unused_variables)]
#![feature(proc_macro_hygiene)]

#[cfg(feature = "main_nro")]
mod random;

#[cfg(feature = "main_nro")]
mod controls;

#[cfg(feature = "main_nro")]
mod lua;

#[cfg(feature = "main_nro")]
mod online;

use skyline::libc::c_char;
#[cfg(feature = "main_nro")]
use skyline_web::*;
use std::{path::Path, fs};

#[cfg(feature = "updater")]
mod updater;

#[smashline::installer]
pub fn install() {
    fighters::install();
}

#[cfg(not(feature = "main_nro"))]
#[export_name = "hdr_delayed_install"]
pub extern "Rust" fn delayed_install() {
    fighters::delayed_install();
}

#[cfg(feature = "add_status")]
extern "Rust" {
    #[link_name = "hdr_delayed_install"]
    fn delayed_install();
}

#[cfg(feature = "main_nro")]
#[export_name = "hdr_is_available"]
pub extern "Rust" fn is_available() -> bool { true }

extern "C" {
    fn change_version_string(arg: u64, string: *const c_char);
}

#[cfg(feature = "main_nro")]
#[skyline::hook(replace = change_version_string)]
fn change_version_string_hook(arg: u64, string: *const c_char) {
    let original_str = unsafe { skyline::from_c_str(string) };
    if original_str.contains("Ver.") {
        let romfs_version = match std::fs::read_to_string("mods:/ui/romfs_version.txt") {
            Ok(version_value) => version_value.trim().to_string(),
            Err(_) => String::from("UNKNOWN"),
        };
        let hdr_version = match std::fs::read_to_string("mods:/ui/hdr_version.txt") {
            Ok(version_value) => version_value.trim().to_string(),
            Err(_) => {
                
                #[cfg(feature = "main_nro")]
                skyline_web::DialogOk::ok("hdr-assets is not enabled! Please enable hdr-assets in arcropolis config.");
                
                String::from("UNKNOWN")
            }
        };
        let new_str = format!(
            "{}\nHDR Ver. {}\nAssets Ver. {}\0",
            original_str,
            hdr_version,
            romfs_version
        );

        call_original!(arg, skyline::c_str(&new_str))
    } else {
        call_original!(arg, string)
    }
}

std::arch::global_asm!(
    r#"
    .section .nro_header
    .global __nro_header_start
    .word 0
    .word _mod_header
    .word 0
    .word 0
    
    .section .rodata.module_name
        .word 0
        .word 3
        .ascii "hdr"
    .section .rodata.mod0
    .global _mod_header
    _mod_header:
        .ascii "MOD0"
        .word __dynamic_start - _mod_header
        .word __bss_start - _mod_header
        .word __bss_end - _mod_header
        .word __eh_frame_hdr_start - _mod_header
        .word __eh_frame_hdr_end - _mod_header
        .word __nx_module_runtime - _mod_header // runtime-generated module object offset
    .global IS_NRO
    IS_NRO:
        .word 1
    
    .section .bss.module_runtime
    __nx_module_runtime:
    .space 0xD0
    "#
);

#[no_mangle]
pub extern "C" fn main() {
    #[cfg(feature = "main_nro")] {
        quick_validate_install();
        skyline::install_hooks!(change_version_string_hook);
        random::install();
        controls::install();
        lua::install();
        online::install();
    }

    #[cfg(not(feature = "runtime"))]
    { utils::init(); }
    fighters::install();
    #[cfg(all(not(feature = "add_status"), feature = "main_nro"))]
    { if !(delayed_install as *const ()).is_null() { unsafe { delayed_install(); } } }

    #[cfg(all(feature = "add_status", not(all(not(feature = "add_status"), feature = "main_nro"))))]
    { fighters::delayed_install(); }

    #[cfg(feature = "updater")]
    {
        std::thread::Builder::new()
            .stack_size(0x40_0000)
            .spawn(|| {
                updater::check_for_updates();
            })
            .unwrap()
            .join();
    }

    

}

pub fn is_on_ryujinx() -> bool {
    unsafe { // Ryujinx skip based on text addr
        let text_addr = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        if text_addr == 0x8004000 {
            println!("we are on Ryujinx");
            return true;
        } else {
            println!("we are not on Ryujinx");
            return false;
        }
    }
}

#[cfg(feature = "main_nro")]
pub fn quick_validate_install() {
    let has_smashline_hook = Path::new("sd:/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/libsmashline_hook.nro").is_file();
    if has_smashline_hook {
        println!("libsmashline_hook.nro is present");
    } else {
        if is_on_ryujinx() {
            println!("No libsmashline_hook.nro found! We will likely crash.");
        } else {
            skyline_web::DialogOk::ok("No libsmashline_hook.nro found! We will likely crash.");
        }
    }

    let has_arcropolis_nro = Path::new("sd:/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/libarcropolis.nro").is_file();
    if has_arcropolis_nro {
        println!("libarcropolis.nro is present");
    } else {
        if is_on_ryujinx() {
            println!("No libarcropolis.nro found! We will either crash, or game functionality will be broken.");
        } else {
            skyline_web::DialogOk::ok("No libarcropolis.nro found! We will either crash, or game functionality will be broken.");
        }
    }

    
    let has_nro_hook = Path::new("sd:/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/libnro_hook.nro").is_file();
    if has_nro_hook {
        println!("libnro_hook.nro is present");
    } else {
        if is_on_ryujinx() {
            println!("No libnro_hook.nro found! We will likely crash.");
        } else {
            skyline_web::DialogOk::ok("No libnro_hook.nro found! We will likely crash.");
        }
    }

    let has_smashline_development_hook = Path::new("sd:/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/libsmashline_hook_development.nro").is_file();
    if has_smashline_development_hook {
        if is_on_ryujinx() {
            println!("libsmashline_hook_development.nro found! This will conflict with hdr! Expect a crash soon.");
        } else {
            let should_delete = skyline_web::Dialog::yes_no("libsmashline_hook_development.nro found! This will conflict with hdr! Would you like to delete it?");
            if should_delete {
                fs::remove_file("sd:/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/libsmashline_hook_development.nro");
                unsafe {
                    skyline::nn::oe::RequestToRelaunchApplication();
                }
            } else {
                skyline_web::DialogOk::ok("Warning, we will likely crash soon because of this conflict.");
            }
        }
    }

    let has_development_nro = Path::new("sd:/atmosphere/contents/01006a800016e000/romfs/smashline/development.nro").is_file();
    let has_dev_folder = Path::new("sd:/ultimate/mods/hdr-dev/").is_dir();
    if has_development_nro && !has_dev_folder {
        if is_on_ryujinx() {
            println!("development.nro found, but there is no hdr-dev folder! This is likely a mistake.");
        } else {
            let should_delete = skyline_web::Dialog::yes_no("development.nro found, but there is no hdr-dev folder! This is likely a mistake. Would you like to delete it?");
            if should_delete {
                fs::remove_file("sd:/atmosphere/contents/01006a800016e000/romfs/smashline/development.nro");
                unsafe {
                    skyline::nn::oe::RequestToRelaunchApplication();
                }
            } 
        }
    }
    

    let has_stale_hdr = Path::new("sd:/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/libhdr.nro").is_file();
    if has_stale_hdr {
        if is_on_ryujinx() {
            println!("stale libhdr.nro found! This will conflict with your newer hdr! Expect a crash soon.");
        } else {
            let should_delete = skyline_web::Dialog::yes_no("Stale libhdr.nro found in atmos/contents! This will conflict with new hdr packaging! Would you like to delete it?");
            if should_delete {
                fs::remove_file("sd:/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/libhdr.nro");
                unsafe {
                    skyline::nn::oe::RequestToRelaunchApplication();
                }
            } else {
                skyline_web::DialogOk::ok("Warning, we will likely crash soon or have undefined behavior because of this conflict.");
            }
        }
    }

    let has_hdr_assets = Path::new("sd:/ultimate/mods/hdr-assets/").is_dir();
    if has_hdr_assets {
        println!("hdr-assets are present");
    } else {
        if is_on_ryujinx() {
            println!("No hdr-assets found! This installation is incomplete. Please install the full package.");
        } else {
            skyline_web::DialogOk::ok("No hdr-assets found! This installation is incomplete. Please install the full package.");
        }
    }

    let has_hdr_stages = Path::new("sd:/ultimate/mods/hdr-stages/").is_dir();
    if has_hdr_stages {
        println!("hdr-stages are present");
    } else {
        if is_on_ryujinx() {
            println!("No hdr-stages found! This installation is incomplete. Please install the full package.");
        } else {
            skyline_web::DialogOk::ok("No hdr-stages found! This installation is incomplete. Please install the full package.");
        }
    }

    println!("simple validation complete.");

}