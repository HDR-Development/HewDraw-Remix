use super::*;

unsafe extern "C" fn special_n_cancel_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_cancel"), 0.0, 1.0, false, 0.0, false, false);
    } else {
        if fighter.get_int(*FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
            fighter.change_status(FIGHTER_STATUS_KIND_JUMP_AERIAL.into(), false.into());
        } // jump cancel
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_cancel"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    let status = fighter.get_int(*FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
    if status == *STATUS_KIND_NONE || status == *FIGHTER_STATUS_KIND_GUARD_ON {fighter.set_int(*FIGHTER_STATUS_KIND_WAIT,*FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS); }
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);
    fighter.main_shift(special_n_cancel_main_loop)
}

unsafe extern "C" fn special_n_cancel_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_GROUND && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_cancel"), -1.0, 1.0, 0.0, false, false);
    }
    else if fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_AIR && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        fighter.set_int(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_cancel"), -1.0, 1.0, 0.0, false, false);
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            let status = fighter.get_int(*FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
            fighter.change_status(L2CValue::I32(status).into(), false.into());
            return 1.into();
        }
        if fighter.sub_air_check_fall_common().get_bool()
        || fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 0.into();
        }
    }    
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into()); }
        else {fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into()); }
    }
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_CANCEL, special_n_cancel_main);
}