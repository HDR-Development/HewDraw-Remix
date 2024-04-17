use super::*;

unsafe extern "C" fn game_straight(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_MIIGUNNER {
       let gunner = utils::util::get_battle_object_from_id(owner_id);
       VarModule::set_int(gunner, vars::miigunner::instance::MISSILE_OBJECT_ID, agent.battle_object_id as i32);
    }
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 361, 90, 0, 50, 3.0, 0.0, 0.0, 1.2, Some(0.0), Some(0.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
       if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_MIIGUNNER {
           let gunner = utils::util::get_battle_object_from_id(owner_id);
           VarModule::on_flag(gunner, vars::miigunner::instance::DETONATE_READY);
       }
    }
}

unsafe extern "C" fn game_sburst(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_MIIGUNNER {
       let gunner = utils::util::get_battle_object_from_id(owner_id);
       VarModule::off_flag(gunner, vars::miigunner::instance::DETONATE_READY);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        let gunner = utils::util::get_battle_object_from_id(owner_id);
        if VarModule::is_flag(gunner, vars::miigunner::status::MISSILE_DETONATE) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 50, 75, 0, 70, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_BOMB);
        }
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}

unsafe extern "C" fn effect_sburst(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    if is_excute(agent) {
       if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_MIIGUNNER {
           let gunner = utils::util::get_battle_object_from_id(owner_id);
           if VarModule::is_flag(gunner, vars::miigunner::status::MISSILE_DETONATE) {
               EFFECT(agent, Hash40::new("miigunner_atk_shot5"), Hash40::new("top"), -10, 0, 0, 0, 0, 0, 1.45, 0, 0, 0, 0, 0, 0, false);
               LAST_EFFECT_SET_COLOR(agent, 0.5, 10.0, 25.0);
           }
           else {
               EFFECT(agent, Hash40::new("sys_misfire"), Hash40::new("top"), 0, -1, 2, 0, 0, 0, 1.75, 0, 0, 0, 0, 0, 0, false);
               EFFECT(agent, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
               LAST_EFFECT_SET_RATE(agent, 1.5);
           }
       }
    }
}

unsafe extern "C" fn sound_sburst(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    if is_excute(agent) {
        if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_MIIGUNNER {
           let gunner = utils::util::get_battle_object_from_id(owner_id);
           if VarModule::is_flag(gunner, vars::miigunner::status::MISSILE_DETONATE) {
               PLAY_SE(agent, Hash40::new("se_miigunner_special_c2_s03"));
               PLAY_SE_REMAIN(agent, Hash40::new("se_common_bomb_l"));
           }
           else {
                PLAY_SE_REMAIN(agent, Hash40::new("se_common_bomb_s"));
           }
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_straight", game_straight);
    agent.acmd("game_sburst", game_sburst);
    agent.acmd("effect_sburst", effect_sburst);
    agent.acmd("sound_sburst", sound_sburst);
}