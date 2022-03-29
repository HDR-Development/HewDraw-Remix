// opff import
utils::import_noreturn!(common::opff::{fighter_common_opff, b_reverse});
use super::*;
use globals::*;

 
unsafe fn holy_water_ac_b_rev(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        if frame > 19.0 {
            if boma.is_cat_flag(Cat1::AirEscape) && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
                if situation_kind == *SITUATION_KIND_AIR {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
                }
            }
        }
        common::opff::b_reverse(fighter);
    }
}

// Simon Cross Fast Fall/Land Cancel
unsafe fn cross_ff_land_cancel(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_SIMON_STATUS_KIND_SPECIAL_S2].contains(&status_kind) {
        if situation_kind == *SITUATION_KIND_AIR {
            VarModule::on_flag(boma.object(), vars::common::AIR_CROSS);
            if boma.is_cat_flag(Cat2::FallJump)
                && stick_y < -0.66
                && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
    }
}

// Turn off air_cross flag if not in Cross in the air
unsafe fn air_cross_air_off(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32) {
    if situation_kind == *SITUATION_KIND_AIR {
        if ![*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_SIMON_STATUS_KIND_SPECIAL_S2].contains(&status_kind) {
            if VarModule::is_flag(boma.object(), vars::common::AIR_CROSS) {
                VarModule::off_flag(boma.object(), vars::common::AIR_CROSS);
            }
        }
    }
}

// Land cancel Cross if used in the air and fallen to the ground
unsafe fn land_cancel_cross(boma: &mut BattleObjectModuleAccessor, id: usize, situation_kind: i32) {
    if situation_kind == *SITUATION_KIND_GROUND && VarModule::is_flag(boma.object(), vars::common::AIR_CROSS) {
        VarModule::off_flag(boma.object(), vars::common::AIR_CROSS);
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    holy_water_ac_b_rev(fighter, boma, id, status_kind, situation_kind, cat[0], frame);
    cross_ff_land_cancel(boma, id, status_kind, situation_kind, cat[1], stick_y);
    air_cross_air_off(boma, id, status_kind, situation_kind);
    land_cancel_cross(boma, id, situation_kind);
}

#[utils::macros::opff(FIGHTER_KIND_SIMON )]
pub fn simon_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		simon_frame(fighter)
    }
}

pub unsafe fn simon_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}