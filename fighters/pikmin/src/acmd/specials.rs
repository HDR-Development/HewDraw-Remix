use super::*;

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PIKMIN_STATUS_SPECIAL_S_FLAG_THROW);
    }
}

unsafe extern "C" fn game_specialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_PIKMIN_GENERATE_ARTICLE_PIKMIN, false, -1);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, WorkModule::get_float(boma, *FIGHTER_PIKMIN_STATUS_PULL_OUT_WORK_FLOAT_MOT_RATE));
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_AIR)
        && !VarModule::is_flag(agent.battle_object, vars::common::instance::SPECIAL_STALL_USED) {
            VarModule::on_flag(agent.battle_object, vars::common::instance::SPECIAL_STALL_USED);
            if KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) < 0.0 {
                KineticModule::mul_speed(boma, &Vector3f{x: 1.0, y: 0.0, z: 1.0}, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
        }

        WorkModule::on_flag(boma, *FIGHTER_PIKMIN_STATUS_SPECIAL_LW_FLAG_SORT);
        // damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("top"), 7.5, 0.0, 7.0, -8.5, 0.0, 7.0, 8.5, 1.5, 1.0, 50, false, 0.8, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        // damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
}

unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 0.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("pikmin_order"), Hash40::new("s_antenna4"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("pikmin_seiretsu"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 0.55, true);
        LAST_EFFECT_SET_RATE(agent, 2.0);
    }
}

unsafe extern "C" fn game_specialnfailure(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 80, 223, 0, 20, 4.0, 0.0, 2.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialnfailure(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0.0, 2.0, 3.0, 0.0, 0.0, 0.0, 1.0, true);
    }
}

unsafe extern "C" fn game_specialairnfailure(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 80, 223, 0, 20, 4.0, 0.0, 2.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialairnfailure(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0.0, 2.0, 3.0, 0.0, 0.0, 0.0, 1.0, true);
    }
}

unsafe extern "C" fn sound_specialairnfailure(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_pikmin_special_n02"));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specials", game_specials, Priority::Low);
    agent.acmd("game_specialairs", game_specials, Priority::Low);
    agent.acmd("game_specialnstart", game_specialnstart, Priority::Low);
    agent.acmd("game_specialnfailure", game_specialnfailure, Priority::Low);
    agent.acmd("effect_specialnfailure", effect_specialnfailure, Priority::Low);
    agent.acmd("game_specialairnfailure", game_specialairnfailure, Priority::Low);
    agent.acmd("effect_specialairnfailure", effect_specialairnfailure, Priority::Low);
    agent.acmd("sound_specialairnfailure", sound_specialairnfailure, Priority::Low);
    agent.acmd("game_speciallw", game_speciallw, Priority::Low);
    agent.acmd("game_specialairlw", game_speciallw, Priority::Low);
    agent.acmd("effect_speciallw", effect_speciallw, Priority::Low);
    agent.acmd("effect_specialairlw", effect_speciallw, Priority::Low);
}