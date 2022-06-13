// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn fishing_rod_shield_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, frame: f32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_START].contains(&status_kind) {
        if frame < 24.0 {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                if situation_kind == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
                }
            }
        }
    }
}

// Reel in
unsafe fn reel_in(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, frame: f32) {
    if status_kind == *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_END {
        if frame < 3.0 {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                if situation_kind == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
                }
            }
        }
    }
}

// Lloid Trap Fire Jump Cancel
unsafe fn lloid_trap_fire_jc(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, stick_x: f32, facing: f32, frame: f32) {
    if status_kind == *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_LW_FIRE {
        if frame > 5.0 {
            if boma.is_input_jump() && !boma.is_in_hitlag() {
                if situation_kind == *SITUATION_KIND_AIR {
                    if boma.get_num_used_jumps() < boma.get_jump_count_max() {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                    }
                } else if situation_kind == *SITUATION_KIND_GROUND {
                    if facing * stick_x < 0.0 {
                        PostureModule::reverse_lr(boma);
                    }
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                }
            }
        }
    }
}

// Balloon Trip Cancel
unsafe fn balloon_cancel(fighter: &mut L2CFighterCommon) {
    if (MotionModule::frame(fighter.module_accessor) > 5.0 && fighter.is_motion_one_of(&[Hash40::new("special_hi"), Hash40::new("special_air_hi")])) || fighter.is_status_one_of(&[*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_WAIT, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_FLAP]) {
        // Cancel balloon trip early if character is holding shield, allowing for movement
        if fighter.is_button_on(Buttons::Guard) || fighter.is_button_on(Buttons::Catch) || fighter.is_button_on(Buttons::AttackAll) {
            // Check if the user canceled before the initial swing, punishing them by setting their fuel to 0, else set their fuel to 80% of what they had
            if !fighter.is_motion_one_of(&[Hash40::new("special_hi"), Hash40::new("special_air_hi")]) {
                if WorkModule::get_float(fighter.module_accessor, *FIGHTER_MURABITO_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_FRAME) > 100.0 {
                    VarModule::set_float(fighter.object(), vars::shizue::STORED_BALLOON_POWER, WorkModule::get_float(fighter.module_accessor, *FIGHTER_MURABITO_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_FRAME) - 100.0);
                }
                else {
                    VarModule::set_float(fighter.object(), vars::shizue::STORED_BALLOON_POWER, 1.0);
                }
                VarModule::on_flag(fighter.object(), vars::shizue::IS_NOT_QUICK_RELEASE);
                EffectModule::req_follow(fighter.module_accessor, Hash40::new("shizue_putaway_catch"), Hash40::new("bust"), &Vector3f::zero(), &Vector3f::zero(), 0.8, true, 0, 0, 0, 0, 0, false, false);
            } 
            else {
                VarModule::off_flag(fighter.object(), vars::shizue::IS_NOT_QUICK_RELEASE);
                VarModule::set_float(fighter.object(), vars::shizue::STORED_BALLOON_POWER, 1.0);
                EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_damage_aura"), Hash40::new("bust"), &Vector3f::zero(), &Vector3f::zero(), 0.8, true, 0, 0, 0, 0, 0, false, false);
            }
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_DETACH, true);
        }
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    fishing_rod_shield_cancel(boma, status_kind, situation_kind, frame);
    reel_in(boma, status_kind, situation_kind, frame);
    lloid_trap_fire_jc(boma, status_kind, situation_kind, cat[0], stick_x, facing, frame);
}

#[utils::macros::opff(FIGHTER_KIND_SHIZUE )]
pub fn shizue_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		shizue_frame(fighter);
        balloon_cancel(fighter);
    }
}

pub unsafe fn shizue_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}