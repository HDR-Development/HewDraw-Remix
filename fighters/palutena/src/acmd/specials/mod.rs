use super::*;

mod special_n_r;
mod special_n_o;
mod special_n_y;
mod special_n_g;
mod special_n_b;
mod special_n_p;

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 72, 40, 0, 70, 8.0, 0.0, 12.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), -0.2, 22, -1, 10, 90, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.1, 0.1, 0.15);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light_trace"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.1, 0.1, 0.15);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.1, 0.1, 0.15);
        EFFECT(agent, Hash40::new("palutena_wand_finish"), Hash40::new("top"), 0.0, 12.0, 10.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.1, 0.1, 0.15);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light_trace"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light2"), false, false);
    }
}

unsafe extern "C" fn sound_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_palutena_special_n01"));
    }
    wait(lua_state, 22.0);
    if is_excute(agent) {
        sound!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
    }
}

unsafe extern "C" fn expression_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let current_damage = DamageModule::damage(boma, 0);
    frame(lua_state, 4.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 20.0);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        VarModule::set_float(boma.object(), vars::palutena::status::ADD_DAMAGE, DamageModule::damage(boma, 0) - current_damage);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0 + (VarModule::get_float(boma.object(), vars::palutena::status::ADD_DAMAGE) * 0.75), 361, 50, 0, 70, 7.0, 0.0, 10.5, 13.0, Some(0.0), Some(10.5), Some(14.5), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 1, 0, Hash40::new("top"), 12.0 + (VarModule::get_float(boma.object(), vars::palutena::status::ADD_DAMAGE) * 0.75), 361, 30, 0, 65, 9.0, 0.0, 10.5, 9.0, Some(0.0), Some(10.5), Some(20.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_MAGIC);
        AttackModule::set_force_reaction(boma, 0, true, false);
        AttackModule::set_force_reaction(boma, 1, true, false);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("palutena_counter_flash"), Hash40::new("shield"), -1, 0, -3, 0, 0, 0, 0.7, true);
        EFFECT(agent, Hash40::new("palutena_counter_success"), Hash40::new("top"), 0, 14.8, -1, 0, 90, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("sys_counter_flash"), Hash40::new("top"), 0, 14.8, -1, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("palutena_backlight"), Hash40::new("waist"), 10, -3, -1, 0, 90, 0, 1, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(agent, 2.75);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        FLASH(agent, 1, 1, 1, 0);
        EFFECT_FLW_POS(agent, Hash40::new("palutena_counter_attack"), Hash40::new("stick"), 0, 8.5, 0, 0, 0, 0, 0.9, true);
        EffectModule::set_disable_render_offset_last(boma);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("palutena_backlight"), -1);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

unsafe extern "C" fn sound_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_palutena_special_l01"));
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_palutena_special_l02"));
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_palutena_special_l03"));
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_palutena_rnd_special_l01"));
    }
}

unsafe extern "C" fn expression_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_counter"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn", game_specialn);
    agent.acmd("game_specialairn", game_specialn);
    agent.acmd("effect_specialn", effect_specialn);
    agent.acmd("effect_specialairn", effect_specialn);
    agent.acmd("sound_specialn", sound_specialn);
    agent.acmd("sound_specialairn", sound_specialn);
    agent.acmd("expression_specialn", expression_specialn);
    agent.acmd("expression_specialairn", expression_specialn);
    special_n_r::install(agent);
    special_n_o::install(agent);
    special_n_y::install(agent);
    special_n_g::install(agent);
    special_n_b::install(agent);
    special_n_p::install(agent);

    agent.acmd("game_speciallw", game_speciallw);
    agent.acmd("game_specialairlw", game_speciallw);
    agent.acmd("effect_speciallw", effect_speciallw);
    agent.acmd("effect_specialairlw", effect_speciallw);
    agent.acmd("sound_speciallw", sound_speciallw);
    agent.acmd("sound_specialairlw", sound_speciallw);
    agent.acmd("expression_speciallw", expression_speciallw);
    agent.acmd("expression_specialairlw", expression_speciallw);
}