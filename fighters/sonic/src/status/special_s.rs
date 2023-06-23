use super::*;
use globals::*;
use smashline::*;


pub fn install() {
    install_status_scripts!(
        pre_special_s, main_special_s
    );
}

#[status_script(agent = "sonic", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_special_s(fighter: &mut L2CFighterCommon) -> L2CValue {
	StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        (
            *FIGHTER_STATUS_ATTR_START_TURN |
            *FIGHTER_STATUS_ATTR_DISABLE_GROUND_FRICTION
        ) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

#[status_script(agent = "sonic", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_special_s(fighter: &mut L2CFighterCommon) -> L2CValue {
    // fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s"), L2CValue::Hash40s("special_air_s"), false.into());
    VarModule::set_int(fighter.battle_object, vars::sonic::status::SPECIAL_S_STEP, vars::sonic::SPECIAL_S_STEP_START);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_s_boost_start"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    if fighter.global_table[globals::SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        VarModule::on_flag(fighter.battle_object, vars::sonic::instance::USED_AIR_ACTION);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    fighter.main_shift(special_s_main_loop)
}

unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Lets you grabl edge once it's enabled
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }

    let step = VarModule::get_int(fighter.battle_object, vars::sonic::status::SPECIAL_S_STEP);
    let situation = fighter.global_table[SITUATION_KIND].get_i32();

    if situation == *SITUATION_KIND_GROUND
    && VarModule::is_flag(fighter.battle_object, vars::sonic::status::SPECIAL_S_ENABLE_JUMP)
    && !StatusModule::is_changing(fighter.module_accessor) {
        fighter.check_jump_cancel(false);
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if step == vars::sonic::SPECIAL_S_STEP_END {
            let status = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                FIGHTER_STATUS_KIND_LANDING
            }
            else {
                FIGHTER_STATUS_KIND_FALL
            };
            fighter.change_status(status.into(), false.into());
            return 1.into();
        }
        if step == vars::sonic::SPECIAL_S_STEP_DASH {
            if situation == *SITUATION_KIND_AIR {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                return 1.into();
            }
        }
        if situation == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
        else {
            VarModule::on_flag(fighter.battle_object, vars::sonic::instance::USED_AIR_ACTION);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        sv_kinetic_energy!(
            set_brake,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            0.1,
            0.0
        );
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if step == vars::sonic::SPECIAL_S_STEP_START {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_s_boost"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            let lr = PostureModule::lr(fighter.module_accessor);
            let x_speed = 3.5;
            let y_speed = if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                    VarModule::on_flag(fighter.battle_object, vars::sonic::status::SPECIAL_S_HOP);
                    1.3
                }
                else {
                    KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
                }
            }
            else {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                    fighter.set_situation(SITUATION_KIND_AIR.into());
                    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                    sv_kinetic_energy!(
                        reset_energy,
                        fighter,
                        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                        ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
                        0.0,
                        0.0,
                        0.0,
                        0.0,
                        0.0
                    );
                    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
                    sv_kinetic_energy!(
                        set_stable_speed,
                        fighter,
                        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                        air_speed_y_stable * 0.7
                    );
                    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    VarModule::on_flag(fighter.battle_object, vars::sonic::status::SPECIAL_S_HOP);
                    VarModule::on_flag(fighter.battle_object, vars::sonic::instance::USED_AIR_ACTION);
                    1.3
                }
                else {
                    0.0
                }
            };
            sv_kinetic_energy!(
                set_limit_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                x_speed,
                0.0
            );
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                x_speed * lr,
                0.0
            );
            sv_kinetic_energy!(
                set_brake,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                0.1,
                0.0
            );
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                y_speed
            );
            VarModule::set_int(fighter.battle_object, vars::sonic::status::SPECIAL_S_STEP, vars::sonic::SPECIAL_S_STEP_DASH);
        }
        else if step == vars::sonic::SPECIAL_S_STEP_DASH {
            let mot = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                VarModule::off_flag(fighter.battle_object, vars::sonic::instance::USED_AIR_ACTION);
                hash40("special_s_boost_end")
            }
            else {
                hash40("special_air_s_boost_end")
            };
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            VarModule::set_int(fighter.battle_object, vars::sonic::status::SPECIAL_S_STEP, vars::sonic::SPECIAL_S_STEP_END);
        }
        else {
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
        }
    }

    0.into()
}
