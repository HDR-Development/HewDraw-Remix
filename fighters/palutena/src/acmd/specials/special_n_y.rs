use super::*;

unsafe extern "C" fn game_specialny(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let hitlag = if VarModule::is_flag(agent.object(), vars::palutena::instance::POWERED) {0.5} else {0.75};
    let paralyze = if VarModule::is_flag(agent.object(), vars::palutena::instance::POWERED) {0.6} else {0.3};
    let power = if VarModule::is_flag(agent.object(), vars::palutena::instance::POWERED) {2} else {4};
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
    FT_MOTION_RATE_RANGE(agent, 1.0, 20.0, 16.0);
    frame(lua_state, 20.0);
    if is_excute(agent) {
        if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
            WorkModule::set_float(boma, PostureModule::pos_x(boma) - 100.0, *FIGHTER_PALUTENA_STATUS_SPECIAL_N_WORK_FLOAT_TARGET_POS_X);
        }
        else {
            WorkModule::set_float(boma, PostureModule::pos_x(boma) + 100.0, *FIGHTER_PALUTENA_STATUS_SPECIAL_N_WORK_FLOAT_TARGET_POS_X);
        }
        WorkModule::set_float(boma, PostureModule::pos_y(boma) + 5.0, *FIGHTER_PALUTENA_STATUS_SPECIAL_N_WORK_FLOAT_TARGET_POS_Y);
        ArticleModule::generate_article(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_AUTOAIMBULLET, false, -1);
    }
}

unsafe extern "C" fn effect_specialny(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), -0.2, 22, -1, 10, 90, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.85, 0.40, 0.001);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light_trace"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.85, 0.40, 0.001);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.85, 0.40, 0.001);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light_trace"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light2"), false, false);
    }
}

unsafe extern "C" fn sound_specialny(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let power = VarModule::is_flag(agent.object(), vars::palutena::instance::POWERED);
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
    agent.acmd("game_specialny", game_specialny);
    agent.acmd("game_specialairny", game_specialny);
    agent.acmd("effect_specialny", effect_specialny);
    agent.acmd("effect_specialairny", effect_specialny);
    agent.acmd("sound_specialny", sound_specialny);
    agent.acmd("sound_specialairny", sound_specialny);
    agent.acmd("expression_specialny", expression_specialny);
    agent.acmd("expression_specialairny", expression_specialny);
}