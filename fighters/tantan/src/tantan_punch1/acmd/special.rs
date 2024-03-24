use super::*;
unsafe extern "C" fn dragon_game_attackshort(agent: &mut L2CAgentBase) {
let lua_state = agent.lua_state_agent;
let boma = agent.boma();

if is_excute(agent) {
      ATTACK(agent, 0, 0, Hash40::new("have"), 8.0, 45, 87, 0, 50, 0.7, 3.1, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
    AttackModule::enable_safe_pos(boma);
    AttackModule::disable_tip(boma);
    AttackModule::set_damage_shake_scale(boma, 0.5);
}
frame(lua_state, 2.0);
if is_excute(agent) {
      ATTACK(agent, 0, 0, Hash40::new("have"), 8.0, 45, 87, 0, 50, 2.9, 3.1, 0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
      ControlModule::set_rumble(boma, Hash40::new("rbkind_76_megabolt"), 24, false, *BATTLE_OBJECT_ID_INVALID as u32);
}
  let clearFrame = if(!WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR)) {12.0} else {9.0};
frame(lua_state, clearFrame);
if is_excute(agent) {
    AttackModule::clear_all(boma);
}
}
unsafe extern "C" fn dragon_game_attacklong(agent: &mut L2CAgentBase) {
let lua_state = agent.lua_state_agent;
let boma = agent.boma();

if is_excute(agent) {
      ATTACK(agent, 0, 0, Hash40::new("have"), 12.0, 45, 78, 0, 40, 0.7, 3.1, 0.5, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.5, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
    AttackModule::enable_safe_pos(boma);
    AttackModule::disable_tip(boma);
    AttackModule::set_damage_shake_scale(boma, 0.5);
}
frame(lua_state, 2.0);
if is_excute(agent) {
      ATTACK(agent, 0, 0, Hash40::new("have"), 12.0, 45, 78, 0, 40, 2.9, 3.1, 0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.5, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
      ControlModule::set_rumble(boma, Hash40::new("rbkind_76_megabolt"), 24, false, *BATTLE_OBJECT_ID_INVALID as u32);
}
frame(lua_state, 4.0);
if is_excute(agent) {
      if(WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)){
        AttackModule::clear_all(boma);
    }
    else{
          ATTACK(agent, 0, 0, Hash40::new("have"), 16.0, 45, 78, 0, 40, 2.9, 3.1, 0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.5, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
          WorkModule::on_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_CHANGE_HIT_EFFECT);
    }
}
frame(lua_state, 7.0);
if is_excute(agent) {
      if(WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)){
        AttackModule::clear_all(boma);
    }
}
frame(lua_state, 9.0);
if is_excute(agent) {
      if(WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)){
        AttackModule::clear_all(boma);
    }
    else 
    {
          ATTACK(agent, 0, 0, Hash40::new("have"), 13.0, 45, 78, 0, 40, 2.9, 3.1, 0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.5, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
          WorkModule::off_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_CHANGE_HIT_EFFECT);
    }
}
  let clearFrame = if(!WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR)) {15.0} else {11.0};
frame(lua_state, clearFrame);
if is_excute(agent) {
    AttackModule::clear_all(boma);
}
}
unsafe extern "C" fn dragon_game_attackdragonshootlong(agent: &mut L2CAgentBase) {
let lua_state = agent.lua_state_agent;
let boma = agent.boma();

if is_excute(agent) {
      ATTACK(agent, 0, 0, Hash40::new("have"), 13.8, 45, 78, 0, 40, 0.7, 5.4, 0.5, 0.3, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4.5, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
    AttackModule::enable_safe_pos(boma);
    AttackModule::disable_tip(boma);
    AttackModule::set_damage_shake_scale(boma, 0.5);
}
frame(lua_state, 1.0);
if is_excute(agent) {
      ATTACK(agent, 0, 0, Hash40::new("have"), 13.8, 45, 78, 0, 40, 3.2, 5.4, 0.5, 0.3, Some(1.0), Some(0.5), Some(0.3), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4.5, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
      ControlModule::set_rumble(boma, Hash40::new("rbkind_76_megabolt"), 24, false, *BATTLE_OBJECT_ID_INVALID as u32);
}
frame(lua_state, 4.0);
if is_excute(agent) {
      if(WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)){
        AttackModule::clear_all(boma);
    }
    else{
          ATTACK(agent, 0, 0, Hash40::new("have"), 16.75, 45, 78, 0, 40, 3.2, 5.4, 0.5, 0.3, Some(1.0), Some(0.5), Some(0.3), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.5, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
          WorkModule::on_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_CHANGE_HIT_EFFECT);
    }
}
  //Dragon Beam linker
frame(lua_state, 6.0);
if is_excute(agent) {
      if(WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)){
        AttackModule::clear_all(boma);
    }
    else{
          let angle: u64 = if WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR) {45} else {361};
          let kbg = if WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR) {70} else {0};
          let fkb = if WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR) {0} else {10};
          let bkb = if WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR) {40} else {60};
          ATTACK(agent, 0, 0, Hash40::new("have"), 16.75, angle, kbg, fkb, bkb, 3.2, 3.1, 0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.5, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
          WorkModule::off_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_CHANGE_HIT_EFFECT);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 5.0, false);
    }
}
frame(lua_state, 9.0);
if is_excute(agent) {
      if(WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND)){
        AttackModule::clear_all(boma);
    }
    else
    {
        AttackModule::set_power(boma, 0,16.0, false);
    }
}
frame(lua_state, 15.0);
if is_excute(agent) {
    AttackModule::clear_all(boma);
}
}
unsafe extern "C" fn dragon_effect_attackdragonshootlong(agent: &mut L2CAgentBase) {
let lua_state = agent.lua_state_agent;
let boma = agent.boma();
if is_excute(agent) {
    EFFECT_FOLLOW(agent, Hash40::new("tantan_wepon1_wind_big"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.25, true);
    LAST_EFFECT_SET_COLOR(agent,1.0,0.8,0.25);
    EffectModule::enable_sync_init_pos_last(boma);
    EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("tantan_wepon_ringwind"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.25, true);
    LAST_EFFECT_SET_COLOR(agent,1.0,0.8,0.25);
    EffectModule::enable_sync_init_pos_last(boma);
    EFFECT_FOLLOW(agent, Hash40::new("tantan_dragon_attack_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.6, true);
    LAST_EFFECT_SET_COLOR(agent,1.0,0.8,0.25);
    EffectModule::enable_sync_init_pos_last(boma);
}
frame(lua_state, 14.0);
if is_excute(agent) {
    EFFECT_FOLLOW(agent, Hash40::new("tantan_dragon_eye_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, true);
    EffectModule::enable_sync_init_pos_last(boma);
}
}

unsafe extern "C" fn dragon_special_hi_attack_game(agent: &mut L2CAgentBase) {
let lua_state = agent.lua_state_agent;
let boma = agent.boma();
let angle = (75.0+WorkModule::get_float(boma,*FIGHTER_TANTAN_INSTANCE_WORK_ID_FLOAT_ATTACK_SHIFT_ANGLE_L)) as u64;
if is_excute(agent) {
      ATTACK(agent, 0, 0, Hash40::new("have"), 12.75, angle, 73, 0, 56, 4.5, 3.0, 0.0, 0.3, Some(-6.0), Some(0.0), Some(0.3), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
}
frame(lua_state, 1.0);
if is_excute(agent) {
      ATTACK(agent, 0, 0, Hash40::new("have"), 12.75, angle, 73, 0, 56, 4.5, 3.0, 0.0, 0.3, Some(0.0), Some(0.0), Some(0.3), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
}
frame(lua_state, 9.0);
if is_excute(agent) {
    AttackModule::clear_all(boma);
}
}
unsafe extern "C" fn dragon_sound_attackbeamloop(agent: &mut L2CAgentBase) {
let lua_state = agent.lua_state_agent;
let boma = agent.boma();
if is_excute(agent) {
    STOP_SE(agent, Hash40::new("se_tantan_attack01_short"));
    STOP_SE(agent, Hash40::new("se_tantan_attack01_long"));
}
frame(lua_state, 2.0);
if is_excute(agent) {
      let sfx = if WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_IS_DRAGONIZE) {Hash40::new("se_tantan_attack01_beam_max")} else {Hash40::new("se_tantan_attack01_beam")};
    PLAY_SE(agent, sfx);
}
}
pub fn install(agent: &mut Agent) {
agent.acmd("game_attackshort", dragon_game_attackshort);
agent.acmd("game_attacklong", dragon_game_attacklong);
agent.acmd("game_attackdragonshootlong",dragon_game_attackdragonshootlong,);
agent.acmd("effect_attackdragonshootlong",dragon_effect_attackdragonshootlong,);
agent.acmd("game_specialairhiattack", dragon_special_hi_attack_game);
agent.acmd("game_specialairhiattackdragon",dragon_special_hi_attack_game,);
agent.acmd("sound_attackbeamloop", dragon_sound_attackbeamloop);
}
