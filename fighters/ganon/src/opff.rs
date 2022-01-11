use ::common::opff_import::*;
use super::*;
use globals::*;
use ::common::opff::*;
 
unsafe fn aerial_wiz_foot_jump_reset_bounce(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END,
        *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_WALL_END].contains(&status_kind) {
        if situation_kind == *SITUATION_KIND_AIR {
            let jump_count_max = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) == jump_count_max {
                WorkModule::set_int(boma, jump_count_max - 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            }
        }
    }
}

// Dtaunt Counter
unsafe fn dtaunt_counter(boma: &mut BattleObjectModuleAccessor, motion_kind: u64, frame: f32) {
    if [hash40("appeal_lw_l"), hash40("appeal_lw_r")].contains(&motion_kind)
        && frame >= 29.0 && frame <= 59.0 {
        if FighterStopModuleImpl::is_damage_stop(boma) {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                DamageModule::add_damage(boma, 100.0, 0);
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
                StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, false);
            }
        }
    }
}

// Warlock Punch B-Reverse
unsafe fn warlock_punch_b_reverse(boma: &mut BattleObjectModuleAccessor, id: usize, motion_kind: u64, stick_x: f32, facing: f32, frame: f32) {
    if motion_kind == hash40("special_air_n") {
        if frame < 5.0 {
            if stick_x * facing < 0.0 {
                PostureModule::reverse_lr(boma);
                PostureModule::update_rot_y_lr(boma);
                if frame > 1.0 && frame < 5.0 &&  !VarModule::is_flag(boma.object(), vars::common::B_REVERSED) {
                    let b_reverse = Vector3f{x: -1.0, y: 1.0, z: 1.0};
                    KineticModule::mul_speed(boma, &b_reverse, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    VarModule::on_flag(boma.object(), vars::common::B_REVERSED);
                }
            }
        }
    }
}

// Wizard's Foot B-Reverse
unsafe fn wizards_foot_b_reverse(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, stick_x: f32, facing: f32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        if frame > 1.0 && frame < 5.0 {
            if stick_x * facing < 0.0 {
                PostureModule::reverse_lr(boma);
                PostureModule::update_rot_y_lr(boma);
                if  !VarModule::is_flag(boma.object(), vars::common::B_REVERSED) {
                    KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                    VarModule::on_flag(boma.object(), vars::common::B_REVERSED);
                }
            }
        }
    }
}

// Repeated Warlock Punch turnaround
unsafe fn repeated_warlock_punch_turnaround(boma: &mut BattleObjectModuleAccessor, status_kind: i32, stick_x: f32, facing: f32, frame: f32) {
    if status_kind == *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_TURN {
        if frame > 30.0 && frame < 45.0 {
            if stick_x * facing < 0.0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_TURN, true);
            }
        }
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    aerial_wiz_foot_jump_reset_bounce(boma, status_kind, situation_kind);
    wizards_foot_b_reverse(boma, id, status_kind, stick_x, facing, frame);
    //dtaunt_counter(boma, motion_kind, frame);
    warlock_punch_b_reverse(boma, id, motion_kind, stick_x, facing, frame);
    repeated_warlock_punch_turnaround(boma, status_kind, stick_x, facing, frame);
}

#[utils::opff(FIGHTER_KIND_GANON )]
pub fn ganon_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		ganon_frame(fighter)
    }
}

pub unsafe fn ganon_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}