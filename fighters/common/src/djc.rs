use super::*;

use globals::*;

pub fn install() {
    smashline::install_hooks!(
        sub_attack_air_inherit_jump_aerial_motion_uniq_process_init_impl,
        sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec_impl
    );
}

// TAGS: DJC, Double Jump Cancel, FIGHTER_STATUS_KIND_ATTACK_AIR
/// Regular attack air status script except uses the animation's movement by default.
/// To be used by fighters who have double jump cancel
#[utils::export(common::djc)]
pub unsafe extern "C" fn attack_air_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_common(L2CValue::Bool(false));
    MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, true);
    fighter.sub_shift_status_main(L2CValue::Ptr(attack_air_main_status_loop as *const () as _))
}

// TAGS: DJC, Double Jump Cancel, FIGHTER_STATUS_KIND_ATTACK_AIR
/// Performs the leniency check for double jump canceling
#[utils::export(common::djc)]
pub unsafe extern "C" fn attack_air_main_status_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_JUMP_AERIAL {
        let djc_leniency_frame = ParamModule::get_int(fighter.battle_object, ParamType::Common, "djc_leniency_frame");
        if fighter.global_table[CURRENT_FRAME].get_i32() <= djc_leniency_frame && !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        }
    }
    if !fighter.status_AttackAir_Main_common().get_bool() {
        fighter.sub_air_check_superleaf_fall_slowly();
        if !fighter.global_table[IS_STOPPING].get_bool() {
            fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec_fix_pos();
        }
        0.into()
    }
    else {
        1.into()
    }
}

// TAGS: DJC, Double Jump Cancel, FIGHTER_STATUS_KIND_ATTACK_AIR
/// Inherits the double jump animation movement when doing an aerial (init)
#[smashline::hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon59sub_attack_air_inherit_jump_aerial_motion_uniq_process_initEv")]
pub unsafe extern "C" fn sub_attack_air_inherit_jump_aerial_motion_uniq_process_init_impl(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[FIGHTER_KIND] == FIGHTER_KIND_DEMON {
        call_original!(fighter)
    } else {
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);

        fighter.sub_attack_air_kind();
        if motion_kind == smash::hash40("jump_aerial_f") || motion_kind == smash::hash40("jump_aerial_b") {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION)
            && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new_raw(motion_kind), frame, 1.0, false, 1.0);
                MotionModule::set_weight(fighter.module_accessor, 1.0, true);
            }
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND);
            } else {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            }
        } else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        }
        let _ = fighter.sub_attack_air_uniq_process_init();
        0.into()
    }
}

#[utils::export(common::djc)]
fn sub_attack_air_inherit_jump_aerial_motion_uniq_process_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    unsafe {
        sub_attack_air_inherit_jump_aerial_motion_uniq_process_init_impl(fighter) 
    }
}

// TAGS: DJC, Double Jump Cancel, FIGHTER_STATUS_KIND_ATTACK_AIR
/// Inherits the double jump animation movement when doing an aerial (exec)
#[smashline::hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon59sub_attack_air_inherit_jump_aerial_motion_uniq_process_execEv")]
pub unsafe extern "C" fn sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec_impl(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_JUMP_AERIAL
    && fighter.global_table[FIGHTER_KIND] != FIGHTER_KIND_DEMON
    && fighter.global_table[CURRENT_FRAME].get_i32() <= ParamModule::get_int(fighter.battle_object, ParamType::Common, "djc_leniency_frame")
    && !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
    }
    call_original!(fighter)
}

#[utils::export(common::djc)]
fn sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    unsafe {
        sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec_impl(fighter)
    }
}