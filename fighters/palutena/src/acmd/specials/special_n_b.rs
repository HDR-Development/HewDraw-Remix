use super::*;

unsafe extern "C" fn game_specialnb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let powered = VarModule::is_flag(agent.object(), vars::palutena::instance::POWERED);
    let power = if powered {5.0} else {0.0};
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !MeterModule::drain(boma.object(), 2) {
            MeterModule::drain(boma.object(), 1);
        }
        else {
            MeterModule::drain(boma.object(), 2);
        }
        VarModule::on_flag(boma.object(), vars::palutena::instance::FLUSH);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
    }
    FT_DESIRED_RATE(agent, 14.0, 8.0);
    frame(lua_state, 18.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0 + power, 88, 91, 0, 53, 4.5, 0.0, 15.0, 9.7, Some(0.0), Some(4.0), Some(9.7), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 1, 0, Hash40::new("top"), 7.0 + power, 88, 91, 0, 58, 2.8, 0.0, 30.0, 9.7, Some(0.0), Some(4.0), Some(9.7), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        if powered {
            ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 88, 91, 0, 62, 2.8, 0.0, 45.0, 9.7, Some(0.0), Some(4.0), Some(9.7), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        }
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(boma);
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            FT_DESIRED_RATE(agent, 17.0, 7.0);
        }
    }
    frame(lua_state, 43.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specialnb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let powered = VarModule::is_flag(agent.object(), vars::palutena::instance::POWERED);
    let length = if powered { 2.7 } else { 1.8 };
    let length2 = if powered { 0.69 } else { 0.5 };
    let y_pos = if powered {25} else {16};
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.05, 0.05, 0.90);
        EFFECT_FOLLOW(agent, Hash40::new("sys_freezer"), Hash40::new("top"), 0, 3, 10, 0, 0, 0, 0.75, true);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_pressure"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, length2, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.3);
        LAST_EFFECT_SET_RATE(agent, (16.0/10.0));
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 4, 21.5, 2, 0, -60, 0, 1, true, 0.7);
        LAST_EFFECT_SET_RATE(agent, 1.1);
        LAST_EFFECT_SET_COLOR(agent, 0.35, 0.35, 0.90);
        EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("top"), 0, y_pos, 10, 0, 250, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.35, 0.35, 0.90);
        EffectModule::set_scale_last(boma, &Vector3f::new(0.5, length, 0.5));
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_ice"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_pressure"), false, false);
    }
    if is_excute(agent) {
        let size = if powered { 0.6 } else { 0.5 };
        EFFECT_FOLLOW(agent, Hash40::new("sys_hit_ice"), Hash40::new("top"), 0, 3, 10, 0, 0, 0, 0.6, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_freezer"), Hash40::new("top"), 0, 24, 10, 0, 0, 0, size, true);
        if powered {
            EFFECT_FOLLOW(agent, Hash40::new("sys_freezer"), Hash40::new("top"), 0, 40, 10, 0, 0, 0, 0.3, true);
        }
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light2"), false, false);
    }
}

unsafe extern "C" fn sound_specialnb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let power = VarModule::is_flag(agent.object(), vars::palutena::instance::POWERED);
    let sound_lvl = if power {Hash40::new("se_common_frieze_l")} else {Hash40::new("se_common_frieze_m")};
    frame(lua_state, 17.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_palutena_attack07"));
        if power {
            PLAY_SE(agent, Hash40::new("se_palutena_smash_h01"));
        }
        PLAY_SE(agent, sound_lvl);
    }
}

unsafe extern "C" fn expression_specialnb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let powered = VarModule::is_flag(agent.object(), vars::palutena::instance::POWERED);

    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 13.0);
    app::sv_animcmd::execute(lua_state, 13.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        AREA_WIND_2ND_arg10(agent, 0, 2, 90, 300, 1, 9, 35, 18, 70, 80);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        if powered {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_beamm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        } else {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_beams"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnb", game_specialnb);
    agent.acmd("game_specialairnb", game_specialnb);
    agent.acmd("effect_specialnb", effect_specialnb);
    agent.acmd("effect_specialairnb", effect_specialnb);
    agent.acmd("sound_specialnb", sound_specialnb);
    agent.acmd("sound_specialairnb", sound_specialnb);
    agent.acmd("expression_specialnb", expression_specialnb);
    agent.acmd("expression_specialairnb", expression_specialnb);
}