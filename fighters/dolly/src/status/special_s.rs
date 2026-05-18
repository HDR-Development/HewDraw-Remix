use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_S

pub unsafe extern "C" fn special_s_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(fighter.battle_object, vars::dolly::instance::DISABLE_SPECIAL_S);
    }
    return false.into();
}

pub unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {

    // TODO: cancel into super special if needed..?

    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ATTACK_COMMAND1 | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    return false.into();
}

// FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND

pub unsafe extern "C" fn special_s_command_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(fighter.battle_object, vars::dolly::instance::DISABLE_SPECIAL_S);
    }
    return false.into();
}

// FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK

pub unsafe extern "C" fn special_f_attack_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.on_flag(*FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
        fighter.on_flag(*FIGHTER_DOLLY_STATUS_SPECIAL_S_WORK_FLAG_AIR_ATTACK);
    }

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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    return false.into();
}

// FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END

unsafe extern "C" fn special_f_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Reduce speed on shield
    let prev_inflict_status = VarModule::get_int(fighter.battle_object, vars::common::instance::PREV_STATUS_INFLICT_STATUS);
    if prev_inflict_status & *COLLISION_KIND_MASK_SHIELD != 0 || prev_inflict_status & *COLLISION_KIND_MASK_PARRY != 0 {
        let shield_hit_end_speed_x = if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_f.shield_hit_ground_end_speed_x")
        } else {
            ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_f.shield_hit_air_end_speed_x")
        };
        let lr = PostureModule::lr(fighter.module_accessor);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            shield_hit_end_speed_x * lr,
            0.0
        );
    }
    
    smashline::original_status(Main, fighter, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END)(fighter)
}

// FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B

pub unsafe extern "C" fn special_b_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(fighter.battle_object, vars::dolly::instance::DISABLE_SPECIAL_S);
    }
    // cap start speed
    let lr = PostureModule::lr(fighter.module_accessor);
    let mut speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * lr;
    let mut speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let start_speed_x_max = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_b.start_speed_x_max");
    SET_SPEED_EX(fighter, speed_x.min(start_speed_x_max) * lr, speed_y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    return false.into();
}

// FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND

pub unsafe extern "C" fn special_b_command_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(fighter.battle_object, vars::dolly::instance::DISABLE_SPECIAL_S);
    }
    // cap start speed
    let lr = PostureModule::lr(fighter.module_accessor);
    let mut speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * lr;
    let mut speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let start_speed_x_max = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_b.start_speed_x_max");
    SET_SPEED_EX(fighter, speed_x.min(start_speed_x_max) * lr, speed_y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    return false.into();
}

// FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_ATTACK

// pub unsafe extern "C" fn special_b_attack_init(fighter: &mut L2CFighterCommon) -> L2CValue {
//     if fighter.stick_x() * PostureModule::lr(fighter.module_accessor) > fighter.get_param_float("common", "status_start_turn_stick_x") {
//         PostureModule::reverse_lr(fighter.module_accessor);
//         PostureModule::update_rot_y_lr(fighter.module_accessor);
//         KineticModule::mul_speed(fighter.module_accessor, &Vector3f{x: -1.0, y: 1.0, z: 1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
//     }
//     return false.into();
// }

// pub unsafe extern "C" fn special_b_attack_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
//     if fighter.is_situation(*SITUATION_KIND_AIR) {
//         fighter.on_flag(*FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
//         fighter.on_flag(*FIGHTER_DOLLY_STATUS_SPECIAL_S_WORK_FLAG_AIR_ATTACK);
//     }

//     StatusModule::init_settings(
//         fighter.module_accessor,
//         app::SituationKind(*SITUATION_KIND_AIR),
//         *FIGHTER_KINETIC_TYPE_MOTION_AIR,
//         *GROUND_CORRECT_KIND_AIR as u32,
//         app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
//         true,
//         *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
//         *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
//         *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
//         0
//     );
//     FighterStatusModuleImpl::set_fighter_status_data(
//         fighter.module_accessor,
//         false,
//         *FIGHTER_TREADED_KIND_NO_REAC,
//         false,
//         false,
//         false,
//         (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ATTACK_COMMAND1 | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
//         *FIGHTER_STATUS_ATTR_START_TURN as u32,
//         *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
//         0
//     );
//     return false.into();
// }

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_init);
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_pre);
    agent.status(Pre, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK, special_f_attack_pre);
    agent.status(Main, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END, special_f_end_main);
    agent.status(Init, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND, special_s_command_init);
    agent.status(Init, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B, special_b_init);
    agent.status(Init, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND, special_b_command_init);
    // agent.status(Init, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_ATTACK, special_b_attack_init);
    // agent.status(Pre, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_ATTACK, special_b_attack_pre);
}