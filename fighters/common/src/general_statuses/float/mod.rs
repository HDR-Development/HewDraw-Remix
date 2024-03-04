use super::*;

mod pre;
mod main;
mod end;

#[no_mangle]
unsafe fn float_check_air_jump(fighter: &mut L2CFighterCommon, float_status: L2CValue) -> L2CValue {
    let is_aerial = fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0;

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON) {
        if !VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_FLOAT) {
            let mut allow_float = false;
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                let stick_y = fighter.left_stick_y();
                let squat_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("squat_stick_y"));
                if stick_y <= squat_stick_y {
                    allow_float = !is_aerial;
                }
            }

            if allow_float {
                fighter.change_status(float_status, true.into());
                return 1.into();
            }
        }
    }
    0.into()
}

#[no_mangle]
unsafe fn float_check_air_jump_aerial(fighter: &mut L2CFighterCommon, float_status: L2CValue) -> L2CValue {
    let is_aerial = fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0;

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON) {
        if !VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_FLOAT) {
            let mut allow_float = false;
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                if KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) < 0.0 {
                    allow_float = !is_aerial;
                }
            }

            if allow_float {
                fighter.change_status(float_status, true.into());
                return 1.into();
            }
        }
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL) {
        if !VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_FLOAT) {
            let mut allow_float = false;
            let stick_y = fighter.left_stick_y();
            let jump_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_stick_y"));
            if jump_stick_y <= stick_y {
                if KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) < 0.0 {
                    allow_float = !is_aerial;
                }
            }

            if allow_float {
                fighter.change_status(float_status, true.into());
                return 1.into();
            }
        }
    }
    0.into()
}