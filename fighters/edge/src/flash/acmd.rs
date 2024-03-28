use super::*;

unsafe extern "C" fn game_attack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let edge = utils::util::get_battle_object_from_id(owner_id);
		if VarModule::is_flag(edge, vars::edge::status::FLASH_HOLD) {
			let pos_x = PostureModule::pos_x(boma);
			let pos_y = PostureModule::pos_y(boma);
            let facing = PostureModule::lr(boma);
			PostureModule::set_pos(boma, &Vector3f::new(pos_x + (35.0 * PostureModule::lr(boma)), pos_y, 0.0));
		}
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 366, 65, 60, 40, 12.0, 0.0, 1.5, 0.0, None, None, None, 0.8, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 5, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting_flash"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.5, 60, 70, 0, 80, 13.0, 0.0, 1.5, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting_flash"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attack", game_attack);
}
