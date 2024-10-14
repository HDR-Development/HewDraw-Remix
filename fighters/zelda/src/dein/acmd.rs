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
			let tame_size = agent.get_float(*WEAPON_ZELDA_DEIN_STATUS_WORK_FLOAT_COUNT);
			// Generate and store effects
			let flash_handle = EffectModule::req_follow(boma, Hash40::new("sys_flash"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 0.35 + 0.005 * tame_size, false, 0, 0, 0, 0, 0, false, false);
			let fire_handle = EffectModule::req_follow(boma, Hash40::new("zelda_appeal_s_fire"), Hash40::new("top"), &Vector3f::new(2.0, 0.0, 0.0), &Vector3f::zero(), 0.8 + 0.024 * tame_size, false, 0, 0, 0, 0, 0, false, false);
			VarModule::set_int64(zelda, vars::zelda::instance::SPECIAL_S_DEIN_FLASH_EFFECT_HANDLE, flash_handle);
			VarModule::set_int64(zelda, vars::zelda::instance::SPECIAL_S_DEIN_FIRE_EFFECT_HANDLE, fire_handle);
		}
	}
	for h in 21..=146 {
		if is_excute(agent) {
			let start_color = Vector3f { x: 0.85, y: 1.05, z: 1.15 };
			let start_color_flame = Vector3f { x: 1.2, y: 4.5, z: 6.0 };
			let end_color = Vector3f { x: 0.9, y: 0.044, z: 0.005 };
			// Smoothly interpolate from starting to ending color
			let flame_blend_vector = Vector3f {
				x: end_color.x + (start_color_flame.x * ((h as f32) / 146.0)),
				y: end_color.y + (start_color_flame.y * ((h as f32) / 146.0)),
				z: end_color.z + (start_color_flame.z * ((h as f32) / 146.0))
			};
			let blend_vector = Vector3f {
				x: start_color.x + ((end_color.x - start_color.x) * ((h as f32) / 146.0)),
				y: start_color.y + ((end_color.y - start_color.y) * ((h as f32) / 146.0)),
				z: start_color.z + ((end_color.z - start_color.z) * ((h as f32) / 146.0))
			};
			//println!("blend: {}, {}, {}", blend_vector.x, blend_vector.y, blend_vector.z);
			let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    		if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_ZELDA {
        		let zelda = utils::util::get_battle_object_from_id(owner_id);
				let flash_handle = VarModule::get_int64(zelda, vars::zelda::instance::SPECIAL_S_DEIN_FLASH_EFFECT_HANDLE);
				let fire_handle = VarModule::get_int64(zelda, vars::zelda::instance::SPECIAL_S_DEIN_FIRE_EFFECT_HANDLE);
				// Spawn (and store) new effects at certain frame intervals
				if [50, 80, 112, 146].contains(&h) {
					//println!("aha! h is {}", h);
					let tame_size = agent.get_float(*WEAPON_ZELDA_DEIN_STATUS_WORK_FLOAT_COUNT);
					let flash_size = if h == 50 { 0.45 + 0.01 * tame_size * 0.95} else if h == 80 { 0.6 + 0.0145 * tame_size * 0.95} else if h == 112 { 0.75 + 0.019 * tame_size * 0.95} else { 0.9 + 0.023 * tame_size * 0.95};
					let fire_size = if h == 146 { 0.8 + 0.037 * tame_size } else { 0.8 + 0.024 * tame_size };
					let flash_handle = EffectModule::req_follow(boma, Hash40::new("sys_flash"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), flash_size, false, 0, 0, 0, 0, 0, false, false);
					let fire_handle = EffectModule::req_follow(boma, Hash40::new("zelda_appeal_s_fire"), Hash40::new("top"), &Vector3f::new(2.0, 0.0, 0.0), &Vector3f::zero(), fire_size, false, 0, 0, 0, 0, 0, false, false);
					// Apply color blend
					EffectModule::set_rgb(boma, flash_handle as u32, blend_vector.x, blend_vector.y, blend_vector.z);
					EffectModule::set_rgb(boma, fire_handle as u32, flame_blend_vector.x, flame_blend_vector.y, flame_blend_vector.z);
					VarModule::set_int64(zelda, vars::zelda::instance::SPECIAL_S_DEIN_FLASH_EFFECT_HANDLE, flash_handle as u64);
					VarModule::set_int64(zelda, vars::zelda::instance::SPECIAL_S_DEIN_FIRE_EFFECT_HANDLE, fire_handle as u64);
				}
				else {
					// Apply color blend
					EffectModule::set_rgb(boma, flash_handle as u32, blend_vector.x, blend_vector.y, blend_vector.z);
					EffectModule::set_rgb(boma, fire_handle as u32, flame_blend_vector.x, flame_blend_vector.y, flame_blend_vector.z);
				}
			}
		}
		wait(lua_state, 1.0);
	}
}

unsafe extern "C" fn sound_tame(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 20.0);
    if is_excute(agent) {
		let sound = SoundModule::play_se(boma, Hash40::new("se_zelda_magic01"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, sound as i32, 0.1, 0);
	}
	frame(lua_state, 50.0);
    if is_excute(agent) {
		let sound = SoundModule::play_se(boma, Hash40::new("se_zelda_magic01"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, sound as i32, 0.1, 0);
	}
	frame(lua_state, 80.0);
    if is_excute(agent) {
		let sound = SoundModule::play_se(boma, Hash40::new("se_zelda_magic01"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, sound as i32, 0.1, 0);
	}
	frame(lua_state, 112.0);
    if is_excute(agent) {
		let sound = SoundModule::play_se(boma, Hash40::new("se_zelda_magic01"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, sound as i32, 0.1, 0);
	}
    frame(lua_state, 141.0);//20 before hitbox
    if is_excute(agent) {
		let sound = SoundModule::play_se(boma, Hash40::new("se_zelda_appeal_s01"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, sound as i32, 1.1, 0);
	}
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_tame", effect_tame, Priority::Low);
	agent.acmd("sound_tame", sound_tame, Priority::Low);
}
