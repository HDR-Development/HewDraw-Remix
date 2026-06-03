use super::*;

unsafe extern "C" fn game_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE_RANGE(agent, 1.0, 11.0, 12.0);
	frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

unsafe extern "C" fn sound_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let dash_sfx_handle = SoundModule::play_se(boma, Hash40::new("se_pickel_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
}

unsafe extern "C" fn game_turndash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.2);
		WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        //WorkModule::on_flag(boma, 60192);
		WorkModule::on_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_BLEND_TURN);
    }
    frame(lua_state, 13.0); // Effectively F15
    if is_excute(agent) {
		FT_MOTION_RATE(agent, 1.0);
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

unsafe extern "C" fn effect_run(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    for _ in 0..2 { // flashes signify dash special availability
        if is_excute(agent) {
            FLASH(agent, 1, 1, 0.753, 0.627);
            FLASH_FRM(agent, 5, 0.502, 0, 0, 0);
        }
        wait(lua_state, 4.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 4.0);
    }
    loop {
        frame(lua_state, 14.0);
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(lua_state, 29.0);
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        agent.clear_lua_stack();
        sv_animcmd::wait_loop_sync_mot(lua_state);
        agent.pop_lua_stack(1);
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

unsafe extern "C" fn game_appeals(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(lua_state, 90.0);
    if is_excute(agent){
        if DamageModule::damage(boma, 0) > 2.0 {
            DamageModule::add_damage(boma, -2.0, 0);
        }
    }
}

unsafe extern "C" fn game_guardon(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        ArticleModule::generate_article(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_STUFF, false, -1);
    }
}

unsafe extern "C" fn game_guarddamage(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if !ArticleModule::is_exist(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_STUFF) {
            ArticleModule::generate_article(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_STUFF, false, -1);
        }
    }
}

unsafe extern "C" fn sound_landingheavy(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if WorkModule::is_flag(boma, *FIGHTER_PICKEL_STATUS_LANDING_FLAG_HIGH_PLACE) {
        frame(lua_state, 3.0);
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_pickel_landing_high_place"));
            DamageModule::add_damage(boma, 0.1, 0);
        }
    } else {
        frame(lua_state, 0.0);
        if is_excute(agent) {
            PLAY_LANDING_SE(agent, Hash40::new("se_pickel_landing02"));
        }
    }
}

unsafe extern "C" fn game_passive(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_MELT, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_MELT, Hash40::new("passive"), false, 0.0);
        let melt = ArticleModule::get_article(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_MELT);
        let melt_id = smash::app::lua_bind::Article::get_battle_object_id(melt) as u32;
        let melt_boma = sv_battle_object::module_accessor(melt_id);
        let lr = PostureModule::lr(boma);
        PostureModule::add_pos_2d(melt_boma, &Vector2f {x: (-10.0 * lr), y: 0.0});
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_cliffescape", acmd_stub, Priority::Low);

    agent.acmd("game_dash", game_dash, Priority::Low);
    agent.acmd("sound_dash", sound_dash, Priority::Low);
    agent.acmd("game_turndash", game_turndash, Priority::Low);

    agent.acmd("effect_run", effect_run, Priority::Low);

    agent.acmd("game_escapeair", game_escapeair, Priority::Low);
    agent.acmd("game_escapeairslide", game_escapeairslide, Priority::Low);

    agent.acmd("game_appealsl", game_appeals, Priority::Low);
    agent.acmd("game_appealsr", game_appeals, Priority::Low);

    agent.acmd("game_guardon", game_guardon, Priority::Low);

    agent.acmd("game_guarddamage", game_guarddamage, Priority::Low);

    agent.acmd("sound_landingheavy", sound_landingheavy, Priority::Low);
    agent.acmd("sound_landingfallspecial", sound_landingheavy, Priority::Low);

    agent.acmd("game_passive", game_passive, Priority::Low);
}
