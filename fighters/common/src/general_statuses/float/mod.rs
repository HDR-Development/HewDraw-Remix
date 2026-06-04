use super::*;

mod pre;
mod main;
mod end;

#[no_mangle]
unsafe fn float_checks(fighter: &mut L2CFighterCommon, float_status: L2CValue) -> bool {
    if !VarModule::is_flag(fighter.battle_object, vars::common::instance::ENABLE_FLOAT)
    || VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_FLOATING) { // flag seems only useful if there was a non-once-per-airtime float?
        return false.into();
    }

    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        return false.into();
    }

    let buffer = ControlModule::get_command_life_count_max(fighter.module_accessor) as usize;
    let stick_y = fighter.left_stick_y();
    let squat_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("squat_stick_y"));

    if stick_y > squat_stick_y
    && (InputModule::get_trigger_count(fighter.battle_object, Buttons::Jump) <= buffer
    || KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0) {
        return false.into();
    }

    // remove floating with buffered jump+aerials, instant held float f1 of jump
    let airtime = fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR);
    if airtime < 1 
    && (fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
    && !VarModule::is_flag(fighter.battle_object, vars::common::instance::OMNI_FLOAT)
    || stick_y > squat_stick_y) {
        return false.into()
    }

    // no water floats
    let sit = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_sit = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if sit == *SITUATION_KIND_WATER
    || prev_sit == *SITUATION_KIND_WATER {
        return false.into()
    }

    fighter.change_status(float_status, false.into());
    true.into()
}

#[no_mangle]
unsafe fn float_check_air_jump(fighter: &mut L2CFighterCommon, float_status: L2CValue) -> L2CValue {
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON) {
        if float_checks(fighter, float_status) {
            return 1.into()
        }
    }
    0.into()
}

#[no_mangle]
unsafe fn float_check_air_jump_aerial(fighter: &mut L2CFighterCommon, float_status: L2CValue) -> L2CValue {
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON)
    || WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL) {
        if float_checks(fighter, float_status) {
            return 1.into()
        }
    }
    0.into()
}