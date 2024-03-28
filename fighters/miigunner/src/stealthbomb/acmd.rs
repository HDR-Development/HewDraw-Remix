use super::*;

unsafe extern "C" fn effect_tame(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	if is_excute(agent) {
		EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.15, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_RATE(agent, 0.75);
	}
	frame(lua_state, 50.0);
	if is_excute(agent) {
		EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.20, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_COLOR(agent, 0.5, 0.5, 5.0);
	}
    frame(lua_state, 100.0);
	if is_excute(agent) {
		let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_MIIGUNNER {
            let gunner = utils::util::get_battle_object_from_id(owner_id);
            let flash_handle = EffectModule::req_follow(boma, Hash40::new("sys_smash_flash"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 0.5, false, 0, 0, 0, 0 ,0, false, false);
            LAST_EFFECT_SET_COLOR(agent, 0.15, 0.15, 10.0);
            EffectModule::set_rgb(boma, flash_handle as u32, 0.15, 0.15, 10.0);
            EffectModule::set_rate(boma, flash_handle as u32, 0.3);
            VarModule::set_int64(gunner, vars::miigunner::instance::STEALTHBOMB_EFF_HANDLER, flash_handle);
        }
	}
    for h in 101..=120 {
		if is_excute(agent) {
			let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    		if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_MIIGUNNER {
        		let gunner = utils::util::get_battle_object_from_id(owner_id);
				let flash_handle = VarModule::get_int64(gunner, vars::miigunner::instance::STEALTHBOMB_EFF_HANDLER);
				let start_color = Vector3f { x: 0.15, y: 0.15, z: 10.0 };
                let end_color = Vector3f { x: 10.0, y: 0.15, z: 0.15 };
                // Smoothly interpolate from starting to ending color
                let blend_vector = Vector3f {
                    x: start_color.x + ((end_color.x - start_color.x) * (((h as f32) - 100.0) / 20.0)),
                    y: start_color.y + ((end_color.y - start_color.y) * (((h as f32) - 100.0) / 20.0)),
                    z: start_color.z + ((end_color.z - start_color.z) * (((h as f32) - 100.0) / 20.0))
                };
                // Apply color blend
                EffectModule::set_rgb(boma, flash_handle as u32, blend_vector.x, blend_vector.y, blend_vector.z);
			}
		}
		wait(lua_state, 1.0);
	}
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_tame", effect_tame);
}