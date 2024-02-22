use smash::app::{BattleObject, BattleObjectModuleAccessor};
use smash::lua2cpp::L2CFighterCommon;
use crate::offsets;
use crate::ext::*;
use std::arch::asm;
use smash::phx::Vector2f;
use crate::se;

#[macro_export]
macro_rules! dump_trace {
    () => {{
        let text = ::skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        ::utils::dump_trace!(text)
    }};
    ($base:expr) => {{
        const MAXIMUM_BT_LEN: usize = 0x20;
        let text = $base;
        println!("Current text address: {:#x}", text);

        let mut lr: *const u64;
        unsafe {
            asm!("mov {}, x30", out(reg) lr);
        }

        let mut fp: *const u64;
        unsafe {
            asm!("mov {}, x29", out(reg) fp);
        }

        println!("Current LR:\t\t{:#x} ({:#x})", (lr as u64) - text, (lr as u64));

        let mut counter = 0usize;
        while !fp.is_null() && counter < MAXIMUM_BT_LEN {
            lr = *fp.offset(1) as *const u64;
            if !lr.is_null() {
                println!("[{}]: {:#x} ({:#x})", counter, (lr as u64), (lr as u64) - text);
                counter += 1;
            }
            fp = *fp as *const u64;
        }
    }}
}

#[macro_export]
macro_rules! c_str {
    ($l:tt) => {
        [$l.as_bytes(), "\u{0}".as_bytes()].concat().as_ptr()
    }
}

pub fn byte_search<T: Eq>(needle: &[T]) -> Option<usize> {   
    let text = unsafe {
        let start = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const T;
        let end = skyline::hooks::getRegionAddress(skyline::hooks::Region::Rodata) as *const T;
        let length = end.offset_from(start) as usize;
        std::slice::from_raw_parts(start, length)
    };

    text.windows(needle.len()).position(|window| window == needle)
}

pub fn byte_search_rodata<T: Eq>(needle: &[T]) -> Option<usize> {
    const RODATA_LEN: usize = 0xCC8C9B;
    let (rodata, text_len) = unsafe {
        let start = skyline::hooks::getRegionAddress(skyline::hooks::Region::Rodata) as *const T;
        let end = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Rodata) as usize + RODATA_LEN) as *const T;
        let text = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const T;
        let length = end.offset_from(start) as usize;
        (std::slice::from_raw_parts(start, length), start.offset_from(text) as usize)
    };

    rodata.windows(needle.len()).position(|window| window == needle).map(|x| x + text_len)
}

pub fn offset_to_addr<T>(offset: usize) -> *const T {
    unsafe {
        (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const u8).add(offset) as _
    }
}

pub fn get_match_mode() -> (u32, u32) {
    #[skyline::from_offset(offsets::get_match_mode())]
    fn get_mode_internal(main: &mut u32, sub: &mut u32);

    let mut main = 0u32;
    let mut sub = 0u32;
    unsafe {
        get_mode_internal(&mut main, &mut sub);
    }
    (main, sub)
}

pub fn get_global_frame_count() -> usize {
    unsafe {
        *offset_to_addr::<usize>(offsets::global_frame_counter())
    }
}

#[skyline::from_offset(offsets::get_battle_object_from_id())]
pub fn get_battle_object_from_id(id: u32) -> *mut BattleObject;

pub fn get_battle_object_from_accessor(boma: *mut BattleObjectModuleAccessor) -> *mut BattleObject {
    unsafe {
        get_battle_object_from_id((*boma).battle_object_id)
    }
}

pub fn get_fighter_common_from_accessor<'a>(boma: &'a mut BattleObjectModuleAccessor) -> &'a mut L2CFighterCommon {
    unsafe {
        let lua_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x190 / 8);
        std::mem::transmute(*((lua_module + 0x1D8) as *mut *mut L2CFighterCommon))
    }
}

pub fn get_fighter_common_from_entry_id(entry_id: u32) -> Option<&'static mut L2CFighterCommon> {
    if let Some(object) = get_battle_object_from_entry_id(entry_id) {
        unsafe {
            Some(get_fighter_common_from_accessor(std::mem::transmute((*object).module_accessor)))
        }
    } else {
        None
    }
}

pub fn get_lua_state_from_entry_id(entry_id: u32) -> Option<u64> {
    get_fighter_common_from_entry_id(entry_id).map(|x| x.lua_state_agent)
}

pub fn get_active_battle_object_id_from_entry_id(entry_id: u32) -> Option<u32> {
    use smash::lib::lua_const::*;
    use smash::app::lua_bind::*;
    let object = get_battle_object_from_entry_id(entry_id)?;
    if object.is_null() { return None; }
    let object = unsafe { &mut *object };
    let kind = object.kind as i32;
    let status = unsafe {
        StatusModule::status_kind(object.module_accessor)
    };
    if status != *FIGHTER_STATUS_KIND_NONE && status != *FIGHTER_STATUS_KIND_STANDBY {
        return Some(object.battle_object_id);
    }
    if kind == *FIGHTER_KIND_ELIGHT || kind == *FIGHTER_KIND_EFLAME {
        Some(object.battle_object_id + 0x10000)
    } else if kind == *FIGHTER_KIND_PZENIGAME || kind == *FIGHTER_KIND_PFUSHIGISOU || kind == *FIGHTER_KIND_PLIZARDON {
        let next_id = object.battle_object_id + 0x10000;
        let next_object = unsafe { get_battle_object_from_id(next_id) };
        if !next_object.is_null() {
            let next_object = unsafe { &mut *next_object };
            let next_status = unsafe {
                StatusModule::status_kind(next_object.module_accessor)
            };
            if next_status != *FIGHTER_STATUS_KIND_NONE && next_status != *FIGHTER_STATUS_KIND_STANDBY {
                Some(next_id)
            } else {
                Some(next_id + 0x10000)
            }
        }
        else {
            Some(object.battle_object_id)
        }
    } else {
        Some(object.battle_object_id)
    }
}

/// This gets ALL active battle object IDs, including both Ice Climbers,
/// and only the ACTIVE character of Pokemon Trainer and Aegis.
pub unsafe fn get_all_active_battle_object_ids() -> Vec<u32> {
    use smash::lib::lua_const::*;
    use smash::app::lua_bind::*;
    use super::ext::*;
    let mut vec: Vec<u32> = Vec::new();
    for entry_id in 0..8 {
        // get the active battle object id and add it to the list
        let id = get_active_battle_object_id_from_entry_id(entry_id).unwrap_or(*BATTLE_OBJECT_ID_INVALID as u32);
        vec.push(id);

        // from here on out, we are doing this to account for both ice climbers

        // get the object back from the id
        let object = get_battle_object_from_id(id);
        if object.is_null() { continue; }
        let object = unsafe { &mut *object };

        // get the fighter kind - check if it is popo
        let kind = object.kind as i32;
        if kind != *FIGHTER_KIND_POPO { continue; }

        // if it is popo, get nana and add her to the list too
        let boma = &mut *(*object).module_accessor;
        let nana_id = WorkModule::get_int(boma, *FIGHTER_POPO_INSTANCE_WORK_ID_INT_PARTNER_OBJECT_ID) as u32;
        let nana_object = get_battle_object_from_id(nana_id);
        if nana_object.is_null() { continue; }
        let nana_object = unsafe { &mut *nana_object };
        vec.push(nana_object.battle_object_id);
    }
    return vec;
}

extern "C" {
    #[link_name = "\u{1}_ZN3app8lua_bind38FighterManager__get_fighter_entry_implEPNS_14FighterManagerENS_14FighterEntryIDE"]
    fn get_fighter_entry(manager: *mut smash::app::FighterManager, entry_id: u32) -> *mut u8;
}

pub fn get_battle_object_from_entry_id(entry_id: u32) -> Option<*mut BattleObject> {
    unsafe {
        let entry = get_fighter_entry(super::singletons::FighterManager(), entry_id);
        if entry.is_null() {
            None
        } else {
            Some(*(entry.add(0x4160) as *mut *mut BattleObject))
        }
    }
}

/// Only pulls the game state to perform actions on
pub fn get_game_state() -> *const u64 {
    unsafe {
        let p_p_p_game_state = *offset_to_addr::<*const *const *const u64>(offsets::p_p_game_state());
        if p_p_p_game_state.is_null() {
            return std::ptr::null();
        }
        let p_p_game_state = *p_p_p_game_state;
        if p_p_game_state.is_null() {
            return std::ptr::null();
        }
        let p_game_state = *p_p_game_state;
        if p_game_state.is_null() {
            return std::ptr::null();
        }
        p_game_state
    }
}

pub unsafe fn get_mapped_controller_inputs_from_id(player: usize) -> &'static MappedInputs {
    let base = *((skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8)
        .add(0x52c50f0) as *const u64);
    &*((base + 0x2b8 + 0x8 * (player as u64)) as *const MappedInputs)
}

pub unsafe fn get_controller_mapping_from_id(player: usize) -> &'static ControllerMapping {
    let base = *((skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8)
        .add(0x52c50f0) as *const u64);
    &*((base + 0x18) as *const ControllerMapping).add(player as usize)
}

#[repr(C)]
struct SomeControllerStruct {
    padding: [u8; 0x10],
    controller: &'static mut Controller,
}

pub unsafe fn get_controller_from_id(player: usize) -> &'static Controller {
    let base = *((skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8)
        .add(0x5339860) as *const u64);
    let uVar3 = *((base + 0x298 + (4 * (player as u64))) as *const u32);
    let controller_struct = ((base + (0x8 * (uVar3 as i32)) as u64) as *mut SomeControllerStruct);
    (*controller_struct).controller
}

/// Triggers a match exit (all the way back to the stage select screen) by entering into the `StateExit` game state.
/// Note: Calling this function otuside of a match shouldn't crash but it has undefined behavior. If you do that, don't
pub fn trigger_match_exit() {
    unsafe {
        let p_game_state = get_game_state();
        if p_game_state.is_null() {
            return;
        }
        // Finally call the vtable function on the game state
        let vtable_func: extern "C" fn(*const u64) = std::mem::transmute(*(*p_game_state as *const u64).add(0x3));
        vtable_func(p_game_state);
    }
}

/// Triggers a match reset by loading into the same state that classic mode uses when you retry a game
/// Note: Calling this function outside of a match shouldn't crash but it has undefined behavior. If you do that, don't
pub fn trigger_match_reset() {
    unsafe {
        let p_game_state = get_game_state();
        if p_game_state.is_null() {
            return;
        }
        // Finally call the vtable function on the game state
        let vtable_func: extern "C" fn(*const u64) = std::mem::transmute(*(*p_game_state as *const u64).add(0x5));
        vtable_func(p_game_state);
    }
}

/// Utility function to compare two masks, such as a "cat flag"
/// Internally, its just a binary & operation. When possible, use 
/// fighter.is_cat_flag() or boma.is_cat_flag() instead, for clarity.
/// 
/// # Arguments:
/// * `mask1` : a bitmask, such as a cat flag mask (cat1, cat2, etc)\n
/// * `mask2` : another bitmask, such as `*FIGHTER_KIND_MARIO`
/// 
/// # Returns:
/// `bool` : whether the binary & of the masks is nonzero (aka "true")
pub fn compare_mask(mask1: i32, mask2: i32) -> bool {
    return (mask1 & mask2) != 0;
}

/// get a vector3f where the given value is the strength and stick X is used to determine
/// the direction of the vector in the x direction. y and z are always zero.
/// 
#[inline(always)]
pub unsafe fn x_motion_vec(val: f32, stick_x: f32) -> smash::phx::Vector3f {
    smash::phx::Vector3f{x: val * stick_x.signum(), y: 0.0, z: 0.0}
}

extern "C"{
    /// gets whether we are in training mode
    #[link_name = "\u{1}_ZN3app9smashball16is_training_modeEv"]
    pub fn is_training_mode() -> bool;
}

extern "C" {
    #[link_name = "_ZN3app13sv_debug_draw11draw_circleERKN3phx8Vector2fEfi"]
    pub fn debug_draw_circle(center: &Vector2f, radius: f32, num_frames: i32);
   #[link_name = "_ZN3app13sv_debug_draw9draw_lineERKN3phx8Vector2fES4_i"]
    pub fn debug_draw_line(a: &Vector2f, b: &Vector2f, num_frames: i32);
    #[link_name = "_ZN3app13sv_debug_draw14set_draw_colorEffff"]
    pub fn debug_set_draw_color(r: f32, g: f32, b: f32, a: f32);
}
