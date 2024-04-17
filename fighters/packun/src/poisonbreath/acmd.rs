use super::*;

unsafe extern "C" fn game_start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.2, 361, 0, 0, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 14, false, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.2, 361, 0, 0, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 14, false, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 2, 0, Hash40::new("top"), 1.8, 361, 0, 0, 0, 10.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 14, false, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 3, 0, Hash40::new("top"), 2.4, 361, 0, 0, 0, 10.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 14, false, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        attack!(agent, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
        attack!(agent, *MA_MSC_CMD_ATTACK_SET_LERP, 2, 3);
        AttackModule::set_no_hop_opponent_all(boma, true, false);
    }
}

unsafe extern "C" fn game_startmax(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.2, 361, 0, 0, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 14, false, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.4, 361, 0, 0, 0, 10.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 14, false, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        AttackModule::set_no_hop_opponent_all(boma, true, false);
    }
}

unsafe extern "C" fn game_shoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.9, 361, 0, 0, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 4, true, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.1, 361, 0, 0, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 4, true, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 2, 0, Hash40::new("top"), 0.5, 361, 0, 0, 0, 10.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 4, true, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 3, 0, Hash40::new("top"), 1.2, 361, 0, 0, 0, 10.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 4, true, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        attack!(agent, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
        attack!(agent, *MA_MSC_CMD_ATTACK_SET_LERP, 2, 3);
        AttackModule::set_no_hop_opponent_all(boma, true, false);
    }
}

unsafe extern "C" fn game_shootmax(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.1, 361, 0, 0, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 7, true, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.2, 361, 0, 0, 0, 10.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 7, true, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        AttackModule::set_no_hop_opponent_all(boma, true, false);
    }
}

unsafe extern "C" fn game_explode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE);
	frame(lua_state, 2.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
        ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 55, 90, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 55, 95, 0, 50, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.1, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
    }
	wait(lua_state, 3.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	}
	frame(lua_state, 15.0);
	if is_excute(agent) {
		WorkModule::set_int(boma, 1, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
	}
}	

unsafe extern "C" fn effect_explode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE);
	if is_excute(agent) {
		EffectModule::kill_kind(boma, Hash40::new("packun_poison_breath"), false, false);
		EffectModule::kill_kind(boma, Hash40::new("packun_poison_breath2"), false, false);
		EffectModule::kill_kind(boma, Hash40::new("packun_poison_gas"), false, false);
		EffectModule::kill_kind(boma, Hash40::new("packun_poison_max"), false, false);
		EffectModule::kill_kind(boma, Hash40::new("packun_poison_max_smoke"), false, false);
		EffectModule::kill_kind(boma, Hash40::new("packun_poison_mouth"), false, false);
		EffectModule::kill_kind(boma, Hash40::new("packun_poison_mouth2"), false, false);
		EFFECT(agent, Hash40::new("sys_flame"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.75, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_RATE(agent, 0.7);
    }
}

unsafe extern "C" fn sound_explode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE);
	if is_excute(agent) {
		PLAY_SE(agent, Hash40::new("se_common_bomb_l"));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_start", game_start);
    agent.acmd("game_startmax", game_startmax);

    agent.acmd("game_shoot", game_shoot);
    agent.acmd("game_shootmax", game_shootmax);
    
    agent.acmd("game_explode", game_explode);
    agent.acmd("effect_explode", effect_explode);
    agent.acmd("sound_explode", sound_explode);
}