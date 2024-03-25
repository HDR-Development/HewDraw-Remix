use super::*;

unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
let lua_state = agent.lua_state_agent;
let boma = agent.boma();
if is_excute(agent) {
    VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_hide") as i64);
}
frame(lua_state, 14.0);
if is_excute(agent) {
    VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_show") as i64);
      WorkModule::on_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVELEG_INSTANCE_WORK_ID_FLAG_DISABLE_LINK_STOP);
}
frame(lua_state, 16.0);
if is_excute(agent) {
      // Ground-only
      ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 273, 10, 0, 150, 9.0, 0.0, 28.0, 16.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
      // Air-only 
      ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 270, 63, 0, 10, 9.0, 0.0, 28.0, 16.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    AttackModule::set_add_reaction_frame(boma, 0, 4.0, false);
}
wait(lua_state, 1.0);
if is_excute(agent) {
      // Ground-only
      ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 273, 10, 0, 150, 12.0, 0.0, 8.0, 18.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
      // Air-only
      ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 270, 63, 0, 10, 12.0, 0.0, 8.0, 18.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
      WorkModule::on_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVELEG_INSTANCE_WORK_ID_FLAG_CANCEL_EFFECT);
    AttackModule::set_add_reaction_frame(boma, 0, 4.0, false);
}
wait(lua_state, 5.0);
if is_excute(agent) {
    AttackModule::clear_all(boma);
}
frame(lua_state, 41.0);
if is_excute(agent) {
      WorkModule::off_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVELEG_INSTANCE_WORK_ID_FLAG_CANCEL_EFFECT);
}
}
unsafe extern "C" fn escape_air_game(agent: &mut L2CAgentBase) {
let lua_state = agent.lua_state_agent;
let boma = agent.boma();
let escape_air_cancel_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_cancel_frame"));

frame(lua_state, 4.0);
if is_excute(agent) {
    notify_event_msc_cmd!(agent, Hash40::new_raw(0x2ea0f68425), true);
}
frame(lua_state, 8.0);
if is_excute(agent) {
    notify_event_msc_cmd!(agent, Hash40::new_raw(0x2ea0f68425), false);
}
frame(lua_state, 29.0);
if is_excute(agent) {
      KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
}
frame(lua_state, escape_air_cancel_frame);
if is_excute(agent) {
      notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
}
}
pub fn install(agent: &mut Agent) {
agent.acmd("game_attacklw4", game_attacklw4);
}
