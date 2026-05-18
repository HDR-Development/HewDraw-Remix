use super::*;

pub unsafe extern "C" fn special_n3_loop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_air_check_dive();
    if !StopModule::is_stop(fighter.module_accessor) {
        sub_special_n3_loop(fighter, false.into());
    }

    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sub_special_n3_loop as *const () as _));
    fighter.main_shift(special_n3_loop_main_loop)
}

unsafe extern "C" fn sub_special_n3_loop(fighter: &mut L2CFighterCommon, param: L2CValue) -> L2CValue {
    if param.get_bool() {
        fighter.inc_int(*FIGHTER_MIIGUNNER_STATUS_GRENADE_LAUNCHER_WORK_INT_HOLD_COUNT);
    }
    // else {
    //     if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
    //         fighter.global_table[SUB_STATUS2].assign(&L2CValue::I32(0));
    //         fighter.global_table[SUB_STATUS].assign(&L2CValue::I32(0));
    //     }
    //     else {
    //         let hold_count = fighter.get_int(*FIGHTER_MIIGUNNER_STATUS_GRENADE_LAUNCHER_WORK_INT_HOLD_COUNT);
    //         let max_hold = fighter.get_param_int("param_special_n", "n3_hold_max_frame");
    //         if hold_count > max_hold {
    //             fighter.on_flag(*FIGHTER_MIIGUNNER_STATUS_GRENADE_LAUNCHER_FLAG_MAX);
    //             fighter.global_table[SUB_STATUS2].assign(&L2CValue::I32(0));
    //             fighter.global_table[SUB_STATUS].assign(&L2CValue::I32(0));
    //         }
    //     }
    // }

    return 0.into();
}

pub unsafe extern "C" fn special_n3_loop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor)
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            if fighter.is_flag(*FIGHTER_MIIGUNNER_STATUS_GRENADE_LAUNCHER_FLAG_FIRST) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n3_loop"), -1.0, 1.0, 0.0, false, false);
            } else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n3_loop"), 0.0, 1.0, false, 0.0, false, false);
                fighter.on_flag(*FIGHTER_MIIGUNNER_STATUS_GRENADE_LAUNCHER_FLAG_FIRST);
            };
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            if fighter.is_flag(*FIGHTER_MIIGUNNER_STATUS_GRENADE_LAUNCHER_FLAG_FIRST) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n3_loop"), -1.0, 1.0, 0.0, false, false);
            } else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n3_loop"), 0.0, 1.0, false, 0.0, false, false);
                fighter.on_flag(*FIGHTER_MIIGUNNER_STATUS_GRENADE_LAUNCHER_FLAG_FIRST);
            };
        }
    }
    //special_n3_charge(fighter);
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
    //|| fighter.is_flag(*FIGHTER_MIIGUNNER_STATUS_GRENADE_LAUNCHER_FLAG_MAX) {
        fighter.change_status(FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N3_END.into(), true.into());
        return 1.into();
    }
    
    return 0.into();
}

// unsafe extern "C" fn special_n3_charge(fighter: &mut L2CFighterCommon) {
//     let charge = VarModule::get_float(fighter.battle_object, vars::miigunner::status::ATTACK_CHARGE);
//     let mut charge_start_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_charge.special_n3_charge_start");
//     let mut charge_end_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_charge.special_n3_charge_end");
//     let mut max_charge_frames = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_charge.max_charge_frames");
//     if (charge_start_frame..charge_end_frame).contains(&fighter.motion_frame()) && charge < max_charge_frames
//     && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
//         let motion_rate = (charge_end_frame - charge_start_frame)/max_charge_frames;
//         MotionModule::set_rate(fighter.module_accessor, motion_rate);
//         VarModule::set_float(fighter.battle_object, vars::miigunner::status::ATTACK_CHARGE, charge + 1.0);
//     }
//     else {
//         VarModule::set_float(fighter.battle_object, vars::miigunner::instance::SPECIAL_N3_CHARGE, charge);
//         MotionModule::set_rate(fighter.module_accessor, 1.0);
//     }
// }

pub unsafe extern "C" fn special_n3_end_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_air_check_dive();
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N3_LOOP, special_n3_loop_main);
    agent.status(Exec, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N3_END, special_n3_end_exec);
}