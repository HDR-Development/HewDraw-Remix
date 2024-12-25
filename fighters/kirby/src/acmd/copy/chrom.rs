use super::*;

unsafe extern "C" fn effect_chromspecialnend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_volcano_hold_1"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_volcano_hold_2"), false, false);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_sword_purple"), Hash40::new("havel"), 0, 1.25, 0, -90, 90, 0, 1, true);
    }
    //frame(lua_state, 3.0);
    //if is_excute(agent) {
        //AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_chrom_sword1"), Hash40::new("tex_chrom_sword2"), 8, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, false, Hash40::new("chrom_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    //}
    frame(lua_state, 12.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 17, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 6);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_sword_purple"), false, false);
    }
}

unsafe extern "C" fn expression_chromspecialnend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        //AREA_WIND_2ND_arg10(agent, 0, 2, 110, 300, 0.6, 0, 12, 30, 30, 40);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 5);
        RUMBLE_HIT(agent, Hash40::new("rbkind_explosion"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, 0);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        //QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
    }
}

unsafe extern "C" fn expression_chromspecialairnend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        //AREA_WIND_2ND_arg10(agent, 0, 2, 110, 300, 0.6, 0, 12, 30, 30, 40);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 5);
        RUMBLE_HIT(agent, Hash40::new("rbkind_explosion"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, 0);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        //QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_chromspecialnend", effect_chromspecialnend, Priority::Low);
    agent.acmd("effect_chromspecialairnend", effect_chromspecialnend, Priority::Low);
    agent.acmd("expression_chromspecialnend", expression_chromspecialnend, Priority::Low);
    agent.acmd("expression_chromspecialairnend", expression_chromspecialairnend, Priority::Low);
}