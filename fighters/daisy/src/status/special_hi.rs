use super::*;


unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_int(2, *FIGHTER_PEACH_STATUS_SPECIAL_HI_WORK_INT_ENABLE_UNIQ);

    let grav_speed_y = fighter.get_speed_y(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    fighter.change_motion_by_situation("special_hi_start", "special_air_hi_start", 0.0, 1.0, false, 0.0, false, false);
    fighter.change_kinetic_by_situation(*FIGHTER_KINETIC_TYPE_GROUND_STOP, *FIGHTER_KINETIC_TYPE_PEACH_SPECIAL_AIR_HI_START);
    fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND, *GROUND_CORRECT_KIND_AIR);
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        let mut gravity_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut app::FighterKineticEnergyGravity;
        smash::app::lua_bind::FighterKineticEnergyGravity::set_speed(gravity_energy, grav_speed_y);
    }

    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
    GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_PEACH_CLIFF_HANG_DATA_SPECIAL_HI as u32);

    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.sub_transition_group_check_air_cliff().get_bool() {
        angling(fighter);
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_AIR_END.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn special_hi_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_situation_keep(L2CValue::I32(*SITUATION_KIND_AIR), 0.into());
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    return 0.into();
}

// FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_AIR_END

unsafe extern "C" fn special_hi_air_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_FALL,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_PEACH_SPECIAL_HI_AIR_END_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_PEACH_SPECIAL_HI_AIR_END_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_PEACH_SPECIAL_HI_AIR_END_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_hi_air_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let landing_frame = fighter.get_param_int("param_special_hi", "special_hi_landing_mot_frame");
    fighter.set_float(landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_end"), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

    // fix endlag speed to match angled momentum
    let maxrot = fighter.get_param_float("param_special_hi", "special_hi_start_dir_mul");
    let angle = VarModule::get_float(fighter.battle_object, vars::daisy::instance::SPECIAL_HI_ANGLE);
    let lr = fighter.lr();
    let speed_x_mul = -angle*lr/maxrot + 0.55;
    sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, speed_x_mul);

    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
    GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_PEACH_CLIFF_HANG_DATA_SPECIAL_HI as u32);

    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_air_end_main_loop as *const () as _))
}

unsafe extern "C" fn special_hi_air_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.sub_transition_group_check_air_cliff().get_bool() {
        fighter.sub_air_check_dive();
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            return 1.into();
        }

        if MotionModule::is_end(fighter.module_accessor) {
            let accel_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fall_special_accel_x_mul");
            let speed_x_max_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fall_special_speed_x_max_mul");
            fighter.set_float(accel_x_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_MUL_FALL_X_ACCEL);
            fighter.set_float(speed_x_max_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn angling(fighter: &mut L2CFighterCommon) -> bool {
    if fighter.is_flag(*FIGHTER_PEACH_STATUS_SPECIAL_HI_FLAG_MOVE_TRANS) {
        fighter.off_flag(*FIGHTER_PEACH_STATUS_SPECIAL_HI_FLAG_MOVE_TRANS);
        // start rise
        if fighter.get_int(*FIGHTER_PEACH_STATUS_SPECIAL_HI_WORK_INT_ENABLE_UNIQ) > 1 {
            fighter.set_situation(SITUATION_KIND_AIR.into());
            fighter.set_situation_keep(L2CValue::I32(*SITUATION_KIND_AIR), 1.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE);
            
            fighter.set_int(1, *FIGHTER_PEACH_STATUS_SPECIAL_HI_WORK_INT_ENABLE_UNIQ);
            return true;
        }
        // angle
        if fighter.get_int(*FIGHTER_PEACH_STATUS_SPECIAL_HI_WORK_INT_ENABLE_UNIQ) > 0 {
            // angle f8
            fighter.set_situation_keep(L2CValue::I32(*SITUATION_KIND_AIR), 0.into());

            let maxrot = fighter.get_param_float("param_special_hi", "special_hi_start_dir_mul");
            let speed= fighter.get_param_float("param_special_hi", "special_hi_start_trans_speed_mul");
            let facing = fighter.lr();
            let stick = Vector2f::new(
                fighter.left_stick_x(),
                fighter.left_stick_y()        
            );

            let angle_from_vertical = app::sv_math::vec2_angle(stick.x, stick.y, 0.0, 1.0).to_degrees();
            let mut angle = angle_from_vertical * stick.x.signum() * -1.0;
            angle = if facing < 0.0 {
                angle.clamp(-maxrot, maxrot)
            } else {
                angle.clamp(-maxrot, maxrot)
            };
            VarModule::set_float(fighter.battle_object, vars::daisy::instance::SPECIAL_HI_ANGLE, angle);
            let angle_rad = angle.to_radians();

            sv_kinetic_energy!(set_angle, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, angle_rad);
            sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, speed);
            fighter.set_int(0, *FIGHTER_PEACH_STATUS_SPECIAL_HI_WORK_INT_ENABLE_UNIQ);
            return true;
        }
    }
    return false
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_main);
    agent.status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_exit);

    agent.status(Pre, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_AIR_END, special_hi_air_end_pre);
    agent.status(Main, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_AIR_END, special_hi_air_end_main);
}