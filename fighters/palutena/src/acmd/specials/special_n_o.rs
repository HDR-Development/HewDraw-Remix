use super::*;

unsafe extern "C" fn game_specialno(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        MeterModule::drain(boma.object(), 2);
        VarModule::on_flag(boma.object(), vars::palutena::instance::FLUSH);
        PostureModule::set_lr(boma, PostureModule::lr(boma));
        PostureModule::update_rot_y_lr(boma);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_REFLECTIONBOARD, false, 0);
    }
}

unsafe extern "C" fn effect_specialno(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light_trace"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
        LAST_EFFECT_SET_COLOR(agent, 0.8, 0.2, 0.01);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.8, 0.2, 0.01);
        EFFECT(agent, Hash40::new("palutena_throw_twinkle"), Hash40::new("top"), 0.0, 16.0, -8.0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("palutena_mirror_break"), Hash40::new("top"), 0.0, 16.0, -8.0, 0, 0, 0, 0.225, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), -1, 21, 1, 0, 90, 0, 1, true, 0.7);
        LAST_EFFECT_SET_COLOR(agent, 0.8, 0.2, 0.01);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light_trace"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light2"), false, false);
    }
}

unsafe extern "C" fn sound_specialno(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_palutena_rnd_special_l02"));
    }
}

unsafe extern "C" fn expression_specialno(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 13.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_beams"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialno", game_specialno);
    agent.acmd("game_specialairno", game_specialno);
    agent.acmd("effect_specialno", effect_specialno);
    agent.acmd("effect_specialairno", effect_specialno);
    agent.acmd("sound_specialno", sound_specialno);
    agent.acmd("sound_specialairno", sound_specialno);
    agent.acmd("expression_specialno", expression_specialno);
    agent.acmd("expression_specialairno", expression_specialno);
}