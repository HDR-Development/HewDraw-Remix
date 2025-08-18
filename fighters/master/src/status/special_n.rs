use super::*;

unsafe extern "C" fn special_n_cancel_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_cancel"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_cancel"), 0.0, 1.0, false, 0.0, false, false);
    }
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BOW) {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BOW, Hash40::new("special_n_cancel"), false, -1.0);
    }

    fighter.main_shift(special_n_cancel_main_loop)
}

unsafe extern "C" fn special_n_cancel_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_check_charge_cancel_jump_mini_attack();
    fighter.sub_air_check_dive();
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_cancel"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_cancel"), -1.0, 1.0, 0.0, false, false);
        }
        if !StatusModule::is_changing(fighter.module_accessor) {
            fighter.set_int(*FIGHTER_MASTER_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_MASTER_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
        }
    }
    // stub cancel types we don't want
    let cancel = fighter.get_int(*FIGHTER_MASTER_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
    let cancel_type = if [
        *FIGHTER_MASTER_SPECIAL_N_CANCEL_TYPE_AIR_ESCAPE_AIR,
        *FIGHTER_MASTER_SPECIAL_N_CANCEL_TYPE_GROUND_ESCAPE,
        *FIGHTER_MASTER_SPECIAL_N_CANCEL_TYPE_GROUND_ESCAPE_F,
        *FIGHTER_MASTER_SPECIAL_N_CANCEL_TYPE_GROUND_ESCAPE_B,
    ].contains(&cancel) { *FIGHTER_MASTER_SPECIAL_N_CANCEL_TYPE_NONE } else { cancel };
    if cancel_type == *FIGHTER_MASTER_SPECIAL_N_CANCEL_TYPE_GROUND_GUARD {
        if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
            fighter.set_int(*FIGHTER_MASTER_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_MASTER_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
        }
    }
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            if cancel_type == *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_GROUND_JUMP {
                FighterControlModuleImpl::update_attack_air_kind(fighter.module_accessor, true);
            }
        }
    }
    let mut change_status = false;
    if cancel_type == *FIGHTER_MASTER_SPECIAL_N_CANCEL_TYPE_NONE {
        if !MotionModule::is_end(fighter.module_accessor) {
            if CancelModule::is_enable_cancel(fighter.module_accessor) {
                change_status = true;
            }
        }
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            change_status = true;
        }
    }
    if change_status {
        if cancel_type == *FIGHTER_MASTER_SPECIAL_N_CANCEL_TYPE_GROUND_GUARD {
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD_ON.into(), false.into());
            return 1.into();
        }
        if cancel_type == *FIGHTER_MASTER_SPECIAL_N_CANCEL_TYPE_GROUND_JUMP_MINI_ATTACK {
            fighter.on_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK);
            fighter.change_status(FIGHTER_STATUS_KIND_JUMP_SQUAT.into(), false.into());
            return 1.into();
        }
        if cancel_type == *FIGHTER_MASTER_SPECIAL_N_CANCEL_TYPE_GROUND_JUMP {
            fighter.change_status(FIGHTER_STATUS_KIND_JUMP_SQUAT.into(), false.into());
            return 1.into();
        }
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            ControlModule::clear_command(fighter.module_accessor, true);
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 1.into();
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_CANCEL, special_n_cancel_main);
}