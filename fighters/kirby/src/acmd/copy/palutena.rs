use super::*;

unsafe extern "C" fn effect_palutenaspecialn(agent: &mut L2CAgentBase) {
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
    frame(lua_state, 14.0);
    if VarModule::get_int(boma.object(), vars::palutena::instance::CYAN_ENERGY) >= 3 {
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_PALUTENA_STATUS_SPECIAL_N_FLAG_SIGHT_EFFECT_ON);
        }
        frame(lua_state, 15.0);
        if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_STATUS_SPECIAL_N_FLAG_TARGET_EXIST) {
            if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
                if is_excute(agent) {
                    EFFECT_FOLLOW(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 0, 11.5, -1.5, 0, 65, 0, 1.1, true);
                }
                else {
                if is_excute(agent) {
                    EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light_trace"), Hash40::new("havel"), 0, 6.5, 0, 0, 0, 0, 0.9, true);
                    EffectModule::enable_sync_init_pos_last(boma);
                    EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2"), Hash40::new("havel"), 0, 6.5, 0, 0, 0, 0, 0.9, true);
                }
                frame(lua_state, 20.0);
                if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
                    if is_excute(agent) {
                        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 0, 11, -1.5, 0, 65, 0, 1.1, true, 0.8);
                    }
                    else {
                    if is_excute(agent) {
                        EFFECT_FOLLOW(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 0, 11.5, -1.5, 0, -65, 0, 1.1, true);
                    }
                    if is_excute(agent) {
                        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light_trace"), Hash40::new("havel"), 0, 6.5, 0, 0, 0, 0, 0.95, true);
                        EffectModule::enable_sync_init_pos_last(boma);
                        EFFECT_FOLLOW(agent, Hash40::new("palutena_lockon"), Hash40::new("havel"), 0, -3.6, 0, 0, 0, 0, 1.2, true);
                    }
                    else {
                    if is_excute(agent) {
                        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 0, 11, -1.5, 0, -65, 0, 1.1, true, 0.8);
                    }
                }
            }
        }
    }
    }
    }
    frame(lua_state, 57.0);
    if is_excute(agent) {
    EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light2"), false, false);
    EFFECT_OFF_KIND(agent, Hash40::new("palutena_lockon"), false, false);
    }
    frame(lua_state, 59.0);
    if is_excute(agent) {
    EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light_trace"), false, false);
    }
    }
}

unsafe extern "C" fn sound_palutenaspecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_palutena_special_n01"));
    }
    frame(lua_state, 14.0);
    if VarModule::get_int(boma.object(), vars::palutena::instance::CYAN_ENERGY) >= 3 {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("vc_kirby_copy_palutena_03"));
        }
    }
    wait(lua_state, 22.0);
    if is_excute(agent) {
        sound!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
    }
}

unsafe extern "C" fn expression_palutenaspecialn(agent: &mut L2CAgentBase) {
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

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_palutenaspecialn", effect_palutenaspecialn, Priority::Low);
    agent.acmd("effect_palutenaspecialairn", effect_palutenaspecialn, Priority::Low);
    agent.acmd("sound_palutenaspecialn", sound_palutenaspecialn, Priority::Low);
    agent.acmd("sound_palutenaspecialairn", sound_palutenaspecialn, Priority::Low);
    agent.acmd("expression_palutenaspecialn", expression_palutenaspecialn, Priority::Low);
    agent.acmd(
        "expression_palutenaspecialairn",
        expression_palutenaspecialn,
        Priority::Low
    );
}