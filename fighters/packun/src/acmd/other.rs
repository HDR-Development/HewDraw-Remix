use super::*;

unsafe extern "C" fn game_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 15.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

unsafe extern "C" fn sound_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let dash_sfx_handle = SoundModule::play_se(boma, Hash40::new("se_packun_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_STEP_FLIPPABLE(agent, Hash40::new("se_packun_step_right_m"), Hash40::new("se_packun_step_left_m"));
    }
    wait(lua_state, 9.0);
    if is_excute(agent) {
        PLAY_STEP_FLIPPABLE(agent, Hash40::new("se_packun_step_left_m"), Hash40::new("se_packun_step_right_m"));
    }
}

unsafe extern "C" fn game_turndash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

unsafe extern "C" fn game_escapeair(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 29.0);
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }

}

unsafe extern "C" fn game_escapeairslide(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 29.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }

}

unsafe extern "C" fn game_appealhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
}

unsafe extern "C" fn sound_appealhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if !VarModule::is_flag(agent.object(), vars::packun::status::APPEAL_CLOUD_COVER) {
        frame(lua_state, 19.0);
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_packun_appear01"));
        }
    }
}

unsafe extern "C" fn game_appeals2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ModelModule::set_mesh_visibility(agent.boma(), Hash40::new("foot"), true);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.object(), vars::packun::status::STANCE_INIT);
    }
    frame(lua_state, 107.0);
    if is_excute(agent) {
        ModelModule::set_mesh_visibility(agent.boma(), Hash40::new("foot"), false);
    }
}

unsafe extern "C" fn effect_appeals2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if !VarModule::is_flag(agent.object(), vars::packun::status::APPEAL_CLOUD_COVER) {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("packun_appeal_left"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, false);
        }
    }
}

unsafe extern "C" fn sound_appeals2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if !VarModule::is_flag(agent.object(), vars::packun::status::APPEAL_CLOUD_COVER) {
        frame(lua_state, 29.0);
        if is_excute(agent) {
            PLAY_STEP_FLIPPABLE(agent, Hash40::new("se_packun_step_right_m"), Hash40::new("se_packun_step_left_m"));
        }
        frame(lua_state, 54.0);
        if is_excute(agent) {
            PLAY_STEP_FLIPPABLE(agent, Hash40::new("se_packun_step_left_m"), Hash40::new("se_packun_step_right_m"));
        }
        frame(lua_state, 78.0);
        if is_excute(agent) {
            PLAY_STEP_FLIPPABLE(agent, Hash40::new("se_packun_step_right_m"), Hash40::new("se_packun_step_left_m"));
        }
    }
}

unsafe extern "C" fn expression_appeals2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 54.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 78.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_appeals(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if boma.is_button_on(Buttons::AppealSL) {
        if is_excute(agent) {
            MotionModule::change_motion(boma, Hash40::new("appeal_s_2"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.object(), vars::packun::status::STANCE_INIT);
    }
}

unsafe extern "C" fn sound_appeals(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if !VarModule::is_flag(agent.object(), vars::packun::status::APPEAL_CLOUD_COVER) {
        frame(lua_state, 1.0);
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_packun_appeal_s01"));
        }
        frame(lua_state, 14.0);
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_packun_appeal_s02"));
        }
        frame(lua_state, 30.0);
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_packun_appeal_s03"));
        }
        frame(lua_state, 46.0);
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_packun_appeal_s04"));
        }
    }
}

unsafe extern "C" fn game_appeallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.object(), vars::packun::status::STANCE_INIT);
    }
}

unsafe extern "C" fn effect_appeallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if !VarModule::is_flag(agent.object(), vars::packun::status::APPEAL_CLOUD_COVER) {
        frame(lua_state, 5.0);
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn sound_appeallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if !VarModule::is_flag(agent.object(), vars::packun::status::APPEAL_CLOUD_COVER) {
        frame(lua_state, 1.0);
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_packun_appeal_l01"));
        }
        frame(lua_state, 21.0);
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_packun_appeal_l02"));
        }
    }
}

unsafe extern "C" fn game_passivestand(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 24.0, 20.0);
    frame(lua_state, 24.0);
    FT_MOTION_RATE_RANGE(agent, 24.0, 41.0, 20.0);
    frame(lua_state, 41.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_cliffjump2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        PostureModule::add_pos(boma, &Vector3f::new(0.0, -2.275, 0.0));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_cliffescape", acmd_stub, Priority::Low);

    agent.acmd("game_dash", game_dash, Priority::Low);
    agent.acmd("sound_dash", sound_dash, Priority::Low);
    agent.acmd("game_turndash", game_turndash, Priority::Low);

    agent.acmd("game_escapeair", game_escapeair, Priority::Low);
    agent.acmd("game_escapeairslide", game_escapeairslide, Priority::Low);

    agent.acmd("game_appealhil", game_appealhi, Priority::Low);
    agent.acmd("game_appealhir", game_appealhi, Priority::Low);
    agent.acmd("effect_appealhil", acmd_stub, Priority::Low);
    agent.acmd("effect_appealhir", acmd_stub, Priority::Low);
    agent.acmd("sound_appealhil", sound_appealhi, Priority::Low);
    agent.acmd("sound_appealhir", sound_appealhi, Priority::Low);

    agent.acmd("game_appeals2", game_appeals2, Priority::Low);
    agent.acmd("effect_appeals2", effect_appeals2, Priority::Low);
    agent.acmd("sound_appeals2", sound_appeals2, Priority::Low);
    agent.acmd("expression_appeals2", expression_appeals2, Priority::Low);

    agent.acmd("game_appealsl", game_appeals, Priority::Low);
    agent.acmd("game_appealsr", game_appeals, Priority::Low);
    agent.acmd("effect_appealsl", acmd_stub, Priority::Low);
    agent.acmd("effect_appealsr", acmd_stub, Priority::Low);
    agent.acmd("sound_appealsl", sound_appeals, Priority::Low);
    agent.acmd("sound_appealsr", sound_appeals, Priority::Low);

    agent.acmd("game_appeallwl", game_appeallw, Priority::Low);
    agent.acmd("game_appeallwr", game_appeallw, Priority::Low);
    agent.acmd("effect_appeallwl", effect_appeallw, Priority::Low);
    agent.acmd("effect_appeallwr", effect_appeallw, Priority::Low);
    agent.acmd("sound_appeallwl", sound_appeallw, Priority::Low);
    agent.acmd("sound_appeallwr", sound_appeallw, Priority::Low);

    agent.acmd("game_passivestandf", game_passivestand, Priority::Low);
    agent.acmd("game_passivestandb", game_passivestand, Priority::Low);

    agent.acmd("game_cliffjump2", game_cliffjump2, Priority::Low);
}