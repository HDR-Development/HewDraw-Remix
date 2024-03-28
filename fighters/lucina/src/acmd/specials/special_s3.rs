use super::*;

unsafe extern "C" fn game_specials3hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 125, 70, 0, 80, 4.5, 0.0, 6.0, 9.0, Some(0.0), Some(15.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 125, 70, 0, 80, 3.0, 0.0, 6.0, 12.0, Some(0.0), Some(16.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 125, 70, 0, 80, 4.5, 0.0, 9.0, 3.5, Some(0.0), Some(11.0), Some(3.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 27.0/(43.0-8.0));
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
}

unsafe extern "C" fn effect_specials3hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        FLASH(agent, 0, 0.24, 1, 0.7);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("lucina_sword_blue"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("lucina_mc_3hi"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::set_disable_render_offset_last(boma);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("lucina_sword_blue"), false, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

unsafe extern "C" fn sound_specials3hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
    PLAY_SE(agent, Hash40::new("vc_lucina_attack03"));
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
    PLAY_SE(agent, Hash40::new("se_lucina_special_s03h"));
    }
}

unsafe extern "C" fn game_specials3s(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 54, 86, 0, 57, 6.5, 0.0, 9.0, 9.0, Some(0.0), Some(11.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 54, 86, 0, 57, 6.0, 0.0, 9.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 54, 86, 0, 57, 4.0, 0.0, 9.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 25.0/(43.0-7.0));
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
}

unsafe extern "C" fn effect_specials3s(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0, 0.05, 0.7);
        EFFECT_FOLLOW(agent, Hash40::new("lucina_sword_red"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("lucina_mc_3s"), Hash40::new("top"), 0, -2.5, 2.5, 0, 0, 0, 1, true);
        EffectModule::set_disable_render_offset_last(boma);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("lucina_sword_red"), false, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

unsafe extern "C" fn sound_specials3s(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
    PLAY_SE(agent, Hash40::new("vc_lucina_attack05"));
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
    PLAY_SE(agent, Hash40::new("se_lucina_special_s03s"));
    }
}

unsafe extern "C" fn game_specials3lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 39, 39, 0, 67, 4.8, 0.0, 7.0, 10.0, Some(0.0), Some(7.0), Some(14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 39, 39, 0, 67, 3.0, 0.0, 7.0, 10.0, Some(0.0), Some(7.0), Some(19.3), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 23.0/(43.0-8.0));
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
}

unsafe extern "C" fn effect_specials3lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        FLASH(agent, 0, 0.93, 0.03, 0.7);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("lucina_sword_green"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("lucina_mc_3lw"), Hash40::new("top"), 0, -0.5, -1, 0, 0, 0, 1, true);
        EffectModule::set_disable_render_offset_last(boma);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("lucina_sword_green"), false, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

unsafe extern "C" fn sound_specials3lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
    PLAY_SE(agent, Hash40::new("se_lucina_special_s03l"));
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
    PLAY_SE(agent, Hash40::new("vc_lucina_attack02"));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specials3hi", game_specials3hi);
    agent.acmd("game_specialairs3hi", game_specials3hi);
    agent.acmd("effect_specials3hi", effect_specials3hi);
    agent.acmd("effect_specialairs3hi", effect_specials3hi);
    agent.acmd("sound_specials3hi", sound_specials3hi);
    agent.acmd("sound_specialairs3hi", sound_specials3hi);

    agent.acmd("game_specials3s", game_specials3s);
    agent.acmd("game_specialairs3s", game_specials3s);
    agent.acmd("effect_specials3s", effect_specials3s);
    agent.acmd("effect_specialairs3s", effect_specials3s);
    agent.acmd("sound_specials3s", sound_specials3s);
    agent.acmd("sound_specialairs3s", sound_specials3s);
    
    agent.acmd("game_specials3lw", game_specials3lw);
    agent.acmd("game_specialairs3lw", game_specials3lw);
    agent.acmd("effect_specials3lw", effect_specials3lw);
    agent.acmd("effect_specialairs3lw", effect_specials3lw);
    agent.acmd("sound_specials3lw", sound_specials3lw);
    agent.acmd("sound_specialairs3lw", sound_specials3lw);
}