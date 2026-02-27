use super::*;

unsafe extern "C" fn sound_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let dash_sfx_handle = SoundModule::play_se(boma, Hash40::new("se_samus_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 9.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_samus_step_right_m"));
    }
    wait(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_samus_step_left_m"))
    }
}

unsafe extern "C" fn game_turndash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
		WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

unsafe extern "C" fn game_escapeair(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let escape_air_cancel_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_cancel_frame"));
    frame(lua_state, 29.0);
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    frame(lua_state, escape_air_cancel_frame);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn game_escapeairslide(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 29.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn game_squat(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
    }
}

unsafe extern "C" fn expression_squat(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        ItemModule::set_attach_item_visibility(boma, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 10, true);
    }
    for i in 1..i32::MAX{
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(lua_state, 30.0);
    }
}

unsafe extern "C" fn game_jumpaerialfront(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    if is_excute(agent) {
        GroundModule::select_cliff_hangdata(boma, 3);
    }
}

unsafe extern "C" fn game_jumpaerialback(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    if is_excute(agent) {
        GroundModule::select_cliff_hangdata(boma, 3);
    }
}

unsafe extern "C" fn game_cliffjump2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        PostureModule::add_pos(boma, &Vector3f::new(0.0, -1.0, 0.0));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_cliffescape", acmd_stub, Priority::Low);

    agent.acmd("sound_dash", sound_dash, Priority::Low);
    agent.acmd("game_turndash", game_turndash, Priority::Low);
    
    agent.acmd("game_escapeair", game_escapeair, Priority::Low);
    agent.acmd("game_escapeairslide", game_escapeairslide, Priority::Low);

    agent.acmd("game_squatn", game_squat, Priority::Low);

    agent.acmd("game_squatentry", game_squat, Priority::Low);
    agent.acmd("effect_squatentry", acmd_stub, Priority::Low);
    agent.acmd("sound_squatentry", acmd_stub, Priority::Low);
    agent.acmd("expression_squatentry", expression_squat, Priority::Low);

    agent.acmd("game_squatf", game_squat, Priority::Low);
    agent.acmd("effect_squatf", acmd_stub, Priority::Low);
    agent.acmd("sound_squatf", acmd_stub, Priority::Low);
    agent.acmd("expression_squatf", expression_squat, Priority::Low);

    agent.acmd("game_squatb", game_squat, Priority::Low);
    agent.acmd("effect_squatb", acmd_stub, Priority::Low);
    agent.acmd("sound_squatb", acmd_stub, Priority::Low);
    agent.acmd("expression_squatb", expression_squat, Priority::Low);

    agent.acmd("game_jumpaerialfront", game_jumpaerialfront, Priority::Low);
    agent.acmd("game_jumpaerialback", game_jumpaerialback, Priority::Low);

    agent.acmd("game_cliffjump2", game_cliffjump2, Priority::Low);
}