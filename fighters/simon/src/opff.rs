use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn holy_water_ac_b_rev(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat1: i32, stick_x: f32, facing: f32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        if frame > 19.0 {
            if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
                if situation_kind == *SITUATION_KIND_AIR {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
                }
            }
        }
        if frame < 5.0 {
            if stick_x * facing < 0.0 {
                PostureModule::reverse_lr(boma);
                PostureModule::update_rot_y_lr(boma);
                if frame > 1.0 && frame < 5.0 && !b_reversed[id] {
                    let b_reverse = Vector3f{x: -1.0, y: 1.0, z: 1.0};
                    KineticModule::mul_speed(boma, &b_reverse, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    b_reversed[id] = true;
                }
            }
        }
    }
}

// Simon Cross Fast Fall/Land Cancel
unsafe fn cross_ff_land_cancel(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_SIMON_STATUS_KIND_SPECIAL_S2].contains(&status_kind) {
        if situation_kind == *SITUATION_KIND_AIR {
            air_cross[id] = true;
            if hdr::compare_cat(cat2, *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP)
                && stick_y < -0.66
                && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
    }
}

// Turn off air_cross flag if not in Cross in the air
unsafe fn air_cross_air_off(id: usize, status_kind: i32, situation_kind: i32) {
    if situation_kind == *SITUATION_KIND_AIR {
        if ![*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_SIMON_STATUS_KIND_SPECIAL_S2].contains(&status_kind) {
            if air_cross[id] {
                air_cross[id] = false;
            }
        }
    }
}

// Land cancel Cross if used in the air and fallen to the ground
unsafe fn land_cancel_cross(boma: &mut BattleObjectModuleAccessor, id: usize, situation_kind: i32) {
    if situation_kind == *SITUATION_KIND_GROUND && air_cross[id] {
        air_cross[id] = false;
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    holy_water_ac_b_rev(boma, id, status_kind, situation_kind, cat[0], stick_x, facing, frame);
    cross_ff_land_cancel(boma, id, status_kind, situation_kind, cat[1], stick_y);
    air_cross_air_off(id, status_kind, situation_kind);
    land_cancel_cross(boma, id, situation_kind);
}

#[utils::opff(FIGHTER_KIND_SIMON )]
pub fn simon_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		simon_frame(fighter)
    }
}

pub unsafe fn simon_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}