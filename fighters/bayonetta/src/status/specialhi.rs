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
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 0.0, 1.0, false, 0.0, false, false);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.set_situation_keep(L2CValue::I32(*SITUATION_KIND_AIR), 1.into());
        fighter.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_SITUATION_KEEP);
    } else {
        fighter.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_GROUND_START);
    }
    ItemModule::set_change_status_event(fighter.module_accessor, true);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into()
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) && !StatusModule::is_changing(fighter.module_accessor) {
        fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND, *GROUND_CORRECT_KIND_AIR);
        fighter.change_kinetic_by_situation(*FIGHTER_KINETIC_TYPE_MOTION, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        if fighter.global_table[globals::SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            let accel_y = fighter.get_param_float("param_special_hi", "air_accel_y");
            let air_max_speed_y = fighter.get_param_float("param_special_hi", "air_max_speed_y");
            sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, accel_y);
            sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_max_speed_y);
        }
    }
    if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP) {
        if fighter.global_table[globals::SITUATION_KIND] != *SITUATION_KIND_GROUND {
            fighter.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_AIR);
        }
        fighter.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
        KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY); // make distance not based on current speed
        fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_JUMP.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn special_hi_heavy_check(fighter: &mut L2CFighterCommon) -> L2CValue {
    if hold_check(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.75, 4, 4, 4, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
        VarModule::inc_int(fighter.battle_object, vars::bayonetta::instance::SPECIAL_BULLET_ARTS_COUNT);
    }
    0.into()
}

unsafe extern "C" fn special_hi_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ItemModule::set_change_status_event(fighter.module_accessor, true);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_jump_main_loop as *const () as _))
}

unsafe extern "C" fn special_hi_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into()
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) && (fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool()) {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[globals::SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
    }
    fighter.sub_air_check_dive();
    // vanilla land before move ends during fall portion
    if fighter.global_table[globals::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let motion_y: f32 = MotionModule::trans_move_speed(fighter.module_accessor).x();
        if !KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION)
        || motion_y < 0.0 {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
            return 1.into();
        }
    }
    // vanilla cancel mechanic on tap up b
    if fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SHOOTING_STEP) == *FIGHTER_BAYONETTA_SHOOTING_STEP_SHOOTING {
        fighter.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_SHOOTING);
    }
    if !fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_SHOOTING)
    && fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_NO_SHOOTING_ENABLE_CANCEL) {
        fighter.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_NO_SHOOTING_ENABLE_CANCEL);
        CancelModule::enable_cancel(fighter.module_accessor);
    }
    // heavy variant on bullet arts input
    if VarModule::is_flag(fighter.battle_object, vars::bayonetta::status::SPECIAL_1F_CHECK) {
        special_hi_heavy_check(fighter);
        VarModule::off_flag(fighter.battle_object, vars::bayonetta::status::SPECIAL_1F_CHECK);
    }
    // enable cancel flag if non-BA lands?
    if !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) 
    && VarModule::get_int(fighter.battle_object, vars::common::instance::LAST_ATTACK_HITBOX_ID) < 6 {
        VarModule::on_flag(fighter.battle_object, vars::bayonetta::status::SPECIAL_HIT_NO_BULLET);
    }
    0.into()
}

unsafe extern "C" fn special_hi_jump_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(End, fighter, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_JUMP)(fighter);
    fighter.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
    fighter.set_int(*FIGHTER_BAYONETTA_SHOOTING_STEP_WAIT_END, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SHOOTING_STEP);
    set_lag(fighter);
    ret
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_main);

    agent.status(Main, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_JUMP, special_hi_jump_main);
    agent.status(End, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_JUMP, special_hi_jump_end);
}