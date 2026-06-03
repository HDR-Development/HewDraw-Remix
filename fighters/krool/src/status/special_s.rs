use super::*;

unsafe extern "C" fn special_s_catch_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_CROWN, ArticleOperationTarget(0));
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, 0.0);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

    fighter.main_shift(special_s_catch_main_loop)
}

unsafe extern "C" fn special_s_catch_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor)
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        special_s_change_motion(fighter, Hash40::new("special_s_catch"), Hash40::new("special_air_s_catch"));
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
        return 0.into();
    }

    return 0.into()
}

unsafe extern "C" fn special_s_change_motion(fighter: &mut L2CFighterCommon, ground_motion: Hash40, air_motion: Hash40) {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        if !fighter.is_flag(*FIGHTER_KROOL_STATUS_SPECIAL_S_FLAG_FIRST) {
            fighter.on_flag(*FIGHTER_KROOL_STATUS_SPECIAL_S_FLAG_FIRST);
            MotionModule::change_motion(fighter.module_accessor, ground_motion, 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, ground_motion, -1.0, 1.0, 0.0, false, false);
        }
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if !fighter.is_flag(*FIGHTER_KROOL_STATUS_SPECIAL_S_FLAG_FIRST) {
            fighter.on_flag(*FIGHTER_KROOL_STATUS_SPECIAL_S_FLAG_FIRST);
            MotionModule::change_motion(fighter.module_accessor, air_motion, 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, air_motion, -1.0, 1.0, 0.0, false, false);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_CATCH, special_s_catch_main);
}