use super::*;

unsafe extern "C" fn game_specialnr(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    let powered = VarModule::is_flag(agent.object(), vars::palutena::status::SPECIAL_N_PRIMARY_POWERED) ||
                        VarModule::get_int(agent.object(), vars::palutena::instance::POWER_BOARD_SLOT_2) == 1;
    //println!("POWERED (game): {}", powered);
    let power1 = if powered {18.0} else {14.0};
    let power2 = if powered {12.0} else {10.0};
    let sound_lvl = if powered {*ATTACK_SOUND_LEVEL_L} else {*ATTACK_SOUND_LEVEL_M};
    let size1 = if powered {6.6} else {5.6};
    let size2 = if powered {10.6} else {8.6};
    let kbg1 = if powered {85} else {95};
    let kbg2 = if powered {87} else {97};
    FT_MOTION_RATE_RANGE(agent, 1.0, 18.0, 11.0);
    frame(lua_state, 18.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        VarModule::on_flag(boma.object(), vars::palutena::instance::SPECIAL_N_FLUSH_BOARD);
        ATTACK(agent, 0, 0, Hash40::new("top"), power1, 361, kbg1, 0, 40, size1, 1.0, 11.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), sound_lvl, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 1, 0, Hash40::new("top"), power2, 361, kbg2, 0, 40, size2, 1.0, 11.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), sound_lvl, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_MAGIC);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialnr(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let power = VarModule::is_flag(agent.object(), vars::palutena::status::SPECIAL_N_PRIMARY_POWERED);
    //println!("POWERED (effects): {}", power);
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light_trace_grey"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0125, 0.025);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2_grey"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0125, 0.025);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_backlight_grey"), Hash40::new("top"), 1, 21, 2.5, 0, -50, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0125, 0.025);
    }
    frame(lua_state, 18.0);
    if power {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_bomb_d"), Hash40::new("top"), 0.0, 12.0, 10.0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0.0, 12.0, 10.0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light_trace_grey"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light2_grey"), false, false);
    }
}

unsafe extern "C" fn sound_specialnr(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let power = VarModule::is_flag(agent.object(), vars::palutena::status::SPECIAL_N_PRIMARY_POWERED);
    let sound_lvl = if power {Hash40::new("se_common_bomb_l")} else {Hash40::new("se_common_bomb_s")};
    frame(lua_state, 14.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_palutena_rnd_smash_s"));
        if power {
            PLAY_SE(agent, Hash40::new("se_palutena_smash_s01"));
        }
        PLAY_SE(agent, sound_lvl);
    }
}

unsafe extern "C" fn expression_specialnr(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 4);
        AREA_WIND_2ND_arg10(agent, 0, 1, 80, 300, 0.8, 0, 12, 24, 24, 40);
    }
    frame(lua_state, 11.0);
    app::sv_animcmd::execute(lua_state, 11.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        AREA_WIND_2ND_arg10(agent, 0, 2, 30, 300, 0.8, 25, 12, 50, 24, 80);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        RUMBLE_HIT(agent, Hash40::new("rbkind_explosion"), 0);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
    frame(lua_state, 65.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnr", game_specialnr, Priority::Low);
    agent.acmd("game_specialairnr", game_specialnr, Priority::Low);
    agent.acmd("effect_specialnr", effect_specialnr, Priority::Low);
    agent.acmd("effect_specialairnr", effect_specialnr, Priority::Low);
    agent.acmd("sound_specialnr", sound_specialnr, Priority::Low);
    agent.acmd("sound_specialairnr", sound_specialnr, Priority::Low);
    agent.acmd("expression_specialnr", expression_specialnr, Priority::Low);
    agent.acmd("expression_specialairnr", expression_specialnr, Priority::Low);
}