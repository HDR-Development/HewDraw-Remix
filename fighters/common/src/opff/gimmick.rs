use crate::opff_import::*;
use smash::app::BattleObjectModuleAccessor;
use smash::phx::{Vector2f, Vector3f, Vector4f};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::hash40;

pub unsafe fn gimmick_flash(boma: &mut BattleObjectModuleAccessor) {
    let cbm_vec1 = Vector4f{x: 1.0, y: 0.8, z: 0.35, w: 0.25};
    let cbm_vec2 = Vector4f{x: 1.0, y: 0.8, z: 0.35, w: 0.00};
    ColorBlendModule::set_main_color(boma, &cbm_vec1, &cbm_vec2, 0.5, 0.5, 5, true);

    gimmick_ready_glow_timer[hdr::get_player_number(boma)] = 1;
}

pub unsafe fn gimmick_ready_glow_timer_counting(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    let id = hdr::get_player_number(boma);
    if gimmick_ready_glow_timer[id] > 0 && gimmick_ready_glow_timer[id] < 121 { // 250/5 = 50F
        if gimmick_ready_glow_timer[id] > 119 {
            ColorBlendModule::cancel_main_color(boma, 0);
        } else {
            gimmick_ready_glow_timer[id] += 1;
        }
    }
    if [*FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind) {
        gimmick_ready_glow_timer[id] = 0;
    }
}

pub unsafe fn run(boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    gimmick_ready_glow_timer_counting(boma, status_kind);
}
