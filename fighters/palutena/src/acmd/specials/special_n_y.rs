use super::*;

unsafe extern "C" fn game_specialny(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 20.0, 16.0);
    frame(lua_state, 20.0);
    FT_MOTION_RATE_RANGE(agent, 20.0, 54.0, 24.0); // 37 FAF -> 41
    if is_excute(agent) {
        VarModule::on_flag(boma.object(), vars::palutena::instance::SPECIAL_N_FLUSH_BOARD);
        agent.set_float(PostureModule::pos_x(boma) + (100.0 * boma.lr()), *FIGHTER_PALUTENA_STATUS_SPECIAL_N_WORK_FLOAT_TARGET_POS_X);
        agent.set_float(PostureModule::pos_y(boma) + 5.0, *FIGHTER_PALUTENA_STATUS_SPECIAL_N_WORK_FLOAT_TARGET_POS_Y);
        ArticleModule::generate_article(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_AUTOAIMBULLET, false, -1);
    }
    frame(lua_state, 54.0); // cancel frame
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specialny(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_backlight_grey"), Hash40::new("top"), -0.2, 22, -1, 10, 90, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.87, 0.0125);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light_trace_grey"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.87, 0.0125);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2_grey"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.87, 0.0125);
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light_trace_grey"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light2_grey"), false, false);
    }
}

unsafe extern "C" fn sound_specialny(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let power = VarModule::is_flag(agent.object(), vars::palutena::status::SPECIAL_N_PRIMARY_POWERED);
    let sound_lvl = if power {Hash40::new("se_common_electric_hit_l")} else {Hash40::new("se_common_electric_hit_m")};
    frame(lua_state, 11.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_palutena_special_n01"));
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_electric_hit_s"));
    }
    wait(lua_state, 12.0);
    if is_excute(agent) {
        PLAY_SE(agent, sound_lvl);
    }
    wait(lua_state, 29.0);
    if is_excute(agent) {
        sound!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
    }
}

unsafe extern "C" fn expression_specialny(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_beams"), 0);
    }
    wait(lua_state, 12.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_beamm"), 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialny", game_specialny, Priority::Low);
    agent.acmd("game_specialairny", game_specialny, Priority::Low);
    agent.acmd("effect_specialny", effect_specialny, Priority::Low);
    agent.acmd("effect_specialairny", effect_specialny, Priority::Low);
    agent.acmd("sound_specialny", sound_specialny, Priority::Low);
    agent.acmd("sound_specialairny", sound_specialny, Priority::Low);
    agent.acmd("expression_specialny", expression_specialny, Priority::Low);
    agent.acmd("expression_specialairny", expression_specialny, Priority::Low);
}