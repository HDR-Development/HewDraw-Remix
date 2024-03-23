use super::*;
unsafe extern "C" fn game_fly(agent: &mut L2CAgentBase) {
  let lua_state = agent.lua_state_agent;
  let boma = agent.boma();
  let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
  // Tap Input
  if !WorkModule::is_flag(boma, *WEAPON_MIISWORDSMAN_CHAKRAM_INSTANCE_WORK_ID_FLAG_FLICK){
      if is_excute(agent) {
          VarModule::on_flag(owner_module_accessor.object(), vars::miiswordsman::instance::CHAKRAM_STICK_ATTACK);
          ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 365, 40, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -0.5, 0.0, 8, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
          AttackModule::set_add_reaction_frame(boma, 0, -3.0, false);
          AttackModule::enable_safe_pos(boma);
      }
      frame(lua_state, 37.0);
      if is_excute(agent) {
          ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 40, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, -0.5, 0.0, 8, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
          AttackModule::set_add_reaction_frame(boma, 0, -3.0, false);
          AttackModule::enable_safe_pos(boma);
      }
  }
  // Hold Input
  else{
      if is_excute(agent) {
          VarModule::on_flag(owner_module_accessor.object(), vars::miiswordsman::instance::CHAKRAM_STICK_ATTACK);
          ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 85, 50, 0, 60, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
          AttackModule::enable_safe_pos(boma);
      }
  }
}
unsafe extern "C" fn game_flynormalsub(agent: &mut L2CAgentBase) {
  let lua_state = agent.lua_state_agent;
  let boma = agent.boma();
  if is_excute(agent) {
      ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 365, 40, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -0.5, 0.0, 8, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
      AttackModule::set_add_reaction_frame(boma, 0, -2.0, false);
      AttackModule::enable_safe_pos(boma);
  }
  frame(lua_state, 37.0);
  if is_excute(agent) {
      ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 40, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, -0.5, 0.0, 8, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
      AttackModule::set_add_reaction_frame(boma, 0, -2.0, false);
      AttackModule::enable_safe_pos(boma);
  }
}
unsafe extern "C" fn game_flyflicksub(agent: &mut L2CAgentBase) {
  let lua_state = agent.lua_state_agent;
  let boma = agent.boma();
  if is_excute(agent) {
      ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 85, 50, 0, 60, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
      AttackModule::enable_safe_pos(boma);
  }
}
unsafe extern "C" fn miiswordsman_chakram_hop_game(agent: &mut L2CAgentBase) {
  let lua_state = agent.lua_state_agent;
  let boma = agent.boma();
  let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
  if is_excute(agent) {
      VarModule::off_flag(owner_module_accessor.object(), vars::miiswordsman::instance::CHAKRAM_STICK_ATTACK);
  }
}

unsafe extern "C" fn game_stick(agent: &mut L2CAgentBase) {
  let lua_state = agent.lua_state_agent;
  let boma = agent.boma();
  if is_excute(agent) {
      SEARCH(agent, 0, 0, Hash40::new("top"), 4.0, 0.0, 0.0, 0.0, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, true);
  }
      
}

unsafe extern "C" fn effect_stick(agent: &mut L2CAgentBase) {
  let lua_state = agent.lua_state_agent;
  let boma = agent.boma();
  frame(lua_state, 142.0);
  if is_excute(agent) {
      EFFECT(agent, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0, 1, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
  }
      
}

pub fn install(agent: &mut Agent) {
  agent.acmd("game_fly", game_fly);
  agent.acmd("game_flynormalsub", game_flynormalsub);
  agent.acmd("game_flyflicksub", game_flyflicksub);
  //.acmd("game_hop", miiswordsman_chakram_hop_game);
  agent.acmd("game_stick", game_stick);
  agent.acmd("effect_stick", effect_stick);
}
