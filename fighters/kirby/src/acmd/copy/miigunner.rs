use super::*;

unsafe extern "C" fn effect_miigunnerspecialn1firemax(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	if is_excute(agent) {
		if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			EFFECT_FOLLOW(agent, Hash40::new("miigunner_sb_tama"), Hash40::new("armr"), 6.0, 0, 0, 0, 0, 0, 3.5, true);
			LAST_EFFECT_SET_RATE(agent, 2.0);
			LAST_EFFECT_SET_COLOR(agent, 0.15, 100.0, 10.0);
			EFFECT_FOLLOW(agent, Hash40::new("miigunner_sb_tama"), Hash40::new("armr"), 6.0, 0, 0, 90, 0, 0, 3.5, true);
			LAST_EFFECT_SET_RATE(agent, 2.0);
			LAST_EFFECT_SET_COLOR(agent, 0.15, 100.0, 3.0);
		}
	}
	frame(lua_state, 2.6);
	if is_excute(agent) {
		EFFECT_OFF_KIND(agent, Hash40::new("miigunner_sb_tama"), false, false);
		EFFECT_DETACH_KIND(agent, Hash40::new("miigunner_sb_tama"), -1);
	}
	frame(lua_state, 2.8);
	if is_excute(agent) {
		if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			EFFECT_FOLLOW(agent, Hash40::new("miigunner_atk_air_bullet"), Hash40::new("top"), 0, 6.3, 10.5, 0, 0, 0, 1, false);
			LAST_EFFECT_SET_COLOR(agent, 0.15, 5.0, 0.55);
			LAST_EFFECT_SET_RATE(agent, 0.85);
		}
	}
	frame(lua_state, 3.0);
	if is_excute(agent) {
		if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			EFFECT_FOLLOW(agent, Hash40::new("miigunner_cshot_shot"), Hash40::new("top"), 6, 3, 0, 0, 0, 0, 1, false);
			EFFECT_FOLLOW(agent, Hash40::new("miigunner_atk_laser"), Hash40::new("top"), 0, 6.3, 10.5, 0, 0, 0, 1, false);
			LAST_EFFECT_SET_SCALE_W(agent, 1.0, 0.7, 1.0);
			LAST_EFFECT_SET_RATE(agent, 0.8);
			EFFECT_FOLLOW(agent, Hash40::new("miigunner_atk_shot_s"), Hash40::new("armr"), 6.3, 0, 0, 0, 0, -90, 1, false);
			LAST_EFFECT_SET_COLOR(agent, 0.15, 5.0, 5.0);
			LAST_EFFECT_SET_RATE(agent, 0.5);
			if agent.is_situation(*SITUATION_KIND_GROUND) {
				LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
				FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.5, 0, 0, 0, 0, 0, 0, false);
			}
		}
		else {
			EFFECT(agent, Hash40::new("miigunner_cshot_shot"), Hash40::new("top"), 6, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
			if agent.is_situation(*SITUATION_KIND_GROUND) {
				LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
			}
		}
	}

}

unsafe extern "C" fn sound_miigunnerspecialn1firemax(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	if is_excute(agent) {
		PLAY_SEQUENCE(agent, Hash40::new("seq_miigunner_rnd_special_c1_n01"));
		if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			PLAY_SE(agent, Hash40::new("se_miigunner_final01"));
		}
	}
	frame(lua_state, 3.0);
	if is_excute(agent) {
		if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			STOP_SE(agent, Hash40::new("se_miigunner_final01"));
			PLAY_SE(agent, Hash40::new("se_miigunner_final04"));
		}
	}

}

pub fn install(agent: &mut Agent) {
    agent.acmd(
        "effect_miigunnerspecialn1firemax",
        effect_miigunnerspecialn1firemax,
    );
    agent.acmd(
        "effect_miigunnerspecialairn1firemax",
        effect_miigunnerspecialn1firemax,
    );
    agent.acmd(
        "sound_miigunnerspecialn1firemax",
        sound_miigunnerspecialn1firemax,
    );
    agent.acmd(
        "sound_miigunnerspecialairn1firemax",
        sound_miigunnerspecialn1firemax,
    );
}