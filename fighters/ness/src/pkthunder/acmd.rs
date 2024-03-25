use super::*;

unsafe extern "C" fn game_attackfly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        //Rebound hitbox
        ATTACK(agent, 0, 0, Hash40::new("cakram1"), 2.5, 50, 40, 0, 60, 1.75, 0.0, 2.0, 0.0, Some(0.0), Some(-2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(boma);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("cakram1"), 2.5, 35, 40, 0, 60, 3.0, 0.0, 2.0, 0.0, Some(0.0), Some(-2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(boma);
        AttackModule::clear(boma,1,false);
    }
    let clearFrame = if !WorkModule::is_flag(boma, *WEAPON_TANTAN_RING_INSTANCE_WORK_ID_FLAG_IS_AIR)
    && WorkModule::is_flag(boma, *WEAPON_TANTAN_RING_INSTANCE_WORK_ID_FLAG_IS_LONG)
    {7.0} else {6.0};
    frame(lua_state, clearFrame);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

//Fire ramram
unsafe extern "C" fn game_attacks4fly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    //Scoop for the first few frames, then send back and down
    frame(lua_state, 1.0);
    if is_excute(agent) {
        //Rebound hitbox
        ATTACK(agent, 0, 0, Hash40::new("cakram1"), 4.0, 50, 0, 10, 90, 2.0, 0.0, 2.0, 0.0, Some(0.0), Some(-2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(boma);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("cakram1"), 4.0, 35, 0, 10, 90, 4.0, 0.0, 2.0, 0.0, Some(0.0), Some(-2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        AttackModule::clear(boma,1,false);
    }
    let clearFrame = if !WorkModule::is_flag(boma, *WEAPON_TANTAN_RING_INSTANCE_WORK_ID_FLAG_IS_AIR)
    && WorkModule::is_flag(boma, *WEAPON_TANTAN_RING_INSTANCE_WORK_ID_FLAG_IS_LONG)
    {7.0} else {6.0};
    frame(lua_state, clearFrame);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install(agent: &mut Agent) {
agent.acmd("game_move", ness_pkthunder_move);
agent.acmd("game_movechild", ness_pkthunder_move_child);
}
