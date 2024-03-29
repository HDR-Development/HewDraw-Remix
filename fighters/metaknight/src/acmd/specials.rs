use super::*;

unsafe extern "C" fn game_specialnspin(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 367, 0, 0, 40, 8.0, 0.0, 8.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 367, 0, 0, 40, 8.0, 0.0, 11.0, 5.0, Some(0.0), Some(11.0), Some(-5.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 367, 0, 0, 40, 9.0, 0.0, 8.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
}

unsafe extern "C" fn game_specialnend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ATTACK(agent, 0, 1, Hash40::new("top"), 5.0, 90, 120, 0, 50, 12.0, 0.0, 10.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialnend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("metaknight_tornado_smoke_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        EFFECT_OFF_KIND(agent, Hash40::new("metaknight_tornado"), false, false);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_tornado_end"), Hash40::new("trans"), 0, 0, 0, 0, 0, 0, 0.4, false);
    }
}

unsafe extern "C" fn effect_specialnspingroundeffect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("metaknight_tornado_smoke_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
        WorkModule::set_float(boma, 500.0, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_FLOAT_GROUND_EFFECT_COUNTER);
    }
}

unsafe extern "C" fn effect_specialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::set_int(boma, 10, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_EFFECT_START_FRAME);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("metaknight_tornado_smoke_l"), Hash40::new("top"), 0, -4.5, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_specialairnend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("metaknight_tornado"), false, false);
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_tornado_end"), Hash40::new("trans"), 0, 0, 0, 0, 0, 0, 0.4, false);
    }
}

unsafe extern "C" fn game_specialairsfinish(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_status(boma, true);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 10.0, 70, 60, 0, 80, 5.5, 0.0, 5.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 10.0, 70, 60, 0, 80, 5.5, 0.0, 1.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"),   10.0, 70, 60, 0, 40, 8.0, 0.0, 6.5, 1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MotionModule::set_rate(boma, 2.0);
    }
}

unsafe extern "C" fn game_specialsdrill(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
        ATTACK(agent, 0, 0, Hash40::new("haver"), 1.1, 367, 50, 30, 80, 3.5, 0.0, 5.0, 0.0, None, None, None, 0.6, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 1.1, 367, 50, 30, 80, 3.5, 0.0, 1.0, 0.0, None, None, None, 0.6, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"),   1.1, 367, 50, 30, 80, 6.0, 0.0, 6.5, 1.0, None, None, None, 0.6, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), false);
    }
}

unsafe extern "C" fn effect_specialsdrill(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_drilllush"), Hash40::new("haver"), 0, 16, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 2.0);
    for i in 1..8 {
        wait(lua_state, 1.0);
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_spin_wind_s"), Hash40::new("rot"), 0, 0, 20, 90, 0, 0, 0.5, 0, 0, 10, 20, 15, 360, true);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_spin_wind_s"), Hash40::new("rot"), 0, 0, 15, 90, 0, 0, 0.8, 0, 0, 10, 20, 15, 360, true);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_spin_wind_s"), Hash40::new("rot"), 0, 0, 10, 90, 0, 0, 1.1, 0, 0, 10, 20, 15, 360, true);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_spin_wind_s"), Hash40::new("rot"), 0, 0, 5, 90, 0, 0, 1.4, 0, 0, 10, 20, 15, 360, true);
        }
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_drilllush_end"), Hash40::new("haver"), 0, 16, 0, 0, 0, 0, 1, false);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("metaknight_drilllush_end_wind"), Hash40::new("haver"), 0, 16, 0, 0, 0, 0, 1, false);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 45, 61, 0, 90, 4.0, 0.0, 0.0, 6.0, Some(0.0), Some(12.0), Some(8.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 45, 61, 0, 90, 4.0, 0.0, 0.0, 12.0, Some(0.0), Some(12.0), Some(14.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if agent.is_stick_backward() {
            PostureModule::reverse_lr(agent.module_accessor);
            PostureModule::update_rot_y_lr(agent.module_accessor);
        }
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 86, 120, 125, 0, 4.0, 0.0, 0.0, 6.0, Some(0.0), Some(12.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 88, 120, 125, 0, 4.0, 0.0, 0.0, 12.0, Some(0.0), Some(12.0), Some(14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 84, 100, 68, 0, 4.0, 0.0, 0.0, 6.0, Some(0.0), Some(12.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 84, 100, 68, 0, 4.0, 0.0, 0.0, 12.0, Some(0.0), Some(12.0), Some(14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        GroundModule::set_passable_check(boma, true);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 7.0, 80, 90, 0, 55, 7.7, 0.0, 3.5, -2.0, Some(0.0), Some(10.0), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("bust"), 7.0, 80, 90, 0, 55, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        GroundModule::set_passable_check(boma, false);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        sv_kinetic_energy!(
            reset_energy,
            agent,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let air_speed_x_stable = WorkModule::get_param_float(boma, hash40("air_speed_x_stable"), 0);
        let fall_x_mul = WorkModule::get_param_float(
            boma,
            hash40("param_special_hi"),
            hash40("fall_x_mul")
        );
        sv_kinetic_energy!(
            set_stable_speed,
            agent,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            air_speed_x_stable * fall_x_mul,
            0.0
        );
    }
}

unsafe extern "C" fn game_specialhiloop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 45, 61, 0, 90, 4.0, 0.0, 0.0, 6.0, Some(0.0), Some(12.0), Some(8.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 45, 61, 0, 90, 4.0, 0.0, 0.0, 12.0, Some(0.0), Some(12.0), Some(14.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if agent.is_stick_backward() {
            PostureModule::reverse_lr(agent.module_accessor);
            PostureModule::update_rot_y_lr(agent.module_accessor);
        }
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 86, 120, 125, 0, 4.0, 0.0, 0.0, 6.0, Some(0.0), Some(12.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 88, 120, 125, 0, 4.0, 0.0, 0.0, 12.0, Some(0.0), Some(12.0), Some(14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 84, 100, 68, 0, 4.0, 0.0, 0.0, 6.0, Some(0.0), Some(12.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 84, 100, 68, 0, 4.0, 0.0, 0.0, 12.0, Some(0.0), Some(12.0), Some(14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(boma);
        GroundModule::set_passable_check(boma, true);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 7.0, 80, 90, 0, 55, 7.7, 0.0, 3.5, -2.0, Some(0.0), Some(10.0), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("bust"), 7.0, 80, 90, 0, 55, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(boma);
        GroundModule::set_passable_check(boma, false);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        sv_kinetic_energy!(
            reset_energy,
            agent,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let air_speed_x_stable = WorkModule::get_param_float(boma, hash40("air_speed_x_stable"), 0);
        let fall_x_mul = WorkModule::get_param_float(
            boma,
            hash40("param_special_hi"),
            hash40("fall_x_mul")
        );
        sv_kinetic_energy!(
            set_stable_speed,
            agent,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            air_speed_x_stable * fall_x_mul,
            0.0
        );
    }
}

unsafe extern "C" fn effect_specialhiloop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent){
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 3, 10, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 2.0);
    if is_excute(agent){
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_shuttleloop1"), Hash40::new("top"), 0, 0, -2.5, 4, 0, 0, 1, true);
        EffectModule::set_disable_render_offset_last(agent.boma());
    }
    frame(lua_state, 14.0);
    if is_excute(agent){
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_shuttleloop2"), Hash40::new("top"), 0, -25, -4, 1, 0, 0, 1, true);
        EffectModule::set_disable_render_offset_last(agent.boma());
    }
    frame(lua_state, 23.0);
    if is_excute(agent){
        EFFECT_OFF_KIND(agent, Hash40::new("metaknight_sword"), false, false);
    }
}

unsafe extern "C" fn effect_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 5.0);
    if is_excute(agent){
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 3, 10, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent){
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_shuttleloop1"), Hash40::new("top"), 0, 0, -2.5, 4, 0, 0, 1, true);
        EffectModule::set_disable_render_offset_last(agent.boma());
    }
    frame(lua_state, 22.0);
    if is_excute(agent){
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_shuttleloop2"), Hash40::new("top"), 0, -25, -4, 1, 0, 0, 1, true);
        EffectModule::set_disable_render_offset_last(agent.boma());
    }
    frame(lua_state, 30.0);
    if is_excute(agent){
        EFFECT_OFF_KIND(agent, Hash40::new("metaknight_sword"), false, false);
    }
}

unsafe extern "C" fn game_speciallwf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_NO_HOP);
        ArticleModule::change_motion(boma, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, smash::phx::Hash40::new("special_air_lw_f"), false, 0.0);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 361, 114, 0, 20, 8.5, 0.0, 7.5, 6.0, Some(0.0), Some(7.5), Some(8.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 361, 114, 0, 20, 5.5, 0.0, 6.4, 20.0, Some(0.0), Some(6.4), Some(-2.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_SITUATION_CHANGE);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, true);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn game_speciallwsubf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 361, 114, 0, 20, 8.5, 0.0, 7.5, 6.0, Some(0.0), Some(7.5), Some(8.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 361, 114, 0, 20, 5.5, 0.0, 6.4, 20.0, Some(0.0), Some(6.4), Some(-2.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_SITUATION_CHANGE);
    }
}

unsafe extern "C" fn game_specialairlwf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ArticleModule::change_motion(boma, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, smash::phx::Hash40::new("special_air_lw_f"), false, 0.0);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 53, 114, 0, 20, 11.2, 0.0, 7.5, 6.0, Some(0.0), Some(7.5), Some(8.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 53, 114, 0, 20, 7.5, 0.0, 6.4, 21.0, Some(0.0), Some(6.4), Some(-2.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_SITUATION_CHANGE);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, true);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn game_speciallwsubairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 53, 114, 0, 20, 11.2, 0.0, 7.5, 6.0, Some(0.0), Some(7.5), Some(8.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 53, 114, 0, 20, 7.5, 0.0, 6.4, 21.0, Some(0.0), Some(6.4), Some(-2.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_SITUATION_CHANGE);
    }
}

unsafe extern "C" fn game_speciallwb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        REVERSE_LR(agent);
        ArticleModule::change_motion(boma, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, smash::phx::Hash40::new("special_lw_b"), false, 0.0);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 361, 114, 0, 20, 7.5, 0.0, 7.5, -6.0, Some(0.0), Some(7.5), Some(-8.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 361, 114, 0, 20, 5.5, 0.0, 6.5, -16.5, Some(0.0), Some(6.5), Some(2.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_SITUATION_CHANGE);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, true);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn game_speciallwsubb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 361, 114, 0, 20, 7.5, 0.0, 7.5, -6.0, Some(0.0), Some(7.5), Some(-8.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 361, 114, 0, 20, 5.5, 0.0, 6.5, -16.5, Some(0.0), Some(6.5), Some(2.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_SITUATION_CHANGE);
    }
}

unsafe extern "C" fn game_specialairlwb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        REVERSE_LR(agent);
        ArticleModule::change_motion(boma, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, smash::phx::Hash40::new("special_lw_b"), false, 0.0);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 361, 114, 0, 20, 11.2, 0.0, 7.5, -6.0, Some(0.0), Some(7.5), Some(-8.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 361, 114, 0, 20, 7.5, 0.0, 6.5, -21.0, Some(0.0), Some(6.5), Some(2.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_SITUATION_CHANGE);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, true);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn game_speciallwsubairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 361, 114, 0, 20, 11.2, 0.0, 7.5, -6.0, Some(0.0), Some(7.5), Some(-8.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 361, 114, 0, 20, 7.5, 0.0, 6.5, -21.0, Some(0.0), Some(6.5), Some(2.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_SITUATION_CHANGE);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnspin", game_specialnspin);
    agent.acmd("game_specialairnend", game_specialnend);
    agent.acmd("game_specialnend", game_specialnend);
    agent.acmd("effect_specialnend", effect_specialnend);
    agent.acmd("effect_specialnspingroundeffect", effect_specialnspingroundeffect);
    agent.acmd("effect_specialnstart", effect_specialnstart);
    agent.acmd("effect_specialairnend", effect_specialairnend);

    agent.acmd("game_specialairsfinish", game_specialairsfinish);
    agent.acmd("game_specialsend", game_specialairsfinish);
    agent.acmd("game_specialsdrill", game_specialsdrill);
    agent.acmd("effect_specialsdrill", effect_specialsdrill);

    agent.acmd("game_specialhi", game_specialhi);
    agent.acmd("game_specialhiloop", game_specialhiloop);
    agent.acmd("effect_specialhiloop", effect_specialhiloop);
    agent.acmd("effect_specialhi", effect_specialhi);

    agent.acmd("game_speciallwf", game_speciallwf);
    agent.acmd("game_speciallwsubf", game_speciallwsubf);
    agent.acmd("game_specialairlwf", game_specialairlwf);
    agent.acmd("game_speciallwsubairf", game_speciallwsubairf);
    agent.acmd("game_speciallwb", game_speciallwb);
    agent.acmd("game_speciallwsubb", game_speciallwsubb);
    agent.acmd("game_specialairlwb", game_specialairlwb);
    agent.acmd("game_speciallwsubairb", game_speciallwsubairb);
}