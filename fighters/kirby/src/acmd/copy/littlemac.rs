use super::*;

unsafe extern "C" fn game_littlemacspecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        let damage =  22.0 * if agent.is_situation(*SITUATION_KIND_GROUND) { 1.0 } else { 0.8 };
        let angle = if agent.is_situation(*SITUATION_KIND_GROUND) { 80 } else { 75 };
        let bkb = if agent.is_situation(*SITUATION_KIND_GROUND) { 40 } else { 30 };
        let kbg = if agent.is_situation(*SITUATION_KIND_GROUND) { 104 } else { 124 };
        let shield_damage = if agent.is_situation(*SITUATION_KIND_GROUND) { 2 } else { 0 };
        ATTACK(agent, 0, 0, Hash40::new("armr"), damage, angle, kbg, 0, bkb, 5.0, 3.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), damage, angle, kbg, 0, bkb, 3.0, -1.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("shoulderr"), damage, angle, kbg, 0, bkb, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 3, 0, Hash40::new("bust"), damage, angle, kbg, 0, bkb, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        AttackModule::set_damage_shake_scale(boma, 0.67);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 1, false);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.object(), vars::kirby::status::KO_PUNCH_GRAVITY);
    }
    
}

unsafe extern "C" fn effect_littlemacspecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        let size = 1.0;
        EFFECT_FLW_POS(agent, Hash40::new("littlemac_ko_uppercut_start"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, size, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("littlemac_ko_uppercut"), Hash40::new("handr"), 0.5, 0, 0, 0, 0, 0, size, true);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("littlemac_ko_uppercut_start"), -1);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
            EFFECT_FOLLOW(agent, Hash40::new("littlemac_ko_uppercut_arc"), Hash40::new("rot"), 0.5, 1, -3, 0, -60, 70, 1, true);
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("littlemac_ko_uppercut_arc_splash"), Hash40::new("rot"), 0.5, 1, -3, 0, -60, 70, 1, false);
        }
        else {
            EFFECT_FOLLOW(agent, Hash40::new("littlemac_ko_uppercut_arc"), Hash40::new("rot"), -4, 1, -3, -15, -60, 90, 1, true);
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("littlemac_ko_uppercut_arc_splash"), Hash40::new("rot"), -4, 1, -3, -15, -60, 90, 1, false);
        }
    }
    frame(lua_state, 8.0);
    let mut handle = EffectModule::req_follow(boma, Hash40::new("sys_starrod_bullet"), Hash40::new("handr"), &Vector3f::new(3.0, 0.0, 0.0), &Vector3f::new(45.0, 135.0, 45.0), 0.3, false, 0, 0, 0, 0, 0, false, false);
    if is_excute(agent) {
        EffectModule::set_rate(boma, handle as u32, 1.5);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EffectModule::set_scale(boma, handle as u32, &Vector3f::new(0.8, 0.8, 0.8));
        let facing = PostureModule::lr(boma);
        EffectModule::set_rot(boma, handle as u32, &Vector3f::new(45.0 * facing, 135.0, 45.0 * facing));
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("littlemac_ko_uppercut"), false, false);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_starrod_bullet"), false, false);
    }
}

unsafe extern "C" fn sound_littlemacspecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_littlemac_special_n03"));
        PLAY_SE(agent, Hash40::new("vc_kirby_hammermax"));
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_littlemac_swing_ll"));
    }
}

unsafe extern "C" fn expression_littlemacspecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_elecattack"), 0);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        AREA_WIND_2ND_arg10(agent, 0, 4, 45, 200, 1, 17, 15, 38, 30, 50);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_littlemacspecialn", game_littlemacspecialn);
    agent.acmd("game_littlemacspecialairn", game_littlemacspecialn);
    agent.acmd("effect_littlemacspecialn", effect_littlemacspecialn);
    agent.acmd("effect_littlemacspecialairn", effect_littlemacspecialn);
    agent.acmd("sound_littlemacspecialn", sound_littlemacspecialn);
    agent.acmd("sound_littlemacspecialairn", sound_littlemacspecialn);
    agent.acmd("expression_littlemacspecialn", expression_littlemacspecialn);
    agent.acmd("expression_littlemacspecialairn", expression_littlemacspecialn);
}