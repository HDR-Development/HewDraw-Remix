use super::*;

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        //println!("H");
        WorkModule::set_float(boma, 10.0, *WEAPON_DUCKHUNT_RETICLE_INSTANCE_WORK_ID_FLOAT_SIZE);
        //ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 60, 100, 0, 30, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_beams"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialhi2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        //println!("HH");
        WorkModule::set_float(boma, 20.0, *WEAPON_DUCKHUNT_RETICLE_INSTANCE_WORK_ID_FLOAT_SIZE);
        //ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 60, 100, 0, 30, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -0.9, 0.0, 0, true, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_beams"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::set_float(boma, 3.0, *WEAPON_DUCKHUNT_RETICLE_INSTANCE_WORK_ID_FLOAT_SIZE);
        //ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 60, 100, 0, 30, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -0.9, 0.0, 0, true, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_beams"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("effect_specialhi", acmd_stub, Priority::Low);
    agent.acmd("sound_specialhi", acmd_stub, Priority::Low);

    agent.acmd("game_specialhi2", game_specialhi2, Priority::Low);
    agent.acmd("effect_specialhi2", acmd_stub, Priority::Low);
    agent.acmd("sound_specialhi2", acmd_stub, Priority::Low);

    agent.acmd("game_specialhi3", game_specialhi3, Priority::Low);
    agent.acmd("effect_specialhi3", acmd_stub, Priority::Low);
    agent.acmd("sound_specialhi3", acmd_stub, Priority::Low);
}