use super::*;

unsafe extern "C" fn game_attackshort(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("cakram1"), 5.0, 70, 95, 0, 40, 0.7, 0.0, 2.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
        AttackModule::enable_safe_pos(boma);
        AttackModule::disable_tip(boma);
        AttackModule::set_damage_shake_scale(boma, 0.5);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("cakram1"), 5.0, 70, 95, 0, 40, 3.0, 0.0, 2.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
    }
    let clearFrame = if(!WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR)) {8.0} else {6.0};
    let reboundFrame = if(!WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR)) {5.0} else {4.0};
    frame(lua_state, reboundFrame);
    //Rebound hitbox
    if is_excute(agent) {
        if (!WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)){
            ControlModule::set_rumble(boma, Hash40::new("rbkind_76_hotling"), 20, false, *BATTLE_OBJECT_ID_INVALID as u32);
            
            ATTACK(agent, 0, 0, Hash40::new("cakram1"), 6.0, 90, 0, 10, 60, 3.0, 0.0, 2.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
            //AttackModule::set_add_reaction_frame_revised(boma, 0, 4.0, false);
        }
    }
    frame(lua_state, clearFrame);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_attacklong(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("cakram1"), 10.0, 70, 69, 0, 30, 0.7, 0.0, 2.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
        AttackModule::enable_safe_pos(boma);
        AttackModule::disable_tip(boma);
        AttackModule::set_damage_shake_scale(boma, 0.5);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("cakram1"), 10.0, 70, 69, 0, 30, 3.0, 0.0, 2.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_76_hotling"), 24, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        if(WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)){
            AttackModule::clear_all(boma);
        }
        else{
            //Rebound hitbox
            ControlModule::set_rumble(boma, Hash40::new("rbkind_76_hotling"), 20, false, *BATTLE_OBJECT_ID_INVALID as u32);
            
            ATTACK(agent, 0, 0, Hash40::new("cakram1"), 12.0, 90, 0, 10, 60, 3.0, 0.0, 2.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
            //AttackModule::set_add_reaction_frame_revised(boma, 0, 4.0, false);
        }
    }
    let clearFrame = if(!WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR)) {10.0} else {9.0};
    frame(lua_state, clearFrame);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_attacklonghold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("cakram1"), 12.0, 368, 0, 10, 90, 0.7, 0.0, 2.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        AttackModule::enable_safe_pos(boma);
        AttackModule::disable_tip(boma);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("cakram1"), &Vector2f{x: 14.0, y: 1.0}, 6, false);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 7.0, false);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("cakram1"), 12.0, 368, 0, 10, 90, 4.0, 0.0, 2.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_76_hotling"), 24, false, *BATTLE_OBJECT_ID_INVALID as u32);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("cakram1"), &Vector2f{x: 14.0, y: 1.0}, 6, false);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 7.0, false);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        if (WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)){
            AttackModule::clear_all(boma);
        }
        else {
            ATTACK(agent, 0, 0, Hash40::new("cakram1"), 13.8, 368, 0, 10, 60, 4.0, 0.0, 2.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            ControlModule::set_rumble(boma, Hash40::new("rbkind_76_hotling"), 24, false, *BATTLE_OBJECT_ID_INVALID as u32);
            AttackModule::set_vec_target_pos(boma, 0, Hash40::new("cakram1"), &Vector2f{x: 14.0, y: 1.0}, 6, false);
            AttackModule::set_add_reaction_frame_revised(boma, 0, 4.0, false);
        }
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if (WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)){
            AttackModule::clear_all(boma);
        }
        else{
            //Rebound hitbox
            ControlModule::set_rumble(boma, Hash40::new("rbkind_76_hotling"), 20, false, *BATTLE_OBJECT_ID_INVALID as u32);
            ATTACK(agent, 0, 0, Hash40::new("cakram1"), 13.8, 90, 0, 10, 60, 3.0, 0.0, 2.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
            //AttackModule::set_add_reaction_frame_revised(boma, 0, 4.0, false);
        }
    }
    let clearFrame = if(!WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR)) {10.0} else {9.0};
    frame(lua_state, clearFrame);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

//Fly controls when RamRam gets sent outwards
pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackshort", game_attackshort);
    agent.acmd("game_attacklong", game_attacklong);
    agent.acmd("game_attacklonghold", game_attacklonghold);
}
