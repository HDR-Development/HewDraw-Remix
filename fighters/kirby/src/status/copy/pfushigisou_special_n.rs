use super::*;

unsafe extern "C" fn pfushigisou_special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_mot_helper(fighter, Hash40::new("special_n_start"), Hash40::new("special_air_n_start"), false);
    fighter.main_shift(pfushigisou_special_n_main_loop)
}

unsafe extern "C" fn pfushigisou_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_PFUSHIGISOU_SPECIAL_N_END.into(), false.into());
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        special_n_mot_helper(fighter, Hash40::new("special_n_start"), Hash40::new("special_air_n_start"), true);
    }

    return 0.into();
}

unsafe extern "C" fn pfushigisou_special_n_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.battle_object, vars::kirby::instance::SPECIAL_N_PFUSHIGISOU_SEED_FIRED);
    special_n_mot_helper(fighter, Hash40::new("special_n_end"), Hash40::new("special_air_n_end"), false);
    fighter.main_shift(pfushigisou_special_n_end_main_loop)
}

unsafe extern "C" fn pfushigisou_special_n_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        special_n_mot_helper(fighter, Hash40::new("special_n_end"), Hash40::new("special_air_n_end"), true);
    }

    return 0.into();
}

unsafe extern "C" fn special_n_mot_helper(fighter: &mut L2CFighterCommon, ground_motion: Hash40, air_motion: Hash40, inherit: bool) {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        if inherit {
            FighterMotionModuleImpl::change_motion_inherit_frame_kirby_copy(fighter.module_accessor, ground_motion, -1.0, 1.0, 0.0, false, false);
        }
        else {
            FighterMotionModuleImpl::change_motion_kirby_copy(fighter.module_accessor, ground_motion, 0.0, 1.0, false, 0.0, false, false);
        }
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        if inherit {
            FighterMotionModuleImpl::change_motion_inherit_frame_kirby_copy(fighter.module_accessor, air_motion, -1.0, 1.0, 0.0, false, false);
        }
        else {
            FighterMotionModuleImpl::change_motion_kirby_copy(fighter.module_accessor, air_motion, 0.0, 1.0, false, 0.0, false, false);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_KIRBY_STATUS_KIND_PFUSHIGISOU_SPECIAL_N, pfushigisou_special_n_main);
    agent.status(Main, *FIGHTER_KIRBY_STATUS_KIND_PFUSHIGISOU_SPECIAL_N_END, pfushigisou_special_n_end_main);
}