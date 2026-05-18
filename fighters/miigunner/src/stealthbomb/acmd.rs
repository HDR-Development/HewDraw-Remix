use super::*;

unsafe extern "C" fn effect_tame(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	if is_excute(agent) {
		EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.15, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_RATE(agent, 0.75);
	}
    frame(lua_state, 40.0);
	if is_excute(agent) {
		let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_MIIGUNNER {
            let gunner = utils::util::get_battle_object_from_id(owner_id);
            let flash_handle = EffectModule::req_follow(boma, Hash40::new("sys_smash_flash"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 0.2, false, 0, 0, 0, 0 ,0, false, false);
            EffectModule::set_rgb(boma, flash_handle as u32, 0.5, 0.5, 5.0);
            EffectModule::set_rate(boma, flash_handle as u32, 0.4);
            VarModule::set_int64(gunner, vars::miigunner::instance::SPECIAL_S2_STEALTHBOMB_EFFECT_HANDLE, flash_handle);
        }
	}
    frame(lua_state, 50.0);
    for h in 1..=20 {
		if is_excute(agent) {
			let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    		if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_MIIGUNNER {
        		let gunner = utils::util::get_battle_object_from_id(owner_id);
				let flash_handle = VarModule::get_int64(gunner, vars::miigunner::instance::SPECIAL_S2_STEALTHBOMB_EFFECT_HANDLE);
				let start_color = Vector3f { x: 0.5, y: 0.5, z: 5.0 };
                let end_color = Vector3f { x: 10.0, y: 0.15, z: 0.15 };
                // Smoothly interpolate from starting to ending color
                let blend_vector = Vector3f {
                    x: start_color.x + ((end_color.x - start_color.x) * ((h as f32) / 20.0)),
                    y: start_color.y + ((end_color.y - start_color.y) * ((h as f32) / 20.0)),
                    z: start_color.z + ((end_color.z - start_color.z) * ((h as f32) / 20.0))
                };
                // Apply color blend
                EffectModule::set_rgb(boma, flash_handle as u32, blend_vector.x, blend_vector.y, blend_vector.z);
                if h == 5 {
                    EFFECT_FOLLOW(agent, Hash40::new("miigunner_sb_tama"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.75, true);
                    LAST_EFFECT_SET_RATE(agent, 0.05);
                    LAST_EFFECT_SET_COLOR(agent, 1.5, 0.75, 0.75);
                }
			}
		}
		wait(lua_state, 1.0);
	}
}

unsafe extern "C" fn game_turn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 366, 0, 0, 0, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BOMB);
    }
}

unsafe extern "C" fn effect_turn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("miigunner_sb_tama"), false, false);
        EFFECT_FOLLOW(agent, Hash40::new("miigunner_sb_tama"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, true);
        LAST_EFFECT_SET_RATE(agent, 0.5);
        LAST_EFFECT_SET_COLOR(agent, 1.5, 0.75, 0.75);
        LAST_EFFECT_SET_ALPHA(agent, 2.0);
        EFFECT_FOLLOW(agent, Hash40::new("miigunner_sb_tama"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.75, true);
        LAST_EFFECT_SET_RATE(agent, 0.25);
        LAST_EFFECT_SET_COLOR(agent, 1.5, 0.75, 0.75);
    }
}

unsafe extern "C" fn sound_turn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        let handle = SoundModule::play_status_se(boma, Hash40::new("se_miigunner_special_c2_s01"), false, false, false);
        SoundModule::set_se_vol(boma, handle as i32, 0.5, 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_tame", effect_tame, Priority::Low);
    agent.acmd("sound_tame", acmd_stub, Priority::Low);

    agent.acmd("game_turn", game_turn, Priority::Low);
    agent.acmd("effect_turn", effect_turn, Priority::Low);
    agent.acmd("sound_turn", sound_turn, Priority::Low);
}