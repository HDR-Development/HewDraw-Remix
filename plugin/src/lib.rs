#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]
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
use std::{fs, path::Path};

#[cfg(feature = "updater")]
mod updater;

#[smashline::installer]
pub fn install() {
    fighters::install();
}

#[cfg(not(feature = "main_nro"))]
#[export_name = "hdr_delayed_install"]
pub fn delayed_install() {
    fighters::delayed_install();
}

#[cfg(feature = "add_status")]
extern "Rust" {
    #[link_name = "hdr_delayed_install"]
    fn delayed_install();
}

#[cfg(feature = "main_nro")]
#[export_name = "hdr_is_available"]
pub fn is_available() -> bool {
    true
}

pub fn is_on_ryujinx() -> bool {
    unsafe {
        // Ryujinx skip based on text addr
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
use once_cell::sync::OnceCell;

/// gets the currently loaded assets version string for display
#[cfg(feature = "main_nro")]
pub fn get_romfs_version() -> &'static String {
    static INSTANCE: OnceCell<String> = OnceCell::new();
    INSTANCE.get_or_init(
        || match std::fs::read_to_string("mods:/ui/romfs_version.txt") {
            Ok(version_value) => version_value.trim().to_string(),
            Err(_) => String::from("UNKNOWN"),
        },
    )
}

/// gets the main plugin version string for display
#[cfg(feature = "main_nro")]
pub fn get_plugin_version() -> &'static String {
    static INSTANCE: OnceCell<String> = OnceCell::new();
    INSTANCE.get_or_init(
        || match std::fs::read_to_string("mods:/ui/hdr_version.txt") {
            Ok(version_value) => version_value.trim().to_string(),
            Err(_) => String::from("UNKNOWN"),
        },
    )
}

extern "C" {
    fn change_version_string(arg: u64, string: *const c_char);
}

#[cfg(feature = "main_nro")]
#[skyline::hook(replace = change_version_string)]
fn change_version_string_hook(arg: u64, string: *const c_char) {
    runtime_motion_patcher::run(true);
    let original_str = unsafe { skyline::from_c_str(string) };
    if original_str.contains("Ver.") {
        let romfs_version = get_romfs_version();
        let hdr_version = match std::fs::read_to_string("mods:/ui/hdr_version.txt") {
            Ok(version_value) => version_value.trim().to_string(),
            Err(_) => {
                #[cfg(feature = "main_nro")]
                if !is_on_ryujinx() {
                    skyline_web::DialogOk::ok(
                        "hdr-assets is not enabled! Please enable hdr-assets in arcropolis config.",
                    );
                }

                String::from("UNKNOWN")
            }
        };
        let new_str = format!(
            "{}\nHDR Ver. {}\nAssets Ver. {}\0",
            original_str, hdr_version, romfs_version
        );

        call_original!(arg, skyline::c_str(&new_str))
    } else {
        call_original!(arg, string)
    }
}

#[skyline::from_offset(0x23ecb70)]
unsafe fn music_function1(arg: u64);

#[skyline::from_offset(0x23ed420)]
unsafe fn music_function2(arg: u64, arg2: u64);

#[skyline::hook(offset = 0x14f97bc, inline)]
unsafe fn training_reset_music2(ctx: &skyline::hooks::InlineCtx) {
    if !smash::app::smashball::is_training_mode() {
        music_function2(*ctx.registers[0].x.as_ref(), *ctx.registers[1].x.as_ref());
    }
}

#[skyline::hook(offset = 0x1509dc4, inline)]
unsafe fn training_reset_music1(ctx: &skyline::hooks::InlineCtx) {
    if !smash::app::smashball::is_training_mode() {
        music_function1(*ctx.registers[0].x.as_ref());
    }
}

#[skyline::hook(offset = 0x235be30, inline)]
unsafe fn main_menu_quick(ctx: &skyline::hooks::InlineCtx) {
    let sp = (ctx as *const skyline::hooks::InlineCtx as *mut u8).add(0x100);
    *(sp.add(0x60) as *mut u64) = 0x1100000000;
    let mut slice = std::slice::from_raw_parts_mut(sp.add(0x68), 18);
    slice.copy_from_slice(b"MenuSequenceScene\0");
    let mode = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64 + 0x53030f0)
        as *const u64;
    // if we are in the controls menu mode, there is no ui overlay, so dont update the hud
    println!("{:#x}", *mode);
}

#[skyline::from_offset(0x353f4d0)]
fn load_file_by_hash40(tables: u64, hash: u64);

#[skyline::hook(offset = 0x1864310, inline)]
unsafe fn title_screen_play(_: &skyline::hooks::InlineCtx) {
    let tables = *((skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const u8)
        .add(0x5330f20) as *const u64);
    load_file_by_hash40(
        tables,
        smash::hash40("ui/layout/menu/main_menu/main_menu/layout.arc"),
    );
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

use skyline::hooks::InlineCtx;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
#[skyline::from_offset(0x15433b0)]
unsafe fn ask_game_state_nicely(arg: *mut u64, game_state: u64, hash: u64);
#[skyline::from_offset(0x135a0d0)]
unsafe fn push_something_base(addr: u64);
#[skyline::from_offset(0x135ac60)]
unsafe fn push_hash_base(addr: u64);
unsafe fn push_something(game_state: u64, amt: u32) {
    let game_state = game_state as *mut u64;
    let mut val1 = *game_state.add(0x100 / 8);
    let mut val2 = *game_state.add(0x108 / 8);
    let mut val3 = if val2 - val1 != 0 {
        (val2 - val1) * 0x20 - 1
    } else {
        0
    };

    let mut val4 = *game_state.add(0x120 / 8) + *game_state.add(0x118 / 8);
    if val4 == val3 {
        push_something_base(game_state as u64 + 0xf8);
        val4 = *game_state.add(0x120 / 8) + *game_state.add(0x118 / 8);
        val1 = *game_state.add(0x100 / 8);
        val2 = *game_state.add(0x108 / 8);
    }
    let addr = *((val1 + (val4 >> 5 & !0x7)) as *mut u64) + (val4 & 0xFF) * 0x10;
    *(addr as *mut u32) = amt;
    *(addr as *mut u64).add(1) = 0xFFFFFFFF_FFFFFFFF;
    *game_state.add(0x120 / 8) += 1;
}

unsafe fn push_hash(game_state: u64, hash: u64) {
    let game_state = game_state as *mut u64;
    let mut val1 = *game_state.add(200 / 8);
    let mut val2 = *game_state.add(0xd0 / 8);
    let mut val3 = if val2 - val1 != 0 {
        (val2 - val1) * 0x40 - 1
    } else {
        0
    };
    let mut val4 = *game_state.add(0xe8 / 8) + *game_state.add(0xe0 / 8);
    if val4 == val3 {
        push_hash_base(game_state as u64 + 0xc0);
        val4 = *game_state.add(0xe8 / 8) + *game_state.add(0xe0 / 8);
        val1 = *game_state.add(200 / 8);
    }
    let addr = *((val1 + (val4 >> 6 & !7)) as *mut u64) + (val4 & 0x1FF) * 8;
    *(addr as *mut u64) = hash;
    *game_state.add(0xe8 / 8) += 1;
}

// let this code stay dormant but this is an example of how to abuse the game state,
// this will exit the game without going to results at the end.
#[skyline::hook(offset = 0x14d6570)]
unsafe fn game_end(game_state: u64) {
    let one =
        *(skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x52c31b2);
    let mode = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64 + 0x53030f0)
        as *const u64;
    if one == 0 && *mode != 0x4040000 {
        push_something(game_state, 2);
        // push_hash(game_state, smash::hash40("statewaitforruletofinish"));
        // push_hash(game_state, smash::hash40("statewaitendproduction"));
        push_hash(game_state, smash::hash40("stateapplyparameters"));
        // push_hash(game_state, smash::hash40("statewaitforsyncwhenending"));
        push_hash(game_state, smash::hash40("statefadeoutwhenending"));
        push_hash(game_state, smash::hash40("stateexit"));
        return;
    }
    call_original!(game_state);
}

#[skyline::hook(offset = 0x14d7ed0)]
unsafe fn game_exit(game_state: u64, arg: u64) {
    let one =
        *(skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x52c31b2);
    let mode = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64 + 0x53030f0)
        as *const u64;
    if one == 0 && *mode != 0x4040000 {
        push_something(game_state, 2);
        // push_hash(game_state, smash::hash40("statewaitforruletofinish"));
        // push_hash(game_state, smash::hash40("statewaitendproduction"));
        push_hash(game_state, smash::hash40("stateapplyparameters"));
        // push_hash(game_state, smash::hash40("statewaitforsyncwhenending"));
        push_hash(game_state, smash::hash40("statefadeoutwhenending"));
        push_hash(game_state, smash::hash40("stateexit"));
        return;
    }

    call_original!(game_state, arg);
}

#[repr(C)]
pub struct FuckingAssStringStructureShit {
    pub fuck_if_i_know: u32,
    pub len: u32,
    pub shit_ass_string: [u8; 40],
}

impl FuckingAssStringStructureShit {
    pub fn set(&mut self, replacement: &str) {
        self.len = replacement.len() as u32;
        self.shit_ass_string[..replacement.len()].copy_from_slice(replacement.as_bytes());
        self.shit_ass_string[replacement.len()] = b'\0';
    }
}

#[skyline::hook(offset = 0x2334b58, inline)]
unsafe fn sss_to_css(ctx: &InlineCtx) {
    let thing = *ctx.registers[1].x.as_ref() as *mut FuckingAssStringStructureShit;
    (*thing).set("CharaSelectScene");
}

#[skyline::hook(offset = 0x23344e4, inline)]
unsafe fn css_to_sss(ctx: &InlineCtx) {
    let thing = *ctx.registers[1].x.as_ref() as *mut FuckingAssStringStructureShit;
    (*thing).set("StageSelectScene");
}

#[repr(C)]
pub struct UnknownFighterInfoStruct {
    unk: [u64; 2],
    hash1: u64,
    hash2: u64,
}

static mut IS_LOADING: bool = false;

#[skyline::hook(offset = 0x1785348)]
unsafe fn load_ingame_call_sequence_scene(arg: u64) {
    let mode = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64 + 0x53030f0)
        as *const u64;
    IS_LOADING = *mode != 0x4040000;
    call_original!(arg)
}

#[skyline::hook(offset = 0x1850190)]
unsafe fn load_melee_scene(arg: u64) {
    IS_LOADING = false;
    call_original!(arg);
}

#[skyline::from_offset(0x1742da0)]
unsafe fn check_mode(mode: &mut u32, submode: &mut u32);

#[skyline::hook(offset = 0x16b7f70)]
unsafe fn copy_fighter_info(
    dst: &mut UnknownFighterInfoStruct,
    src: &mut UnknownFighterInfoStruct,
) {
    let one =
        *(skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x52c31b2);
    if src.hash1 & 0xFF_FFFFFFFF == smash::hash40("ui_chara_random") && one == 0 && IS_LOADING {
        dst.hash1 = 0xC1FFFF00_00000000;
        dst.hash2 = 0xC1FFFF00_00000000;
        src.hash1 = 0xC1FFFF00_00000000;
        src.hash2 = 0xC1FFFF00_00000000;
    }
    call_original!(dst, src);
}

#[no_mangle]
pub extern "C" fn main() {
    #[cfg(feature = "main_nro")]
    {
        quick_validate_install();
        skyline::install_hooks!(change_version_string_hook);
        random::install();
        controls::install();
        lua::install();
        online::install();
        skyline::patching::Patch::in_text(0x14f97bc).nop().unwrap();
        skyline::patching::Patch::in_text(0x1509dc4).nop().unwrap();
        skyline::install_hooks!(
            training_reset_music1,
            training_reset_music2,
            main_menu_quick,
            title_screen_play,
            //sss_to_css,
            //css_to_sss,
            //copy_fighter_info,
            //load_ingame_call_sequence_scene,
            //load_melee_scene,
            //game_end,
            //game_exit
        );
        runtime_motion_patcher::install(true);
    }

    #[cfg(not(feature = "runtime"))]
    {
        utils::init();
    }

    #[cfg(feature = "main_nro")]
    {
        if !is_on_ryujinx() {
            setup_hid_hdr();
        }
    }

    fighters::install();
    #[cfg(all(not(feature = "add_status"), feature = "main_nro"))]
    {
        if !(delayed_install as *const ()).is_null() {
            unsafe {
                delayed_install();
            }
        }
    }

    #[cfg(all(
        feature = "add_status",
        not(all(not(feature = "add_status"), feature = "main_nro"))
    ))]
    {
        fighters::delayed_install();
    }

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

#[cfg(feature = "main_nro")]
pub fn setup_hid_hdr() {
    let status = hid_hdr::get_hid_hdr_status().unwrap();
    match status {
        hid_hdr::Status::NotConnected => {
            if !hid_hdr::connect_to_hid_hdr() {
                hid_hdr::warn_unable_to_connect("troubleshooting", "HDR", "discord.gg/hdr");
                return;
            }

            let status = hid_hdr::get_hid_hdr_status().unwrap();
            match status {
                hid_hdr::Status::Ok => {
                    hid_hdr::configure_stick_gate_changes(true).unwrap();
                }
                other => {
                    hid_hdr::warn_status(other, "troubleshooting", "HDR", "discord.gg/hdr");
                }
            }
        }
        hid_hdr::Status::Ok => {
            panic!("Should not be possible yet");
        }
        other => {
            hid_hdr::warn_status(other, "troubleshooting", "HDR", "discord.gg/hdr");
        }
    }
}

#[cfg(feature = "main_nro")]
pub fn quick_validate_install() {
    let has_smashline_hook = Path::new(
        "sd:/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/libsmashline_hook.nro",
    )
    .is_file();
    if has_smashline_hook {
        println!("libsmashline_hook.nro is present");
    } else {
        if is_on_ryujinx() {
            println!("No libsmashline_hook.nro found! We will likely crash.");
        } else {
            skyline_web::DialogOk::ok("No libsmashline_hook.nro found! We will likely crash.");
        }
    }

    let has_arcropolis_nro = Path::new(
        "sd:/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/libarcropolis.nro",
    )
    .is_file();
    if has_arcropolis_nro {
        println!("libarcropolis.nro is present");
    } else {
        if is_on_ryujinx() {
            println!("No libarcropolis.nro found! We will either crash, or game functionality will be broken.");
        } else {
            skyline_web::DialogOk::ok("No libarcropolis.nro found! We will either crash, or game functionality will be broken.");
        }
    }

    let has_nro_hook =
        Path::new("sd:/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/libnro_hook.nro")
            .is_file();
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
                skyline_web::DialogOk::ok(
                    "Warning, we will likely crash soon because of this conflict.",
                );
            }
        }
    }

    let has_development_nro =
        Path::new("sd:/atmosphere/contents/01006a800016e000/romfs/smashline/development.nro")
            .is_file();
    let has_dev_folder = Path::new("sd:/ultimate/mods/hdr-dev/").is_dir();
    if has_development_nro && !has_dev_folder {
        if is_on_ryujinx() {
            println!(
                "development.nro found, but there is no hdr-dev folder! This is likely a mistake."
            );
        } else {
            let should_delete = skyline_web::Dialog::yes_no("development.nro found, but there is no hdr-dev folder! This is likely a mistake. Would you like to delete it?");
            if should_delete {
                fs::remove_file(
                    "sd:/atmosphere/contents/01006a800016e000/romfs/smashline/development.nro",
                );
                unsafe {
                    skyline::nn::oe::RequestToRelaunchApplication();
                }
            }
        }
    }

    let has_stale_hdr =
        Path::new("sd:/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/libhdr.nro")
            .is_file();
    if has_stale_hdr {
        if is_on_ryujinx() {
            println!("stale libhdr.nro found! This will conflict with your newer hdr! Expect a crash soon.");
        } else {
            let should_delete = skyline_web::Dialog::yes_no("Stale libhdr.nro found in atmos/contents! This will conflict with new hdr packaging! Would you like to delete it?");
            if should_delete {
                fs::remove_file(
                    "sd:/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/libhdr.nro",
                );
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
