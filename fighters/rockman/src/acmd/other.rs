
use super::*;

unsafe extern "C" fn game_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

unsafe extern "C" fn sound_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let dash_sfx_handle = SoundModule::play_se(agent.module_accessor, Hash40::new("se_rockman_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        PLAY_STEP_FLIPPABLE(agent, Hash40::new("se_rockman_step_right_m"), Hash40::new("se_rockman_step_left_m"));
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_STEP_FLIPPABLE(agent, Hash40::new("se_rockman_step_left_m"), Hash40::new("se_rockman_step_right_m"));
    }
}

unsafe extern "C" fn game_turndash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

/*
#[acmd_script( agent = "rockman_crashbomb", script = "game_regular" , category = ACMD_GAME , low_priority)]
unsafe fn rockman_crashbomb_stick_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, Hash40::new("body").hash as i64, Hash40::new("body_open").hash as i64);
        SEARCH(agent, 0, 0, Hash40::new("top"), 4.0, 0.0, 0.0, 0.0, *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, true);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(owner_boma, rockman::DETONATE_CRASHBOMB){
            ArticleModule::change_status(boma, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_CRASHBOMB, *, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    
}
*/

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

pub fn install(agent: &mut Agent) {
    agent.acmd("game_dash", game_dash);
    agent.acmd("sound_dash", sound_dash);

    agent.acmd("game_turndash", game_turndash);

    agent.acmd("game_escapeair", game_escapeair);

    agent.acmd("game_escapeairslide", game_escapeairslide);
}
