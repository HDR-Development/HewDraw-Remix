#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unexpected_cfgs)]
#![allow(static_mut_refs)]
#![feature(proc_macro_hygiene)]

// Not sure why these are needed here, probably cargo-skyline BS
// - blujay
extern crate dynamic;
extern crate locks;
extern crate once_cell;
extern crate rand;
extern crate skyline;
#[cfg(feature = "skyline-web")]
extern crate skyline_web;
extern crate smash;
extern crate smash2;
extern crate smash_arc;
extern crate ninput;
extern crate toml;

mod fighters;

#[cfg(feature = "main_nro")]
mod chara_select;

#[cfg(feature = "main_nro")]
mod controls;

#[cfg(feature = "main_nro")]
mod lua;

#[cfg(feature = "main_nro")]
mod online;

#[cfg(feature = "main_nro")]
mod matchup;

pub mod vsync;

use skyline::libc::c_char;
use std::os::raw::c_void;
#[cfg(feature = "main_nro")]
use skyline_web::*;
use std::{fs, path::Path};
use utils::STAGE_MANAGER;
use std::sync::atomic::Ordering;
use dynamic::util::MATCH_EXITING;

#[cfg(not(feature = "main_nro"))]
#[no_mangle]
pub fn smashline_install() {
    fighters::install();
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
        if text_addr == 0x8504000 || text_addr == 0x80004000 {
            println!("we are on Emulator");
            return true;
        } else {
            println!("we are not on Emulator");
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
    unsafe {
        static mut DID_INIT: bool = false;
        if !DID_INIT {
            DID_INIT = true;
            runtime_motion_patcher::run(true);
        }
    }
    let original_str = unsafe { skyline::from_c_str(string) };
    if original_str.contains("Ver.") {
        let romfs_version = get_romfs_version();
        let hdr_version = match std::fs::read_to_string("mods:/ui/hdr_version.txt") {
            Ok(version_value) => version_value.trim().to_string(),
            Err(_) => {
                #[cfg(feature = "main_nro")]
                if !is_on_ryujinx() {
                    skyline_web::dialog_ok::DialogOk::ok(
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

#[skyline::from_offset(0x23ed810)]
unsafe fn music_function1(arg: u64);

#[skyline::from_offset(0x23ee0c0)]
unsafe fn music_function2(arg: u64, arg2: u64);

#[skyline::hook(offset = 0x14f99cc, inline)]
unsafe fn training_reset_music2(ctx: &skyline::hooks::InlineCtx) {
    if !smash::app::smashball::is_training_mode() {
        music_function2(ctx.registers[0].x(), ctx.registers[1].x());
    }
}

#[skyline::hook(offset = 0x1509fd4, inline)]
unsafe fn training_reset_music1(ctx: &skyline::hooks::InlineCtx) {
    if !smash::app::smashball::is_training_mode() {
        music_function1(ctx.registers[0].x());
    }
}

#[skyline::hook(offset = 0x235cad0, inline)]
unsafe fn main_menu_quick(ctx: &skyline::hooks::InlineCtx) {
    let sp = ctx.sp.x() as *mut u8;
    *(sp.add(0x60) as *mut u64) = 0x1100000000;
    let mut slice = std::slice::from_raw_parts_mut(sp.add(0x68), 18);
    slice.copy_from_slice(b"MenuSequenceScene\0");
    let mode = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64 + 0x53040f0)
        as *const u64;
    // if we are in the controls menu mode, there is no ui overlay, so dont update the hud
    println!("{:#x}", *mode);
}

#[skyline::from_offset(0x353ff00)]
fn load_file_by_hash40(tables: u64, hash: u64);

#[skyline::hook(offset = 0x1864e00, inline)]
unsafe fn title_screen_play(_: &skyline::hooks::InlineCtx) {
    let tables = *((skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const u8)
        .add(0x5331f20) as *const u64);
    load_file_by_hash40(
        tables,
        smash::hash40("ui/layout/menu/main_menu/main_menu/layout.arc"),
    );
}

use skyline::hooks::InlineCtx;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
#[skyline::from_offset(0x15435c0)]
unsafe fn ask_game_state_nicely(arg: *mut u64, game_state: u64, hash: u64);
#[skyline::from_offset(0x135a0f0)]
unsafe fn push_something_base(addr: u64);
#[skyline::from_offset(0x135ac80)]
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

// game_end handles the normal end of game flow
#[skyline::hook(offset = 0x14d6590)]
unsafe fn game_end(game_state: u64) {
    if utils::one_player::one_player_entry() {
        skip_results(game_state);
        return;
    }
    call_original!(game_state);
}

// game_exit handles player-initiated end of game flow
#[skyline::hook(offset = 0x14d7ef0)]
unsafe fn game_exit(game_state: u64, arg: u64) {
    if utils::one_player::one_player_entry() {
        skip_results(game_state);
        return;
    }
    call_original!(game_state, arg);
}

// Skips the results screen at the end of a game
unsafe fn skip_results(game_state: u64) {
    push_something(game_state, 2);
    // push_hash(game_state, smash::hash40("statewaitforruletofinish"));
    // push_hash(game_state, smash::hash40("statewaitendproduction"));
    push_hash(game_state, smash::hash40("stateapplyparameters"));
    // push_hash(game_state, smash::hash40("statewaitforsyncwhenending"));
    push_hash(game_state, smash::hash40("statefadeoutwhenending"));
    push_hash(game_state, smash::hash40("stateexit"));
}

impl HashedString {
    pub fn set(&mut self, replacement: &str) {
        self.length = replacement.len() as u32;
        self.string[..replacement.len()].copy_from_slice(replacement.as_bytes());
        self.string[replacement.len()] = b'\0';
    }

    pub fn as_str(&self) -> &str {
        let len = self.string.iter().position(|&c| c == 0).unwrap_or(self.string.len());
        std::str::from_utf8(&self.string[..len]).unwrap_or("")
    }
}

pub static mut CSS_FIRST: bool = false;
pub static mut SSS_CANCEL_TO_CSS: bool = false;
pub static mut CSS_CANCEL_TO_LOCAL: bool = false;
pub static mut IN_LOCAL_WIRELESS: bool = false;

#[skyline::hook(offset = 0x23357f8, inline)]
unsafe fn sss_to_css(ctx: &InlineCtx) {
    let hashed_string = ctx.registers[1].x() as *mut HashedString;
    let current_scene = (*hashed_string).as_str();

    if current_scene == "StageSelectScene" {
        CSS_FIRST = true;
        (*hashed_string).set("CharaSelectScene");
        CSS_CANCEL_TO_LOCAL = IN_LOCAL_WIRELESS;
    }
}

#[skyline::hook(offset = 0x2335184, inline)]
unsafe fn css_to_sss(ctx: &InlineCtx) {
    let hashed_string = ctx.registers[1].x() as *mut HashedString;
    let current_scene = (*hashed_string).as_str();

    if current_scene == "CharaSelectScene" {
        let text = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        let flag_ptr = (text + 0x530996c) as *const u8;

        let flag = *flag_ptr.add(3);

        // This byte is which stage select mode (anyone, take turns, etc) the game is in
        // The SSS is shown for modes 0, 1, and 2 but not for 3 or 4,
        // therefore, only redirect to SSS if the flag is <= 2
        if flag <= 2 {
            (*hashed_string).set("StageSelectScene");
            SSS_CANCEL_TO_CSS = IN_LOCAL_WIRELESS;
        }
        else {
            CSS_FIRST = false;
        }
    }
}

#[repr(C)]
pub struct HashedString {
    pub length: u32,
    pub hash: u32,
    pub string: [u8; 64],
}

#[repr(C)]
pub struct UnknownFighterInfoStruct {
    unk: [u64; 2],
    hash1: u64,
    hash2: u64,
}

static mut IS_LOADING: bool = false;

// OFFSET IS WRONG, IT'S FROM 13.0.1 but it don't fuckin work
#[skyline::hook(offset = 0x1785348)]
unsafe fn load_ingame_call_sequence_scene(arg: u64) {
    let mode = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64 + 0x53040f0)
        as *const u64;
    IS_LOADING = *mode != 0x4040000;
    call_original!(arg)
}

#[skyline::hook(offset = 0x1850810)]
unsafe fn load_melee_scene(arg: u64) {
    IS_LOADING = false;
    call_original!(arg);
}

#[skyline::from_offset(0x1743870)]
unsafe fn check_mode(mode: &mut u32, submode: &mut u32);

#[skyline::hook(offset = 0x16b6bb0)]
unsafe fn copy_fighter_info(
    dst: &mut UnknownFighterInfoStruct,
    src: &mut UnknownFighterInfoStruct,
) {
    let one =
        *(skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x52c41b2);
    if src.hash1 & 0xFF_FFFFFFFF == smash::hash40("ui_chara_random") && one == 0 && IS_LOADING {
        dst.hash1 = 0xC1FFFF00_00000000;
        dst.hash2 = 0xC1FFFF00_00000000;
        src.hash1 = 0xC1FFFF00_00000000;
        src.hash2 = 0xC1FFFF00_00000000;
    }
    call_original!(dst, src);
}

// This is a hook on the main scene transition function
// key_str is the scene name
// Add anything requiring a scene transition check here
#[skyline::hook(offset = 0x3726120)]
unsafe fn scene_transition(
    list_ptr: *mut c_void,
    key_struct: *const HashedString,
    context_struct: *const HashedString,
    factory: *mut c_void
) {
    if !key_struct.is_null() {
        let str_ptr = (key_struct as *const u8).add(8) as *const c_char;
        let initial_key = skyline::from_c_str(str_ptr);
        println!("Transitioning to scene: '{}'", initial_key);

        // Catch Local Wireless transitions
        if SSS_CANCEL_TO_CSS && initial_key == "MeleeRuleScene" {
            (*(key_struct as *mut HashedString)).set("CharaSelectScene");
            SSS_CANCEL_TO_CSS = false;
        } else if CSS_CANCEL_TO_LOCAL && initial_key == "MeleeRuleScene" {
            (*(key_struct as *mut HashedString)).set("LocalTopScene");
            CSS_CANCEL_TO_LOCAL = false;
        }

        let key_str = skyline::from_c_str((key_struct as *const u8).add(8) as *const c_char);

        if key_str == "StageSelectScene" {
            STAGE_MANAGER.lock().unwrap().matchup_texture_index = None;
        }

        if key_str == "MeleeRuleScene" || key_str == "MainMenuScene" {
            // Clear perma-strikes when going to main menu or the rules screen
            let mut mgr = STAGE_MANAGER.lock().unwrap();
            mgr.perma_striked_stages.clear();

            // Make sure CSS-first state doesn't carry over to other modes unintentionally
            CSS_FIRST = false;

            // Clear custom game modes so they don't carry over accidentally
            utils::game_modes::reset_custom_mode();
        }

        // Set Local Wireless (because get_match_mode doesn't always work in some scenes)
        if key_str.starts_with("Local") {
            IN_LOCAL_WIRELESS = true;
        } else if key_str == "MainMenuScene" || key_str == "MenuSequenceScene" {
            IN_LOCAL_WIRELESS = false;
        }

        if key_str != "StageSelectScene" && key_str != "CharaSelectScene" && key_str != "MeleeRuleScene" {
            SSS_CANCEL_TO_CSS = false;
            CSS_CANCEL_TO_LOCAL = false;
        }
    }

    MATCH_EXITING.store(false, Ordering::Relaxed);

    call_original!(list_ptr, key_struct, context_struct, factory);
}

extern "C" {
    #[link_name = "_ZN2nn5prepo10PlayReport4SaveERKNS_7account3UidE"]
    fn save_report(uid: *mut u8);
}

#[skyline::hook(replace = save_report)]
fn save_report_stub(uid: *mut u8) { }

#[skyline::main(name = "hdr")]
pub fn main() {
    #[cfg(feature = "main_nro")]
    {
        // vsync::setup_ssbu_sync();
        quick_validate_install();
        skyline::install_hooks!(change_version_string_hook);
        chara_select::install();
        controls::install();
        lua::install();
        online::install();
        matchup::install();
        skyline::patching::Patch::in_text(0x14f99cc).nop().unwrap();
        skyline::patching::Patch::in_text(0x1509fd4).nop().unwrap();
        unlock_menu_music();
        unlock_characters();
        skyline::install_hooks!(
            training_reset_music1,
            training_reset_music2,
            main_menu_quick,
            title_screen_play,
            sss_to_css,
            css_to_sss,
            scene_transition,
            save_report_stub,
            //copy_fighter_info,
            //load_ingame_call_sequence_scene,
            //load_melee_scene,
            game_end,
            game_exit
        );
    }

    #[cfg(not(feature = "runtime"))]
    {
        utils::init();
    }

    // #[cfg(feature = "main_nro")]
    // {
    //     if !is_on_ryujinx() {
    //         setup_hid_hdr();
    //     }
    // }

    fighters::install();

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

// #[cfg(feature = "main_nro")]
// pub fn setup_hid_hdr() {
//     let status = hid_hdr::get_hid_hdr_status().unwrap();
//     match status {
//         hid_hdr::Status::NotConnected => {
//             if !hid_hdr::connect_to_hid_hdr() {
//                 hid_hdr::warn_unable_to_connect("troubleshooting", "HDR", "discord.gg/hdr");
//                 return;
//             }

//             let status = hid_hdr::get_hid_hdr_status().unwrap();
//             match status {
//                 hid_hdr::Status::Ok => {
//                     hid_hdr::configure_stick_gate_changes(true).unwrap();
//                 }
//                 other => {
//                     hid_hdr::warn_status(other, "troubleshooting", "HDR", "discord.gg/hdr");
//                 }
//             }
//         }
//         hid_hdr::Status::Ok => {
//             panic!("Should not be possible yet");
//         }
//         other => {
//             hid_hdr::warn_status(other, "troubleshooting", "HDR", "discord.gg/hdr");
//         }
//     }
// }

#[cfg(feature = "main_nro")]
pub fn quick_validate_install() {
    let has_smashline_plugin = Path::new(
        "sd:/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/libsmashline_plugin.nro",
    )
    .is_file();
    if has_smashline_plugin {
        println!("libsmashline_plugin.nro is present");
    } else {
        if is_on_ryujinx() {
            println!("No libsmashline_plugin.nro found! We will likely crash.");
        } else {
            skyline_web::dialog_ok::DialogOk::ok(
                "No libsmashline_plugin.nro found! We will likely crash.",
            );
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
            skyline_web::dialog_ok::DialogOk::ok("No libarcropolis.nro found! We will either crash, or game functionality will be broken.");
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
            skyline_web::dialog_ok::DialogOk::ok("No libnro_hook.nro found! We will likely crash.");
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
            let should_delete = skyline_web::dialog::Dialog::yes_no("development.nro found, but there is no hdr-dev folder! This is likely a mistake. Would you like to delete it?");
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
            let should_delete = skyline_web::dialog::Dialog::yes_no("Stale libhdr.nro found in atmos/contents! This will conflict with new hdr packaging! Would you like to delete it?");
            if should_delete {
                fs::remove_file(
                    "sd:/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/libhdr.nro",
                );
                unsafe {
                    skyline::nn::oe::RequestToRelaunchApplication();
                }
            } else {
                skyline_web::dialog_ok::DialogOk::ok("Warning, we will likely crash soon or have undefined behavior because of this conflict.");
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
            skyline_web::dialog_ok::DialogOk::ok("No hdr-assets found! This installation is incomplete. Please install the full package.");
        }
    }

    let has_hdr_stages = Path::new("sd:/ultimate/mods/hdr-stages/").is_dir();
    if has_hdr_stages {
        println!("hdr-stages are present");
    } else {
        if is_on_ryujinx() {
            println!("No hdr-stages found! This installation is incomplete. Please install the full package.");
        } else {
            skyline_web::dialog_ok::DialogOk::ok("No hdr-stages found! This installation is incomplete. Please install the full package.");
        }
    }

    println!("simple validation complete.");
}

fn unlock_menu_music() {
    if std::path::Path::new("sd:/ultimate/hdr-config/unlock_menu_music").exists() {
        println!("WARNING: potentially bannable operation in effect!");
        // Patch the My Music UI to always show menu music as selectable
        skyline::patching::Patch::in_text(0x184de0c).nop().unwrap();
        skyline::patching::Patch::in_text(0x184de10).data(0x390bbb9fu32).unwrap();
        // Patch the BGM playback function to always use the player's My Music selection
        // instead of defaulting to the standard menu theme.
        skyline::patching::Patch::in_text(0x3311f94).data(0x320003e8u32).unwrap();
    }
}

#[skyline::hook(offset = 0x1797640)]
unsafe fn fighter_unlock_query(fighter: u64, query_type: u32, arg3: u64) -> u64 {
    // query_type is 0 for regular characters, 4 for DLC,
    // and the logic for DLC characters is always "is purchased? then show it"
    // which always returns true for base characters and owned DLC,
    // but false for unowned DLC.
    let effective_query_type = if query_type == 0 { 4 } else { query_type };
    call_original!(fighter, effective_query_type, arg3)
}

fn unlock_characters() {
    if std::path::Path::new("sd:/ultimate/hdr-config/unlock_characters").exists() {
        println!("unlock_characters flag found - unlocking all non-DLC characters");
        skyline::install_hook!(fighter_unlock_query);
    }
}
