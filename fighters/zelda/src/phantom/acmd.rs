use super::*;

unsafe extern "C" fn game_build(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 0.0);
	if is_excute(agent) {
		let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let zelda = utils::util::get_battle_object_from_id(owner_id);
		if VarModule::is_flag(zelda, vars::zelda::instance::HIT_CANCEL_PHANTOM) {
			let pos_x = PostureModule::pos_x(boma);
			let pos_y = PostureModule::pos_y(boma);
			let pos = Vector3f { x: pos_x + 35.2 * (1.0*PostureModule::lr(boma)) , y: pos_y, z: 0.0 };
			PostureModule::set_pos(boma, &pos);
			VarModule::off_flag(zelda, vars::zelda::instance::HIT_CANCEL_PHANTOM);
		}
		WorkModule::inc_int(boma, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_BUILD_NEXT);
	}
	frame(lua_state, 2.0);
	if is_excute(agent) {
		WorkModule::inc_int(boma, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_BUILD_NEXT);
	}
	frame(lua_state, 9.0);
	if is_excute(agent) {
		WorkModule::inc_int(boma, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_BUILD_NEXT);
	}
	frame(lua_state, 12.0);
	if is_excute(agent) {
		WorkModule::inc_int(boma, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_BUILD_NEXT);
	}
	frame(lua_state, 14.0);
	if is_excute(agent) {
		WorkModule::inc_int(boma, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_BUILD_NEXT);
	}
	frame(lua_state, 17.0);
	if is_excute(agent) {
		WorkModule::inc_int(boma, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_BUILD_NEXT);
	}
	frame(lua_state, 19.0);
	if is_excute(agent) {
		WorkModule::inc_int(boma, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_BUILD_NEXT);
	}
	frame(lua_state, 27.0);
	if is_excute(agent) {
		WorkModule::inc_int(boma, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_BUILD_NEXT);
	}
	frame(lua_state, 29.0);
	if is_excute(agent) {
		WorkModule::inc_int(boma, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_BUILD_NEXT);
	}
	frame(lua_state, 31.0);
	if is_excute(agent) {
		WorkModule::inc_int(boma, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_BUILD_NEXT);
	}
	frame(lua_state, 37.0);
	if is_excute(agent) {
		WorkModule::inc_int(boma, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_BUILD_NEXT);
	}
	frame(lua_state, 44.0);
	if is_excute(agent) {
		WorkModule::inc_int(boma, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_BUILD_NEXT);
	}
	frame(lua_state, 49.0);
	if is_excute(agent) {
		WorkModule::inc_int(boma, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_BUILD_NEXT);
	}
	frame(lua_state, 180.0);
	if is_excute(agent) {
		WorkModule::inc_int(boma, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_BUILD_NEXT);
	}   
}

unsafe extern "C" fn effect_build(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	if is_excute(agent) {
		EFFECT(agent, Hash40::new("zelda_phantom_build"), Hash40::new("trans"), 0, 3, 0, 0, -90, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
		let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
		if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_ZELDA {
			let zelda = utils::util::get_battle_object_from_id(owner_id);
			let handle = EffectModule::req_follow(boma, Hash40::new("zelda_entry"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::new(0.0, 150.0, 0.0), 0.75, false, 0, 0, 0, 0, 0, false, false);
			VarModule::set_int64(zelda, vars::zelda::instance::PHANTOM_EFF_HANDLER, handle);
		}
		LAST_EFFECT_SET_COLOR(agent, 0.4, 0.0, 1.0);
	}
	frame(lua_state, 5.0);
	if is_excute(agent) {
		EFFECT_FOLLOW(agent, Hash40::new("zelda_phantom_body_aura"), Hash40::new("toel"), 0, 0, 0, 180, 0, 0, 1.5, true);
		EFFECT_FOLLOW(agent, Hash40::new("zelda_phantom_body_aura"), Hash40::new("toer"), 0, 0, 0, 180, 0, 0, 1.5, true);
		EFFECT_FOLLOW(agent, Hash40::new("zelda_phantom_body_aura"), Hash40::new("footl"), 0, 0, 0, 0, 0, 90, 1.5, true);
		EFFECT_FOLLOW(agent, Hash40::new("zelda_phantom_body_aura"), Hash40::new("footr"), 0, 0, 0, 0, 0, 90, 1.5, true);
	}
	frame(lua_state, 8.0);
	if is_excute(agent) {
		EFFECT_FOLLOW(agent, Hash40::new("zelda_phantom_body_aura"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 90, 1.3, true);
	}
	frame(lua_state, 9.0);
	if is_excute(agent) {
		EFFECT_FOLLOW(agent, Hash40::new("zelda_phantom_body_aura"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 90, 1.3, true);
	}
	frame(lua_state, 14.0);
	if is_excute(agent) {
		EFFECT_FOLLOW(agent, Hash40::new("zelda_phantom_body_aura"), Hash40::new("hip"), -2, -1, 0, 0, 0, 90, 1.8, true);
	}
}

unsafe extern "C" fn game_attackkick(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	let rush_speed = 4.5;
	frame(lua_state, 0.0);
	FT_MOTION_RATE(agent, 6.0/(5.0-0.0));
	if is_excute(agent) {
		KineticModule::unable_energy(boma, *WEAPON_ZELDA_PHANTOM_KINETIC_ENERGY_ID_NORMAL);
	}
	frame(lua_state, 5.0);
	FT_MOTION_RATE(agent, 1.0);
	if is_excute(agent) {
		KineticModule::enable_energy(boma, *WEAPON_ZELDA_PHANTOM_KINETIC_ENERGY_ID_NORMAL);
		agent.clear_lua_stack();
		lua_args!(agent, WEAPON_ZELDA_PHANTOM_KINETIC_ENERGY_ID_NORMAL, rush_speed * PostureModule::lr(boma));
		app::sv_kinetic_energy::set_speed(lua_state);
		agent.clear_lua_stack();
	}
	frame(lua_state, 9.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 361, 50, 0, 50, 5.5, 0.0, 6.0, 14.0, Some(0.0), Some(6.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
	}
	wait(lua_state, 6.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	}
}

unsafe extern "C" fn effect_attackkick(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	if is_excute(agent) {
		if boma.is_situation(*SITUATION_KIND_GROUND) {
			EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
		}
	}
	frame(lua_state, 10.0);
	if is_excute(agent) {
		EFFECT_OFF_KIND(agent, Hash40::new("zelda_entry"), false, false);
		let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
		if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_ZELDA {
			let zelda = utils::util::get_battle_object_from_id(owner_id);
			let handle = VarModule::get_int64(zelda, vars::zelda::instance::PHANTOM_EFF_HANDLER);
			EffectModule::set_scale(boma, handle as u32, &Vector3f::zero());
		}
	}
	frame(lua_state, 33.0);
	if is_excute(agent) {
		if boma.is_situation(*SITUATION_KIND_GROUND) {
			EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 3, 0, 12, 0, 180, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
			LAST_EFFECT_SET_RATE(agent, 1.5);
		}
	}
	frame(lua_state, 45.0);
	if is_excute(agent) {
		EFFECT(agent, Hash40::new("zelda_phantom_end"), Hash40::new("trans"), 0, 4.5, 7, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_RATE(agent, 1.8);
	}
}

unsafe extern "C" fn game_attackpunch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	let rush_speed = 5.0;
	frame(lua_state, 0.0);
	FT_MOTION_RATE(agent, 2.0/(0.5-0.0));
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 100, 60, 0, 4.0, 0.0, 7.0, 11.0, Some(0.0), Some(7.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, true, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 361, 100, 40, 0, 6.0, 0.0, 7.0, 11.0, Some(0.0), Some(7.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, true, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		KineticModule::unable_energy(boma, *WEAPON_ZELDA_PHANTOM_KINETIC_ENERGY_ID_NORMAL);
	}
	frame(lua_state, 0.5);
	FT_MOTION_RATE(agent, 1.0/(1.0-0.5));
	frame(lua_state, 1.0);
	FT_MOTION_RATE(agent, 1.0);
	if is_excute(agent) {
		KineticModule::enable_energy(boma, *WEAPON_ZELDA_PHANTOM_KINETIC_ENERGY_ID_NORMAL);
		agent.clear_lua_stack();
		lua_args!(agent, WEAPON_ZELDA_PHANTOM_KINETIC_ENERGY_ID_NORMAL, rush_speed * PostureModule::lr(boma));
		app::sv_kinetic_energy::set_speed(lua_state);
		agent.clear_lua_stack();
	}
	frame(lua_state, 7.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
		ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 65, 0, 70, 5.5, 0.0, 10.0, 11.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 6, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
		ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 361, 65, 0, 70, 4.5, 0.0, 9.5, 19.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 6, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
	}
	wait(lua_state, 7.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	}
}

unsafe extern "C" fn effect_attackpunch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	if is_excute(agent) {
		if boma.is_situation(*SITUATION_KIND_GROUND) {
			EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
		}
	}
	frame(lua_state, 10.0);
	if is_excute(agent) {
		EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 8, 4, 0, 0, 0, 1, true);
		EFFECT_OFF_KIND(agent, Hash40::new("zelda_entry"), false, false);
		let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
		if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_ZELDA {
			let zelda = utils::util::get_battle_object_from_id(owner_id);
			let handle = VarModule::get_int64(zelda, vars::zelda::instance::PHANTOM_EFF_HANDLER);
			EffectModule::set_scale(boma, handle as u32, &Vector3f::zero());
		}
	}
	frame(lua_state, 57.0);
	if is_excute(agent) {
		EFFECT(agent, Hash40::new("zelda_phantom_end"), Hash40::new("trans"), 0, 4.5, 4, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_RATE(agent, 1.7);
	}
}

unsafe extern "C" fn game_attacks(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	let rush_speed = 5.5;
	frame(lua_state, 0.0);
	if is_excute(agent) {
		//FT_MOTION_RATE(agent, 5.0/(3.0-0.0));
		KineticModule::unable_energy(boma, *WEAPON_ZELDA_PHANTOM_KINETIC_ENERGY_ID_NORMAL);
	}
	frame(lua_state, 3.0);
	FT_MOTION_RATE(agent, 1.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 100, 50, 0, 5.0, 0.0, 8.0, 10.0, Some(0.0), Some(8.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, true, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 361, 100, 40, 0, 7.0, 0.0, 8.0, 10.0, Some(0.0), Some(8.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, true, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		KineticModule::enable_energy(boma, *WEAPON_ZELDA_PHANTOM_KINETIC_ENERGY_ID_NORMAL);
		agent.clear_lua_stack();
		lua_args!(agent, WEAPON_ZELDA_PHANTOM_KINETIC_ENERGY_ID_NORMAL, rush_speed * PostureModule::lr(boma));
		app::sv_kinetic_energy::set_speed(lua_state);
		agent.clear_lua_stack();
	}
	frame(lua_state, 7.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
		ATTACK(agent, 0, 0, Hash40::new("handr"), 6.0, 87, 40, 0, 100, 5.8, 0.0, 0.0, 1.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		ATTACK(agent, 1, 0, Hash40::new("handr"), 6.0, 87, 40, 0, 100, 5.8, 0.0, 0.0, 8.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		ATTACK(agent, 2, 0, Hash40::new("handr"), 6.0, 87, 40, 0, 100, 5.8, 0.0, 0.0, 16.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		ATTACK(agent, 3, 0, Hash40::new("top"), 6.0, 87, 40, 0, 100, 6.5, 0.0, 8.0, 10.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
	}
	wait(lua_state, 5.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	}
}

unsafe extern "C" fn effect_attacks(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	if is_excute(agent) {
		EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("top"), 1.0, 20.0, -10.0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_COLOR(agent, 0.885, 0.051, 0.051);
		LAST_EFFECT_SET_RATE(agent, 0.50);
		EFFECT(agent, Hash40::new("zelda_appeal_s_fire"), Hash40::new("top"), 1.0, 20.0, -10.0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_RATE(agent, 0.75);
		if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
			EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
		}
		//EFFECT_FOLLOW(agent, Hash40::new("zelda_din_start"), Hash40::new("handr"), 0, 0, 1.0, 0, 0, 0, 1, true);
		//LAST_EFFECT_SET_RATE(agent, 0.5);
		EFFECT_FOLLOW(agent, Hash40::new("zelda_din_start"), Hash40::new("handr"), 0, 0, 8.0, 0, 0, 0, 0.5, true);
		LAST_EFFECT_SET_RATE(agent, 0.5);
		EFFECT_FOLLOW(agent, Hash40::new("zelda_din_start"), Hash40::new("handr"), 0, 0, 16.0, 0, 0, 0, 0.75, true);
		LAST_EFFECT_SET_RATE(agent, 0.5);
	}
	frame(lua_state, 3.0);
	if is_excute(agent) {
		EFFECT_OFF_KIND(agent, Hash40::new("zelda_phantom_build"), false, true);
		let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
		if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_ZELDA {
			let zelda = utils::util::get_battle_object_from_id(owner_id);
			let handle = VarModule::get_int64(zelda, vars::zelda::instance::PHANTOM_EFF_HANDLER);
			EffectModule::set_scale(boma, handle as u32, &Vector3f::zero());
		}
		if PostureModule::lr(boma) > 0.0 {
			EFFECT_FOLLOW(agent, Hash40::new("zelda_phantom_line"), Hash40::new("top"), -5, 10, -5, 0, 0, 0, 1, true);
		}
		else{
			EFFECT_FOLLOW(agent, Hash40::new("zelda_phantom_line"), Hash40::new("top"), 5, 10, -5, 0, 0, 0, 1, true);		
		}
		AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_zelda_phantomsword1"), Hash40::new("tex_zelda_phantomsword2"), 6, Hash40::new("handr"), 1.5, 0.4, -1.0, Hash40::new("handr"), 1.5, 0.4, 24.4, true, Hash40::new("zelda_phantom_sword"), Hash40::new("handr"), 1.85, 0.35, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.5);

		//EFFECT_FOLLOW(agent, Hash40::new("zelda_din_start"), Hash40::new("handr"), 0, 0, 1.0, 0, 0, 0, 1, true);
		//LAST_EFFECT_SET_RATE(agent, 0.75);
		EFFECT_FOLLOW(agent, Hash40::new("zelda_din_start"), Hash40::new("handr"), 0, 0, 8.0, 0, 0, 0, 0.5, true);
		LAST_EFFECT_SET_RATE(agent, 0.75);
		EFFECT_FOLLOW(agent, Hash40::new("zelda_din_start"), Hash40::new("handr"), 0, 0, 16.0, 0, 0, 0, 0.75, true);
		LAST_EFFECT_SET_RATE(agent, 0.75);
	}
	frame(lua_state, 7.0);
	if is_excute(agent) {
		QUAKE(agent, *CAMERA_QUAKE_KIND_S);
	}
	frame(lua_state, 12.0);
	if is_excute(agent) {
		EFFECT_OFF_KIND(agent, Hash40::new("zelda_phantom_line"), false, false);
	}
	frame(lua_state, 30.0);
	if is_excute(agent) {
		AFTER_IMAGE_OFF(agent, 4);
	}
	frame(lua_state, 55.0);
	if is_excute(agent) {
		EFFECT(agent, Hash40::new("zelda_phantom_end"), Hash40::new("top"), 0, 4, -3, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_RATE(agent, 1.4);
	}
}

unsafe extern "C" fn game_attackl(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	let rush_speed = 6.0;
	frame(lua_state, 0.0);
	if is_excute(agent) {
		//FT_MOTION_RATE(agent, 20.0/3.0);
		KineticModule::unable_energy(boma, *WEAPON_ZELDA_PHANTOM_KINETIC_ENERGY_ID_NORMAL);
	}
	frame(lua_state, 3.0);
	FT_MOTION_RATE(agent, 1.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 100, 110, 0, 5.5, 0.0, 8.0, 10.0, Some(0.0), Some(8.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, true, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 361, 100, 80, 0, 7.0, 0.0, 8.0, 10.0, Some(0.0), Some(8.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, true, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		KineticModule::enable_energy(boma, *WEAPON_ZELDA_PHANTOM_KINETIC_ENERGY_ID_NORMAL);
		agent.clear_lua_stack();
		lua_args!(agent, WEAPON_ZELDA_PHANTOM_KINETIC_ENERGY_ID_NORMAL, rush_speed * PostureModule::lr(boma));
		app::sv_kinetic_energy::set_speed(lua_state);
		agent.clear_lua_stack();
	}
	frame(lua_state, 6.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
		// Air-only
		ATTACK(agent, 0, 0, Hash40::new("handr"), 8.0, 361, 60, 0, 60, 5.0, 2.0, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
		ATTACK(agent, 1, 0, Hash40::new("handr"), 8.0, 361, 60, 0, 60, 5.6, 2.0, 0.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
		ATTACK(agent, 2, 0, Hash40::new("handr"), 8.0, 361, 60, 0, 60, 5.6, 2.0, 0.0, 16.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
		//ATTACK(agent, 3, 0, Hash40::new("top"), 5.0, 361, 57, 0, 60, 5.0, 0.0, 8.5, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
		// Ground-only
		ATTACK(agent, 3, 0, Hash40::new("handr"), 8.0, 262, 60, 0, 90, 5.0, 2.0, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
		ATTACK(agent, 4, 0, Hash40::new("handr"), 8.0, 262, 60, 0, 90, 5.6, 2.0, 0.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
		ATTACK(agent, 5, 0, Hash40::new("handr"), 8.0, 262, 60, 0, 90, 5.6, 2.0, 0.0, 16.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
		//ATTACK(agent, 3, 0, Hash40::new("top"), 5.0, 270, 40, 0, 120, 5.0, 0.0, 8.5, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
		AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
	}
	wait(lua_state, 7.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	}
}

unsafe extern "C" fn effect_attackl(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	if is_excute(agent) {
		EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("top"), 1.0, 20.0, -10.0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_COLOR(agent, 0.4, 0.0, 1.0);
		LAST_EFFECT_SET_RATE(agent, 0.50);
		EFFECT(agent, Hash40::new("zelda_appeal_s_fire"), Hash40::new("top"), 1.0, 20.0, -10.0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_RATE(agent, 0.75);
		LAST_EFFECT_SET_COLOR(agent, 0.4, 0.0, 50.0);
		if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
			EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
		}
		EFFECT_FOLLOW(agent, Hash40::new("zelda_phantom_body_aura"), Hash40::new("toel"), 0, 0, 0, 180, 0, 0, 0.9, true);
		EFFECT_FOLLOW(agent, Hash40::new("zelda_phantom_body_aura"), Hash40::new("toer"), 0, 0, 0, 180, 0, 0, 0.9, true);
		EFFECT_FOLLOW(agent, Hash40::new("zelda_phantom_body_aura"), Hash40::new("footl"), 0, 0, 0, 0, 0, 90, 1, true);
		EFFECT_FOLLOW(agent, Hash40::new("zelda_phantom_body_aura"), Hash40::new("footr"), 0, 0, 0, 0, 0, 90, 1, true);

		/*
		EFFECT_FOLLOW(agent, Hash40::new("zelda_phantom_body_aura"), Hash40::new("handr"), 0, 0, 1.0, 0, 0, 0, 3.0, true);
		LAST_EFFECT_SET_RATE(agent, 0.75);
		EFFECT_FOLLOW(agent, Hash40::new("zelda_phantom_body_aura"), Hash40::new("handr"), 0, 0, 8.0, 0, 0, 0, 3.0, true);
		LAST_EFFECT_SET_RATE(agent, 0.75);
		EFFECT_FOLLOW(agent, Hash40::new("zelda_phantom_body_aura"), Hash40::new("handr"), 0, 0, 16.0, 0, 0, 0, 3.0, true);
		LAST_EFFECT_SET_RATE(agent, 0.75);
		*/

		EFFECT_FOLLOW(agent, Hash40::new("zelda_din_start"), Hash40::new("handr"), 0, 0, 1.0, 0, 0, 0, 0.35, true);
		LAST_EFFECT_SET_RATE(agent, 0.5);
		LAST_EFFECT_SET_COLOR(agent, 0.1, 0.0, 1.0);
		EFFECT_FOLLOW(agent, Hash40::new("zelda_din_start"), Hash40::new("handr"), 0, 0, 8.0, 0, 0, 0, 0.5, true);
		LAST_EFFECT_SET_RATE(agent, 0.5);
		LAST_EFFECT_SET_COLOR(agent, 0.1, 0.0, 1.0);
		EFFECT_FOLLOW(agent, Hash40::new("zelda_din_start"), Hash40::new("handr"), 0, 0, 16.0, 0, 0, 0, 0.75, true);
		LAST_EFFECT_SET_RATE(agent, 0.5);
		LAST_EFFECT_SET_COLOR(agent, 0.1, 0.0, 1.0);
	}
	frame(lua_state, 3.0);
	if is_excute(agent) {
		if PostureModule::lr(boma) > 0.0 {
			EFFECT_FOLLOW(agent, Hash40::new("zelda_phantom_line"), Hash40::new("top"), -5, 10, -5, 0, 0, 0, 1, true);
		}
		else{
			EFFECT_FOLLOW(agent, Hash40::new("zelda_phantom_line"), Hash40::new("top"), 5, 10, -5, 0, 0, 0, 1, true);
		}
		EFFECT_OFF_KIND(agent, Hash40::new("zelda_entry"), false, false);
		let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
		if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_ZELDA {
			let zelda = utils::util::get_battle_object_from_id(owner_id);
			let handle = VarModule::get_int64(zelda, vars::zelda::instance::PHANTOM_EFF_HANDLER);
			EffectModule::set_scale(boma, handle as u32, &Vector3f::zero());
		}
	}
	frame(lua_state, 4.0);
	if is_excute(agent) {
		EFFECT_FOLLOW(agent, Hash40::new("zelda_phantom_sword"), Hash40::new("handr"), 1.85, 0.35, 0, 0, 0, 0, 1, true);
		AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_zelda_phantomsword1"), Hash40::new("tex_zelda_phantomsword2"), 16, Hash40::new("handr"), 1.5, 0.4, -1.0, Hash40::new("handr"), 1.5, 0.4, 24.4, true, Hash40::new("null"), Hash40::new("handr"), 1.85, 0.35, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.7);
		if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
			EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
		}
		EFFECT_FOLLOW(agent, Hash40::new("zelda_din_start"), Hash40::new("handr"), 0, 0, 1.0, 0, 0, 0, 0.35, true);
		LAST_EFFECT_SET_RATE(agent, 0.5);
		LAST_EFFECT_SET_COLOR(agent, 0.1, 0.0, 1.0);
		EFFECT_FOLLOW(agent, Hash40::new("zelda_din_start"), Hash40::new("handr"), 0, 0, 8.0, 0, 0, 0, 0.5, true);
		LAST_EFFECT_SET_RATE(agent, 0.5);
		LAST_EFFECT_SET_COLOR(agent, 0.1, 0.0, 1.0);
		EFFECT_FOLLOW(agent, Hash40::new("zelda_din_start"), Hash40::new("handr"), 0, 0, 16.0, 0, 0, 0, 0.75, true);
		LAST_EFFECT_SET_RATE(agent, 0.5);
		LAST_EFFECT_SET_COLOR(agent, 0.1, 0.0, 1.0);
	}
	frame(lua_state, 9.0);
	if is_excute(agent) {
		AFTER_IMAGE_OFF(agent, 0);
		EFFECT_FOLLOW(agent, Hash40::new("zelda_phantom_sword_trace"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
	}
	frame(lua_state, 12.0);
	if is_excute(agent) {
		if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
			EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
		}
	}
	frame(lua_state, 10.0);
	if is_excute(agent) {
		//QUAKE(agent, *CAMERA_QUAKE_KIND_M);
		QUAKE(agent, *CAMERA_QUAKE_KIND_S);
	}
	frame(lua_state, 15.0);
	if is_excute(agent) {
		EFFECT_OFF_KIND(agent, Hash40::new("zelda_phantom_line"), false, false);
	}
	frame(lua_state, 17.0);
	if is_excute(agent) {
		EFFECT_OFF_KIND(agent, Hash40::new("zelda_phantom_sword"), true, true);
	}
	frame(lua_state, 64.0);
	if is_excute(agent) {
		EFFECT_OFF_KIND(agent, Hash40::new("zelda_phantom_body_aura"), false, true);
	}
	frame(lua_state, 74.0);
	if is_excute(agent) {
		EFFECT(agent, Hash40::new("zelda_phantom_end"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
	}
}

unsafe extern "C" fn game_attackmax(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	let rush_speed = 7.0;
	frame(lua_state, 0.0);
	if is_excute(agent) {
		FT_MOTION_RATE(agent, 11.0/(3.0-0.0));
		KineticModule::unable_energy(boma, *WEAPON_ZELDA_PHANTOM_KINETIC_ENERGY_ID_NORMAL);
	}
	frame(lua_state, 3.0);
	if is_excute(agent) {
		FT_MOTION_RATE(agent, 1.0);
		ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 100, 130, 0, 6.0, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, true, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 6, 100, 85, 0, 8.5, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, true, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		KineticModule::enable_energy(boma, *WEAPON_ZELDA_PHANTOM_KINETIC_ENERGY_ID_NORMAL);
		agent.clear_lua_stack();
		lua_args!(agent, WEAPON_ZELDA_PHANTOM_KINETIC_ENERGY_ID_NORMAL, rush_speed * PostureModule::lr(boma));
		app::sv_kinetic_energy::set_speed(lua_state);
		agent.clear_lua_stack();
	}
	frame(lua_state, 11.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
		ATTACK(agent, 0, 0, Hash40::new("handr"), 11.0, 46, 90, 0, 60, 6.0, 2.0, 0.0, 1.5, None, None, None, 1.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		ATTACK(agent, 1, 0, Hash40::new("handr"), 11.0, 46, 90, 0, 60, 6.0, 2.0, 1.0, 9.0, None, None, None, 1.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		ATTACK(agent, 2, 0, Hash40::new("handr"), 11.0, 46, 90, 0, 60, 6.0, 2.0, 2.0, 16.5, None, None, None, 1.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		ATTACK(agent, 3, 0, Hash40::new("shoulderr"), 11.0, 46, 90, 0, 60, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
	}
	frame(lua_state, 17.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	}
}

unsafe extern "C" fn effect_attackmax(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	if is_excute(agent) {
		EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("top"), 1.0, 20.0, -10.0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_RATE(agent, 0.50);
		EFFECT(agent, Hash40::new("zelda_atk_flash"), Hash40::new("top"), 1.0, 20.0, -10.0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
		if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
			EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
		}
		EFFECT_FOLLOW(agent, Hash40::new("zelda_phantom_body_aura"), Hash40::new("toel"), 0, 0, 0, 180, 0, 0, 1.5, true);
		EFFECT_FOLLOW(agent, Hash40::new("zelda_phantom_body_aura"), Hash40::new("toer"), 0, 0, 0, 180, 0, 0, 1.5, true);
		EFFECT_FOLLOW(agent, Hash40::new("zelda_phantom_body_aura"), Hash40::new("footl"), 0, 0, 0, 0, 0, 90, 1.5, true);
		EFFECT_FOLLOW(agent, Hash40::new("zelda_phantom_body_aura"), Hash40::new("footr"), 0, 0, 0, 0, 0, 90, 1.5, true);
		EFFECT_FOLLOW(agent, Hash40::new("zelda_phantom_body_aura"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 90, 1.3, true);
		EFFECT_FOLLOW(agent, Hash40::new("zelda_phantom_body_aura"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 90, 1.3, true);
		EFFECT_FOLLOW(agent, Hash40::new("zelda_phantom_body_aura"), Hash40::new("hip"), 0, 0, 0, 0, 0, 90, 2, true);
		EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("zelda_phantom_start"), Hash40::new("trans"), 0, 3, 0, -90, 0, 0, 1, true);
	}
	frame(lua_state, 2.0);
	if is_excute(agent) {
		if PostureModule::lr(boma) > 0.0 {
			EFFECT_FOLLOW(agent, Hash40::new("zelda_phantom_line"), Hash40::new("top"), -5, 10, -7, 0, 0, 0, 1, true);
		}
		else{
			EFFECT_FOLLOW(agent, Hash40::new("zelda_phantom_line"), Hash40::new("top"), 5, 10, -7, 0, 0, 0, 1, true);
		}
		
		EFFECT_FOLLOW(agent, Hash40::new("zelda_atk_hi_flash"), Hash40::new("handr"), 0, 0, 16.0, 0, 0, 0, 2.0, true);
	}
	frame(lua_state, 4.0);
	if is_excute(agent) {
		QUAKE(agent, *CAMERA_QUAKE_KIND_M);
		AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_zelda_phantomsword1"), Hash40::new("tex_zelda_phantomsword2"), 9, Hash40::new("handr"), 1.5, 0.4, -1.0, Hash40::new("handr"), 1.5, 0.4, 24.4, true, Hash40::new("zelda_phantom_sword"), Hash40::new("handr"), 1.85, 0.35, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.5);
	}
	frame(lua_state, 20.0);
	if is_excute(agent) {
		if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
			EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 7, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
			LAST_EFFECT_SET_ALPHA(agent, 0.6);
		}
	}
	frame(lua_state, 21.0);
	if is_excute(agent) {
		EFFECT_OFF_KIND(agent, Hash40::new("zelda_phantom_line"), false, false);
	}
	frame(lua_state, 24.0);
	if is_excute(agent) {
		AFTER_IMAGE_OFF(agent, 8);
	}
	frame(lua_state, 119.0);
	if is_excute(agent) {
		EFFECT_DETACH_KIND(agent, Hash40::new("zelda_phantom_body_aura"), 32);
		EFFECT(agent, Hash40::new("zelda_phantom_build"), Hash40::new("trans"), 0, 3, 0, 0, -90, 0, 1, 0, 0, 0, 0, 0, 0, true);
		EFFECT(agent, Hash40::new("zelda_phantom_end"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1.15, 0, 0, 0, 0, 0, 0, true);
	}
}

unsafe extern "C" fn game_cancel(agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let owner_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
	let zelda = utils::util::get_battle_object_from_id(owner_id);
	frame(lua_state, 1.0);
	if VarModule::is_flag(zelda, vars::zelda::instance::HIT_CANCEL_PHANTOM) {
		FT_MOTION_RATE_RANGE(agent, 1.0, 34.0, 99.0);
	}
	frame(lua_state, 30.0);//100
	if VarModule::is_flag(zelda, vars::zelda::instance::HIT_CANCEL_PHANTOM) {
		FT_MOTION_RATE_RANGE(agent, 30.0, 90.0, 320.0); //8 seconds
		VarModule::off_flag(zelda, vars::zelda::instance::HIT_CANCEL_PHANTOM);
	}
}

unsafe extern "C" fn effect_cancel(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("zelda_phantom_hit"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, false);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
		EFFECT_FOLLOW(agent, Hash40::new("zelda_phantom_end2"), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 1.3, false);
    }
    frame(lua_state, 89.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("zelda_phantom_end"), Hash40::new("trans"), 0, 2, 0, 0, 0, 0, 1.18, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.75);
        EFFECT(agent, Hash40::new("zelda_phantom_build"), Hash40::new("trans"), 0, 1.5, 0, 0, -90, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        agent.on_flag(*WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_FLAG_END);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_build", game_build);
    agent.acmd("effect_build", effect_build);

    agent.acmd("game_attackkick", game_attackkick);
    agent.acmd("effect_attackkick", effect_attackkick);

    agent.acmd("game_attackpunch", game_attackpunch);
    agent.acmd("effect_attackpunch", effect_attackpunch);

    agent.acmd("game_attacks", game_attacks);
    agent.acmd("effect_attacks", effect_attacks);

    agent.acmd("game_attackl", game_attackl);
    agent.acmd("effect_attackl", effect_attackl);

    agent.acmd("game_attackmax", game_attackmax);
    agent.acmd("effect_attackmax", effect_attackmax);

    agent.acmd("game_cancel", game_cancel);
    agent.acmd("effect_cancel", effect_cancel);
}
