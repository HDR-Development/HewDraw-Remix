use crate::opff_import::*;
use smash::app::BattleObjectModuleAccessor;
use smash::phx::{Vector2f, Vector3f};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::hash40;

pub unsafe fn reset_all_magic_flags(boma: &mut BattleObjectModuleAccessor) {
    let id = hdr::get_player_number(boma);
    jab_da_checks[id] = false;
    tilt_checks[id] = false;
    smash_checks[id] = false;
    aerial_checks[id] = false;
}

pub unsafe fn set_all_magic_flags(boma: &mut BattleObjectModuleAccessor) {
    let id = hdr::get_player_number(boma);
    jab_da_checks[id] = true;
    tilt_checks[id] = true;
    smash_checks[id] = true;
    aerial_checks[id] = true;
}

pub unsafe fn reset_magic_flag(boma: &mut BattleObjectModuleAccessor, magic_level: i32) {
    let id = hdr::get_player_number(boma);
    match magic_level {
        1 => {jab_da_checks[id] = false;}
        2 => {tilt_checks[id] = false;}
        3 => {smash_checks[id] = false;}
        4 => {aerial_checks[id] = false;}
        _ => {reset_all_magic_flags(boma);}
    };
}

pub unsafe fn set_magic_flag(boma: &mut BattleObjectModuleAccessor, magic_level: i32) {
    let id = hdr::get_player_number(boma);
    match magic_level {
        1 => {jab_da_checks[id] = true;}
        2 => {tilt_checks[id] = true;}
        3 => {smash_checks[id] = true;}
        4 => {aerial_checks[id] = true;}
        _ => {set_all_magic_flags(boma);}
    };
}

pub unsafe fn run(boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {

}
