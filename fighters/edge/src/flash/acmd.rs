use super::*;

unsafe extern "C" fn game_wait(agent: &mut L2CAgentBase) {
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
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 366, 65, 60, 40, 1.0, 0.0, 0.0, 0.0, None, None, None, 0.8, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 5, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NONE, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting_flash"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
    }
}

unsafe extern "C" fn effect_wait(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        let flash_handle = EffectModule::req_follow(boma, Hash40::new("edge_senkou_shield"), Hash40::new("top"), &Vector3f::new(0.0, 2.0, 0.0), &Vector3f::new(0.0, -55.0, 0.0), 0.7, false, 0, 0, 0, 0, 0, false, false);
        VarModule::set_int64(agent.battle_object, vars::edge_flash::status::EFFECT_HANDLE, flash_handle);
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        let flash_handle = VarModule::get_int64(agent.battle_object, vars::edge_flash::status::EFFECT_HANDLE);
        EffectModule::set_rate(boma, flash_handle as u32, (30.0 - 10.0)/90.0);
    }
    for _ in 0..100 {
        wait(lua_state, 90.0);
        if is_excute(agent) {
            let flash_handle = VarModule::get_int64(agent.battle_object, vars::edge_flash::status::EFFECT_HANDLE);
            EffectModule::set_rate(boma, flash_handle as u32, 1.0);
        }
        wait(lua_state, 10.0);
        if is_excute(agent) {
            let flash_handle = EffectModule::req_follow(boma, Hash40::new("edge_senkou_shield"), Hash40::new("top"), &Vector3f::new(0.0, 2.0, 0.0), &Vector3f::new(0.0, -55.0, 0.0), 0.7, false, 0, 0, 0, 0, 0, false, false);
            EffectModule::set_frame(boma, flash_handle as u32, 10.0);
            EffectModule::set_rate(boma, flash_handle as u32, (30.0 - 10.0)/90.0);
            VarModule::set_int64(agent.battle_object, vars::edge_flash::status::EFFECT_HANDLE, flash_handle);
        }
    }
}

unsafe extern "C" fn sound_wait(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_edge_special_l01_01"));
    }

}

unsafe extern "C" fn game_attack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        let owner = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        ArticleModule::remove(owner, *FIGHTER_EDGE_GENERATE_ARTICLE_FLASH, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
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

unsafe extern "C" fn game_vanish(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

}

unsafe extern "C" fn effect_vanish(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    
}

unsafe extern "C" fn sound_vanish(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_wait", game_wait);
    agent.acmd("effect_wait", effect_wait);
    agent.acmd("sound_wait", sound_wait);

    agent.acmd("game_attack", game_attack);

    agent.acmd("game_vanish", game_vanish);
    agent.acmd("effect_vanish", effect_vanish);
    agent.acmd("sound_vanish", sound_vanish);
}