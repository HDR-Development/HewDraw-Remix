use super::*;

unsafe extern "C" fn game_exp(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackl"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 75, 75, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !WorkModule::is_flag(boma, *WEAPON_EDGE_FLARE2_INSTANCE_WORK_ID_FLAG_REFLECT) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
        }
    }
}

unsafe extern "C" fn game_fly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_furafura"), 15, false, *BATTLE_OBJECT_ID_INVALID as u32);
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 50, 75, 0, 65, 1.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 0.4, 2.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, -3, 0.0, 45, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
}

unsafe extern "C" fn game_try(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackl"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.5, 53, 75, 0, 50, 1.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, -3, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_exp", game_exp);
    agent.acmd("game_fly", game_fly);
    agent.acmd("game_try", game_try);
}
