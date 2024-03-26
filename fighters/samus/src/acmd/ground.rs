
use super::*;

unsafe extern "C" fn game_attackdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.85);
    frame(lua_state, 6.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::samus::instance::SHINESPARK_USED);
        if (ControlModule::get_stick_y(boma) < -0.5) {
            if VarModule::is_flag(agent.battle_object, vars::samus::instance::SHINESPARK_READY) {
                VarModule::off_flag(agent.battle_object, vars::samus::instance::SHINESPARK_READY);
                VarModule::on_flag(agent.battle_object, vars::samus::instance::SHINESPARK_USED);
                MotionModule::set_rate(boma, 0.25);
            }
        }
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 13.0, 55, 80, 0, 76, 5.5, 0.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("bust"), 13.0, 55, 80, 0, 76, 4.5, 0.0, 0.0, 0.0, Some(0.0), Some(9.0), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        if VarModule::is_flag(agent.battle_object, vars::samus::instance::SHINESPARK_USED) {
            MotionModule::set_rate(boma, 0.5);
            let addSpeed1 = Vector3f { x: 1.6, y: 0.0, z: 0.0 };
            KineticModule::add_speed(boma, &addSpeed1);
            ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 361, 110, 0, 70, 4.5, 0.0, 9.0, 3.0, Some(0.0), Some(9.0), Some(2.0), 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 16.0, 361, 110, 0, 70, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(9.0), Some(2.0), 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            ATTACK(agent, 2, 0, Hash40::new("bust"), 16.0, 361, 110, 0, 70, 4.5, 0.0, 0.0, 0.0, Some(0.0), Some(9.0), Some(2.0), 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            ATK_SET_SHIELD_SETOFF_MUL_arg4(agent, 0, 1, 2, 0.2);
        }
                }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 13.0, 55, 80, 0, 76, 5.5, 0.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("bust"), 13.0, 55, 80, 0, 76, 5.0, 0.0, 0.0, 0.0, Some(0.0), Some(9.0), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        if VarModule::is_flag(agent.battle_object, vars::samus::instance::SHINESPARK_USED) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 361, 115, 0, 75, 5.3, 0.0, 9.0, 6.0, Some(0.0), Some(9.0), Some(4.0), 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 13.0, 361, 115, 0, 75, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(9.0), Some(2.0), 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            ATTACK(agent, 2, 0, Hash40::new("bust"), 13.0, 361, 115, 0, 75, 4.5, 0.0, 0.0, 0.0, Some(0.0), Some(9.0), Some(2.0), 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            ATK_SET_SHIELD_SETOFF_MUL_arg4(agent, 0, 1, 2, 0.2);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 9.0, 55, 80, 0, 76, 5.5, 0.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("bust"), 9.0, 55, 80, 0, 76, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(9.0), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        if VarModule::is_flag(agent.battle_object, vars::samus::instance::SHINESPARK_USED) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 361, 120, 0, 80, 4.0, 0.0, 9.0, 6.0, Some(0.0), Some(9.0), Some(5.0), 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 10.0, 361, 120, 0, 80, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(9.0), Some(2.0), 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            ATTACK(agent, 2, 0, Hash40::new("bust"), 10.0, 361, 120, 0, 80, 4.5, 0.0, 0.0, 0.0, Some(0.0), Some(9.0), Some(2.0), 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            ATK_SET_SHIELD_SETOFF_MUL_arg4(agent, 0, 1, 2, 0.2);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 21.0/(34.0-17.0));
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if VarModule::is_flag(agent.battle_object, vars::samus::instance::SHINESPARK_USED) {
            let subSpeed1 = Vector3f { x: -1.6, y: 0.0, z: 0.0 };
            KineticModule::add_speed(boma, &subSpeed1);
            MotionModule::set_rate(boma, 0.25);
        }
    }
	frame(lua_state, 30.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if VarModule::is_flag(agent.battle_object, vars::samus::instance::SHINESPARK_USED) {
            MotionModule::set_rate(boma, 1.0);
        }
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
    }

}

unsafe extern "C" fn sound_attackdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_samus_attackdash"));
        if VarModule::is_flag(agent.battle_object, vars::samus::instance::SHINESPARK_USED)  {
            PLAY_SE(agent, Hash40::new("se_samus_catch"));
            PLAY_SE_REMAIN(agent, Hash40::new("se_samus_special_n05"));
        }
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_samus_step_left_m"));
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_samus_step_right_m"));
    }

}

unsafe extern "C" fn game_attack11(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 70, 40, 0, 35, 3.5, 0.0, 9.5, 8.0, Some(0.0), Some(9.5), Some(16.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.5, 361, 10, 0, 25, 2.5, 0.0, 3.5, 8.0, Some(0.0), Some(3.5), Some(15.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        //WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
    }

}

unsafe extern "C" fn effect_attack11(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 9.5, 2, 0, 0, 0, 0.95, true);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), -1, 9.5, 18, 0, 0, 0, 1.15, 0, 0, 0, 0, 0, 360, true);
    }
}

unsafe extern "C" fn expression_attack12(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackdash", game_attackdash);
    agent.acmd("sound_attackdash", sound_attackdash);
    agent.acmd("game_attack11", game_attack11);
    agent.acmd("effect_attack11", effect_attack11);
    agent.acmd("expression_attack12", expression_attack12);
}
