// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn jc_light_utilt_hit(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3 {
        if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag()) && frame > 20.0 {
            if  !VarModule::is_flag(boma.object(), vars::common::status::IS_HEAVY_ATTACK) {
                if boma.is_input_jump() {
                    if situation_kind == *SITUATION_KIND_AIR {
                        if boma.get_num_used_jumps() < boma.get_jump_count_max() {
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                        }
                    } else if situation_kind == *SITUATION_KIND_GROUND {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                    }
                }
            }
        }
    }
}

// Jump cancel ligh utilt on hit
unsafe fn jc_dtilt_hit(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3 {
        if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag()) && frame > 12.0 {
            if boma.is_input_jump() {
                if situation_kind == *SITUATION_KIND_AIR {
                    if boma.get_num_used_jumps() < boma.get_jump_count_max() {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                    }
                } else if situation_kind == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                }
            }
        }
    }
}

// Mega Man Metal Blad Toss Airdodge Cancel
unsafe fn blade_toss_ac(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if frame > 16.0 {
            if boma.is_cat_flag(Cat1::AirEscape) && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
                if situation_kind == *SITUATION_KIND_AIR {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
                }
            }
        }
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    jc_light_utilt_hit(boma, id, status_kind, situation_kind, cat[0], frame);
    jc_dtilt_hit(boma, status_kind, situation_kind, cat[0], frame);
    blade_toss_ac(boma, status_kind, situation_kind, cat[0], frame);
}

#[utils::macros::opff(FIGHTER_KIND_ROCKMAN )]
pub fn rockman_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		rockman_frame(fighter)
    }
}

pub unsafe fn rockman_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}