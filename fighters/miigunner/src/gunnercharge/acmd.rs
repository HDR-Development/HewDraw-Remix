use super::*;

unsafe extern "C" fn game_shoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 42, 0, 14, 1.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
		ATTACK(agent, 1, 0, Hash40::new("top"), 21.0, 50, 80, 0, 27, 6.0, 0.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -6.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
		attack!(agent, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
		AttackModule::enable_safe_pos(boma);
	}
	for _ in 0..1000 {
		wait(lua_state, 1.0);
		if is_excute(agent) {
			let motion_vec = Vector3f{x: 0.5, y: 1.0, z: 1.0};
			KineticModule::mul_speed(boma, &motion_vec, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
		}
	}
    
}

unsafe extern "C" fn sound_shoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if(WorkModule::get_float(boma, *WEAPON_MIIGUNNER_GUNNERCHARGE_INSTANCE_WORK_ID_FLOAT_CHARGE) <= 0.25) {
            PLAY_SE_REMAIN(agent, Hash40::new("se_miigunner_special_n02"));
        }
        else if(WorkModule::get_float(boma, *WEAPON_MIIGUNNER_GUNNERCHARGE_INSTANCE_WORK_ID_FLOAT_CHARGE) <= 0.625) {
            PLAY_SE_REMAIN(agent, Hash40::new("se_miigunner_special_n03"));
        }
        else if(WorkModule::get_float(boma, *WEAPON_MIIGUNNER_GUNNERCHARGE_INSTANCE_WORK_ID_FLOAT_CHARGE) <= 0.875) {
            PLAY_SE_REMAIN(agent, Hash40::new("se_miigunner_special_n04"));
        }
        else {
            PLAY_SE_REMAIN(agent, Hash40::new("se_miigunner_special_n05"));
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_shoot", game_shoot);
    agent.acmd("sound_shoot", sound_shoot);
}