use super::*;


// FIGHTER_STATUS_KIND_SPECIAL_HI

unsafe extern "C" fn special_hi_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    // start situation
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    fighter.set_int(situation, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_START_SITUATION);
    // collect momentum
    let mut stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
    let reset = *ENERGY_STOP_RESET_TYPE_AIR;
    let x_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let mut y_speed = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    KineticModule::unable_energy_all(fighter.module_accessor);
    if situation == *SITUATION_KIND_GROUND {
        let reset = *ENERGY_STOP_RESET_TYPE_GROUND;
        y_speed = 0.0;
    }
    // momentum to stop energy conversion
    let special_hi_speed_brake = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.special_hi_speed_brake");
    let special_hi_speed_y_max = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.special_hi_speed_y_max");
    let special_hi_limit_speed = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.special_hi_limit_speed");
    lua_bind::KineticEnergy::reset_energy(stop_energy, reset, &Vector2f{x: x_speed, y: y_speed.clamp(-special_hi_speed_y_max, special_hi_speed_y_max)}, &Vector3f::zero(), fighter.module_accessor);
    lua_bind::KineticEnergy::enable(stop_energy);
    sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, special_hi_speed_brake, special_hi_speed_brake);
    sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, special_hi_limit_speed, special_hi_limit_speed);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    0.into()
}

unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

// FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2

unsafe extern "C" fn special_hi2_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    // reset prev momentum to prevent weird height changes
    KineticModule::clear_speed_all(fighter.module_accessor);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_UNIQ);
    smashline::original_status(Init, fighter, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2)(fighter)
}

unsafe extern "C" fn special_hi2_check_attack(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    return 0.into();
}

// statuses::kirby::SPECIAL_HI_H

unsafe extern "C" fn special_hi_h_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
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

unsafe extern "C" fn special_hi_h_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    // collect momentum
    let mut stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
    let mut speed = Vector2f{x: lua_bind::KineticEnergy::get_speed_x(stop_energy), y: lua_bind::KineticEnergy::get_speed_y(stop_energy)};
    let reset = *ENERGY_STOP_RESET_TYPE_AIR;
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let reset = *ENERGY_STOP_RESET_TYPE_GROUND;
        speed.y = 0.0;
    }
    // same motion but transfer remaining speed
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    let special_hi_speed_brake = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.special_hi_speed_brake");
    lua_bind::KineticEnergy::reset_energy(stop_energy, reset, &Vector2f{x: speed.x, y: speed.y}, &Vector3f::zero(), fighter.module_accessor);
    lua_bind::KineticEnergy::enable(stop_energy);
    sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, special_hi_speed_brake, special_hi_speed_brake);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    0.into()
}

unsafe extern "C" fn special_hi_h_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
        // starts on F15 to make the transition from grounded upb look smooth
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_h"), 15.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_h"), 0.0, 1.0, false, 0.0, false, false);
    }
    
    fighter.main_shift(special_hi_h_main_loop)
}

unsafe extern "C" fn special_hi_h_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    let frame = MotionModule::frame(fighter.module_accessor);

    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
        }
        else {
            if fighter.is_motion(Hash40::new("special_air_hi_h")) {
                let accel_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fall_special_accel_x_mul");
                let speed_x_max_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fall_special_speed_x_max_mul");
                WorkModule::set_float(fighter.module_accessor, accel_x_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_MUL_FALL_X_ACCEL);
                WorkModule::set_float(fighter.module_accessor, speed_x_max_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
            }
            else {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
            }
        }
    }
    else {
        if fighter.is_situation(*SITUATION_KIND_GROUND) && fighter.is_prev_situation(*SITUATION_KIND_AIR) {
            if frame <= 13.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_h"), frame + 15.0, 1.0, false, 0.0, false, false);
            }
            else {
                if frame >= 53.0 {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, false);
                }
            }
        }
    }

    return 0.into();
}

unsafe extern "C" fn special_hi_h_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_init);
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_pre);

    agent.status(Init, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2, special_hi2_init);
    agent.status(CheckAttack, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2, special_hi2_check_attack);

    agent.status(Pre, statuses::kirby::SPECIAL_HI_H, special_hi_h_pre);
    agent.status(Init, statuses::kirby::SPECIAL_HI_H, special_hi_h_init);
    agent.status(Main, statuses::kirby::SPECIAL_HI_H, special_hi_h_main);
    agent.status(End, statuses::kirby::SPECIAL_HI_H, special_hi_h_end);
}