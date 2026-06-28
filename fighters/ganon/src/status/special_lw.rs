use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_LW

unsafe extern "C" fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        (*FIGHTER_STATUS_ATTR_START_TURN | *FIGHTER_STATUS_ATTR_DISABLE_GROUND_FRICTION) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    0.into()
}

unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.off_flag(*FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_SP_BRAKE);
    fighter.off_flag(*FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_CLIFF_CHECK);
    fighter.off_flag(*FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK);
    fighter.off_flag(*FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_FALL_ONOFF);
    let hit_reduct_count = fighter.get_param_int("param_special_lw", "hit_reduct_count");
    fighter.set_int(hit_reduct_count, *FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_KICK_REDUCTION_LEFT);
    fighter.set_float(1.0, *FIGHTER_GANON_STATUS_WORK_ID_FLOAT_GANON_KICK_SPEED_COEFFICIENT);
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        fighter.set_int(*SITUATION_KIND_GROUND, *FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_KICK_START_SITUATION);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    else {
        fighter.set_int(*SITUATION_KIND_AIR, *FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_KICK_START_SITUATION);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }

    fighter.main_shift(special_lw_main_loop)
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END.into(), false.into());
        return 0.into();
    }
    if fighter.get_int(*FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_KICK_START_SITUATION) == *SITUATION_KIND_GROUND {
        // Reduce speed on shield
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_PARRY) {
            let shield_hit_speed_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lw.shield_hit_speed_x_mul");
            sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, shield_hit_speed_x_mul);
        }
        let touch_flag = if fighter.lr() <= 0.0 { *GROUND_TOUCH_FLAG_LEFT } else { *GROUND_TOUCH_FLAG_RIGHT };
        if GroundModule::is_touch(fighter.module_accessor, touch_flag as u32) {
            if fighter.is_flag(*FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK) {
                fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_WALL_END.into(), false.into());
                return 0.into();
            }
        }
    }
    else {
        if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
            if fighter.is_flag(*FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK) {
                fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END.into(), false.into());
                return 0.into();
            }
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
    }

    return 0.into();
}

// FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END

unsafe extern "C" fn special_lw_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let x_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

    let ret = smashline::original_status(Main, fighter, *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END)(fighter);

    let start_situation = WorkModule::get_int(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_KICK_START_SITUATION);
    let end_situation = WorkModule::get_int(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_KICK_END_SITUATION);
    if start_situation == *SITUATION_KIND_AIR
    && end_situation == *FIGHTER_GANON_KICK_END_SITUATION_AG {
        // Allows you to slide when landing early into Wizard Kick's animation
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: x_speed.abs(), y: 0.0, z: 0.0});
    }
    else if start_situation == *SITUATION_KIND_AIR
    && end_situation == *FIGHTER_GANON_KICK_END_SITUATION_AA {
        // Allows you to slide when landing late into Wizard Kick's animation
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_KICK_START_SITUATION);
    }

    ret
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main);
    
    agent.status(Main, *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END, special_lw_end_main);
}
