use super::*;

#[status_script(agent = "donkey", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion;
    let kinetic;
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        motion = Hash40::new("special_air_lw");
        kinetic = *FIGHTER_KINETIC_TYPE_FALL;
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
    }
    else {
        motion = Hash40::new("special_lw_start");
        kinetic = *FIGHTER_KINETIC_TYPE_GROUND_STOP;
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    }
    KineticModule::change_kinetic(fighter.module_accessor, kinetic);
    MotionModule::change_motion(
        fighter.module_accessor,
        motion,
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    // <HDR>
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_lw_substatus as *const () as _));
    // </HDR>
    fighter.main_shift(special_lw_main_loop)
}

unsafe extern "C" fn special_lw_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR)
    && param_1.get_bool() {
        /* if VarModule::is_flag(fighter.battle_object, vars::donkey::status::SPECIAL_AIR_LW_STOP) {
            VarModule::off_flag(fighter.battle_object, vars::donkey::status::SPECIAL_AIR_LW_STOP);
            if !VarModule::is_flag(fighter.battle_object, vars::donkey::instance::SPECIAL_AIR_LW_USED_STALL) {
                /*sv_kinetic_energy!(
                    set_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_STOP,
                    0.0,
                    0.0
                );
                sv_kinetic_energy!(
                    set_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    0.5,
                );
                */
                VarModule::on_flag(fighter.battle_object, vars::donkey::instance::SPECIAL_AIR_LW_USED_STALL);
            }
        }
        */
        // enable fastfall
        if fighter.is_cat_flag(Cat2::FallJump)
            && fighter.stick_y() < -0.66
            && KineticModule::get_sum_speed_y(fighter.boma(), *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
            WorkModule::set_flag(fighter.boma(), true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
        }
    }
    0.into()
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_air = MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_lw");
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    if is_air {
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            if fighter.sub_transition_group_check_air_landing().get_bool()
            || fighter.sub_wait_ground_check_common(false.into()).get_bool()
            || fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        let status = if situation != *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_FALL
        }
        else {
            WorkModule::set_float(fighter.module_accessor, 20.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL
        };
        fighter.change_status(status.into(), false.into());
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if is_air {
            FIGHTER_STATUS_KIND_FALL
        }
        else {
            FIGHTER_DONKEY_STATUS_KIND_SPECIAL_LW_LOOP
        };
        fighter.change_status(status.into(), false.into());
    }
    1.into()
}

pub fn install() {
    smashline::install_status_scripts!(
        special_lw_main
    );
}
