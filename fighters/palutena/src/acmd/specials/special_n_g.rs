use super::*;

unsafe extern "C" fn game_specialng(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        MeterModule::drain(boma.object(), 2);
        VarModule::on_flag(boma.object(), vars::palutena::instance::FLUSH);
    }
    FT_DESIRED_RATE(agent, 20.0, 10.0);
    frame(lua_state, 20.0);
    FT_DESIRED_RATE(agent, 25.0, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.5, 366, 40, 70, 0, 6.0, 0.0, 19.0, 9.7, Some(0.0), Some(4.0), Some(9.7), 1.0, 0.7, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 5, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
    }
    wait(lua_state, 25.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        let angle_mod = ((sv_math::rand(hash40("fighter"), 51) as i32) - 25) as u64;
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 90 + angle_mod, 116, 0, 75, 8.0, 0.0, 21.0, 9.7, Some(0.0), Some(4.0), Some(9.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialng(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), -0.2, 22, -1, 10, 90, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.05, 0.50, 0.01);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light_trace"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.05, 0.50, 0.01);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.05, 0.50, 0.01);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_club_tornado"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.05, 1.0, 0.05);
        EFFECT_FOLLOW(agent, Hash40::new("sys_club_tornado"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 0.95, true);
        LAST_EFFECT_SET_COLOR(agent, 0.05, 0.50, 0.05);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_club_tornado"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(agent, 0.25, 0.70, 0.25);
        LAST_EFFECT_SET_ALPHA(agent, 0.3);
    }
    wait(lua_state, 26.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("sys_club_tornado"), -1);
        EFFECT_DETACH_KIND(agent, Hash40::new("sys_club_tornado"), -1);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light_trace"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light2"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_club_tornado"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_club_tornado"), false, false);
    }
}

unsafe extern "C" fn sound_specialng(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_palutena_special_n01"));
        PLAY_SE(agent, Hash40::new("se_common_slip_01"));
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        sound!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
        PLAY_SE(agent, Hash40::new("se_palutena_throw"));
        PLAY_SE(agent, Hash40::new("se_common_throw_02"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_palutena_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_item_club_wind"));
    }
    wait(lua_state, 25.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_item_club_wind"));
    }
}

unsafe extern "C" fn expression_specialng(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_27_spinslash"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashss"), 10);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 10);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialng", game_specialng);
    agent.acmd("game_specialairng", game_specialng);
    agent.acmd("effect_specialng", effect_specialng);
    agent.acmd("effect_specialairng", effect_specialng);
    agent.acmd("sound_specialng", sound_specialng);
    agent.acmd("sound_specialairng", sound_specialng);
    agent.acmd("expression_specialng", expression_specialng);
    agent.acmd("expression_specialairng", expression_specialng);
}