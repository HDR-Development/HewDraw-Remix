use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn jab_2_ftilt_cancel(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, situation_kind: i32, motion_kind: u64) {
    if [*FIGHTER_STATUS_KIND_ATTACK].contains(&status_kind) && motion_kind == hash40("attack_12") {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3)
               && (WorkModule::is_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) || WorkModule::is_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO)) {
                if !StopModule::is_stop(boma) {
                    VarModule::on_flag(boma, trail::ATTACK_12_INTO_S3);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3,false);
                }
            }
        }
    }
    if ![*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_S3].contains(&status_kind) {
        VarModule::off_flag(boma, trail::ATTACK_12_INTO_S3);
    }
}

// Fair 2 -> aerial cancel
unsafe fn fair_cancels(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, situation_kind: i32, motion_kind: u64) {
    // Check for aerial attack inputs during fair 2
    if status_kind == *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F && motion_kind == hash40("attack_air_f2") {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N
                                    | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3
                                    | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4
                                    | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3
                                    | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4)
                && ControlModule::get_attack_air_kind(boma) != *FIGHTER_COMMAND_ATTACK_AIR_KIND_F
                && ControlModule::get_attack_air_kind(boma) != *FIGHTER_COMMAND_ATTACK_AIR_KIND_B {
                if !StopModule::is_stop(boma) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                }
            }
            //if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_HI
            if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3
                                    | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) {
                if !StopModule::is_stop(boma) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                }
            }
            //if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_LW
            if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3
                                    | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) {
                if !StopModule::is_stop(boma) {                                        
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                }
            }
        }
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    jab_2_ftilt_cancel(boma, cat[0], status_kind, situation_kind, motion_kind);
    fair_cancels(boma, cat[0], status_kind, situation_kind, motion_kind);
}

#[utils::opff(0x5D )]
pub fn trail_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		trail_frame(fighter)
    }
}

pub unsafe fn trail_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        let status_kind = if info.status_kind == *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N || info.status_kind == *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F { // status kind checks for nair/fair
            *FIGHTER_STATUS_KIND_ATTACK_AIR
        } else {
            info.status_kind
        };
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}