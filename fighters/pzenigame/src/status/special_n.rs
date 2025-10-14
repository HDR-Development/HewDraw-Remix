use super::*;

unsafe extern "C" fn special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();

    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
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
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_motion_helper(fighter, false, Hash40::new("special_n_start"), Hash40::new("special_air_n_start"));
    fighter.main_shift(special_n_main_loop)
}

unsafe extern "C" fn special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_air_check_dive();
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_N_SHOOT.into(), false.into());
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            special_n_motion_helper(fighter, true, Hash40::new("special_n_start"), Hash40::new("special_air_n_start"));
        }
    }

    return 0.into();
}

unsafe extern "C" fn special_n_shoot_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_motion_helper(fighter, false, Hash40::new("special_n_shot"), Hash40::new("special_air_n_shot"));
    fighter.main_shift(special_n_shoot_main_loop)
}

unsafe extern "C" fn special_n_shoot_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_air_check_dive();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                //let landing_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_n.landing_frame");
                fighter.set_float(10.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
                return 1.into();
            }
            else {
                special_n_motion_helper(fighter, true, Hash40::new("special_n_shot"), Hash40::new("special_air_n_shot"));
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
        return 1.into();
    }

    return 0.into();
}

unsafe extern "C" fn special_n_motion_helper(fighter: &mut L2CFighterCommon, inherit: bool, ground_motion: Hash40, air_motion: Hash40) {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        if inherit {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, ground_motion, -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, ground_motion, 0.0, 1.0, false, 0.0, false, false);
        }
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        if inherit {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, air_motion, -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, air_motion, 0.0, 1.0, false, 0.0, false, false);
        }
    }
}

unsafe extern "C" fn special_n_shoot_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_flag(*FIGHTER_PZENIGAME_STATUS_SPECIAL_N_FLAG_SHOOT_ANGLE_ENABLE) {
        let angle = fighter.get_param_float("param_special_n", "angle");
        ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("head"), &Vector3f::new(0.0, 0.0, -angle), MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_NONE as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    }
    
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_main);

    agent.status(Main, *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_N_SHOOT, special_n_shoot_main);
    agent.status(Exec, *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_N_SHOOT, special_n_shoot_exec);
}