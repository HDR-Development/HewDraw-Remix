use super::*;

unsafe extern "C" fn game_attack11(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 70, 18, 0, 25, 3.5, 0.0, 11.0, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 70, 18, 0, 25, 4.0, 0.0, 8.6, 15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 90, 18, 0, 25, 4.5, 0.0, 8.0, 20.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    wait(lua_state, 8.0);
    if is_excute(agent) {
        //WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
    }
}

unsafe extern "C" fn effect_attack11(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 5, 14, -2, 10, -20, 0, 1.4, true, *EF_FLIP_YZ);
        LAST_PARTICLE_SET_COLOR(agent, 1, 0.544, 0.54);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 5, 16, -6, 10, -20, 0, 1.8, true, *EF_FLIP_YZ);
        LAST_PARTICLE_SET_COLOR(agent, 1, 0.544, 0.54);
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 10, 22, 0, 0, 0, 1.7, 0, 0, 0, 0, 0, 360, true, 0.7);
    }
}

unsafe extern "C" fn game_attack12(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        ATTACK(agent, 0, 0, Hash40::new("armr"), 7.0, 40, 100, 0, 50, 5.0, 6.0, 0.0, -0.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 7.0, 40, 100, 0, 50, 4.0, 0.0, 1.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 7.0, 40, 100, 0, 50, 3.5, -5.0, 1.5, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

unsafe extern "C" fn effect_attack12(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -7, 21, -10, 25, 10, 0, 1.4, true, *EF_FLIP_YZ);
        LAST_PARTICLE_SET_COLOR(agent, 1, 0.544, 0.54);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -7, 23, -4, 25, 10, 0, 1.7, true, *EF_FLIP_YZ);
        LAST_PARTICLE_SET_COLOR(agent, 1, 0.544, 0.54);
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 10, 25, 0, 0, 0, 1.7, 0, 0, 0, 0, 0, 360, true, 0.5);
    }
}

unsafe extern "C" fn game_attackdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.7);
    frame(lua_state, 6.0);
    FT_MOTION_RATE_RANGE(agent, 6.0, 14.0, 4.0);
    frame(lua_state, 14.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("handr"), 12.0, 80, 70, 0, 80, 6.0, 3.5, -0.6, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 10.0, 80, 70, 0, 85, 5.0, 0.0, 0.0, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 80, 70, 0, 85, 4.5, 0.0, 4.0, 4.5, Some(0.0), Some(7.0), Some(4.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attackdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    frame(lua_state, 13.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("koopa_scratch"), Hash40::new("koopa_scratch"), Hash40::new("top"), -2, 20, 5.5, 19, -8, -52, 1.6, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        EffectModule::set_visible_kind(boma, Hash40::new("koopa_scratch"), false);

        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -4, 13, -30, 0, 0, 0, 2.5, true);
        LAST_EFFECT_SET_RATE(agent, 1.75);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EffectModule::set_visible_kind(boma, Hash40::new("koopa_scratch"), true);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        if PostureModule::lr(boma) > 0.0 {
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), 18.0, 2.0, 0.0, 0,0,140,0.9, 0, 0, 0, 0, 0, 0, false);
        }
        else{
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), 18.0, 2.0, 0.0, 0,0,-140,0.9, 0, 0, 0, 0, 0, 0, false);
        }
        LAST_EFFECT_SET_RATE(agent,1.5);
    }
}

unsafe extern "C" fn sound_attackdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    frame(lua_state, 15.0);
    if is_excute(agent) {
        let play_vc = app::sv_math::rand(hash40("fighter"), 3) == 0;
        if play_vc {
            PLAY_SE(agent, Hash40::new("vc_koopa_attack07"));
        }
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_koopa_smash_l02"));
    }
}

unsafe extern "C" fn expression_attackdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attack11", game_attack11);
    agent.acmd("effect_attack11", effect_attack11);
    agent.acmd("game_attack12", game_attack12);
    agent.acmd("effect_attack12", effect_attack12);
    
    agent.acmd("game_attackdash", game_attackdash);
    agent.acmd("effect_attackdash", effect_attackdash);
    agent.acmd("sound_attackdash", sound_attackdash);
    agent.acmd("expression_attackdash", expression_attackdash);
}
