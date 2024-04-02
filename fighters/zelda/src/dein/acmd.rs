use super::*;

unsafe extern "C" fn effect_tame(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	if is_excute(agent) {
		EFFECT_FOLLOW(agent, Hash40::new("zelda_din_bullet_end"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, true);
	}
	frame(lua_state, 20.0);
	if is_excute(agent) {
		let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
		if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_ZELDA {
			let zelda = utils::util::get_battle_object_from_id(owner_id);
			// Generate and store effects
			let flash_handle = EffectModule::req_follow(boma, Hash40::new("sys_flash"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 0.6, false, 0, 0, 0, 0, 0, false, false);
			let fire_handle = EffectModule::req_follow(boma, Hash40::new("zelda_appeal_s_fire"), Hash40::new("top"), &Vector3f::new(2.0, 0.0, 0.0), &Vector3f::zero(), 1.0, false, 0, 0, 0, 0, 0, false, false);
			VarModule::set_int64(zelda, vars::zelda::instance::DEIN_EFF_HANDLER_FLASH, flash_handle);
			VarModule::set_int64(zelda, vars::zelda::instance::DEIN_EFF_HANDLER_FIRE, fire_handle);
		}
	}
	for h in 21..=146 {
		if is_excute(agent) {
			let start_color = Vector3f { x: 1.0, y: 1.0, z: 1.0 };
			let end_color = Vector3f { x: 0.885, y: 0.051, z: 0.051 };
			// Smoothly interpolate from starting to ending color
			let blend_vector = Vector3f {
				x: start_color.x + ((end_color.x - start_color.x) * ((h as f32) / 146.0)),
				y: start_color.y + ((end_color.y - start_color.y) * ((h as f32) / 146.0)),
				z: start_color.z + ((end_color.z - start_color.z) * ((h as f32) / 146.0))
			};
			//println!("blend: {}, {}, {}", blend_vector.x, blend_vector.y, blend_vector.z);
			let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    		if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_ZELDA {
        		let zelda = utils::util::get_battle_object_from_id(owner_id);
				let flash_handle = VarModule::get_int64(zelda, vars::zelda::instance::DEIN_EFF_HANDLER_FLASH);
				let fire_handle = VarModule::get_int64(zelda, vars::zelda::instance::DEIN_EFF_HANDLER_FIRE);
				// Spawn (and store) new effects at certain frame intervals
				if [50, 80, 112, 146].contains(&h) {
					//println!("aha! h is {}", h);
					let tame_size = agent.get_float(*WEAPON_ZELDA_DEIN_STATUS_WORK_FLOAT_COUNT);
					let flash_size = if h == 50 { 1.0 + 0.003 * tame_size } else if h == 80 { 1.0 + 0.0135 * tame_size } else if h == 112 { 1.0 + 0.0165 * tame_size } else { 1.0 + 0.021 * tame_size };
					let fire_size = if h == 146 { 0.8 + 0.0037 * tame_size } else { 0.8 + 0.024 * tame_size };
					let flash_handle = EffectModule::req_follow(boma, Hash40::new("sys_flash"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), flash_size, false, 0, 0, 0, 0, 0, false, false);
					let fire_handle = EffectModule::req_follow(boma, Hash40::new("zelda_appeal_s_fire"), Hash40::new("top"), &Vector3f::new(2.0, 0.0, 0.0), &Vector3f::zero(), fire_size, false, 0, 0, 0, 0, 0, false, false);
					// Apply color blend
					EffectModule::set_rgb(boma, flash_handle as u32, blend_vector.x, blend_vector.y, blend_vector.z);
					EffectModule::set_rgb(boma, fire_handle as u32, blend_vector.x, blend_vector.y, blend_vector.z);
					VarModule::set_int64(zelda, vars::zelda::instance::DEIN_EFF_HANDLER_FLASH, flash_handle as u64);
					VarModule::set_int64(zelda, vars::zelda::instance::DEIN_EFF_HANDLER_FIRE, fire_handle as u64);
				}
				else {
					// Apply color blend
					EffectModule::set_rgb(boma, flash_handle as u32, blend_vector.x, blend_vector.y, blend_vector.z);
					EffectModule::set_rgb(boma, fire_handle as u32, blend_vector.x, blend_vector.y, blend_vector.z);
				}
			}
		}
		wait(lua_state, 1.0);
	}
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_tame", effect_tame);
}
