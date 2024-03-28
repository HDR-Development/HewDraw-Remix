use super::*;

unsafe extern "C" fn game_fly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent,  0,  0,  Hash40::new("top"),  2.0,  75,  50,  0,  20,  1.0,  0.0,  0.0,  0.0,  None,  None,  None,  3.1,  1.0,  *ATTACK_SETOFF_KIND_OFF,  *ATTACK_LR_CHECK_F,  false,  -1,  0.0,  0,  true,  false,  false,  false,  false,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_L,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(boma);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *WEAPON_DUCKHUNT_CLAY_INSTANCE_WORK_ID_FLAG_IS_ADD_ACCEL_Y);
    }
}

unsafe extern "C" fn game_hit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 0.0);
    if is_excute(agent) {
        ATTACK(agent,  0,  0,  Hash40::new("top"),  2.5,  60,  100,  20,  0,  6.0,  0.0,  0.0,  0.0,  None,  None,  None,  0.5,  0.5,  *ATTACK_SETOFF_KIND_OFF,  *ATTACK_LR_CHECK_F,  false,  -1.3,  0.0,  7,  true,  true,  false,  false,  false,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_L,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ATTACK(agent,  0,  0,  Hash40::new("top"),  2.5,  100,  100,  20,  0,  10.0,  0.0,  0.0,  0.0,  None,  None,  None,  0.5,  0.5,  *ATTACK_SETOFF_KIND_OFF,  *ATTACK_LR_CHECK_F,  false,  -1.3,  0.0,  7,  true,  true,  false,  false,  false,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_L,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent,  0,  0,  Hash40::new("top"),  3.0,  75,  70,  0,  82,  3.0,  0.0,  9.0,  -6.0,  Some(0.0),  Some(-9.0),  Some(6.0),  0.2,  0.5,  *ATTACK_SETOFF_KIND_OFF,  *ATTACK_LR_CHECK_F,  false,  -1.5,  0.0,  0,  true,  true,  false,  false,  false,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_L,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_OBJECT);
        ATTACK(agent,  1,  0,  Hash40::new("top"),  3.0,  75,  70,  0,  82,  3.0,  0.0,  0.5,  -11.0,  Some(0.0),  Some(-1.1),  Some(11.0),  0.2,  0.5,  *ATTACK_SETOFF_KIND_OFF,  *ATTACK_LR_CHECK_F,  false,  -1.5,  0.0,  0,  true,  true,  false,  false,  false,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_L,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_OBJECT);
        ATTACK(agent,  2,  0,  Hash40::new("top"),  3.0,  75,  70,  0,  82,  3.0,  0.0,  -8.0,  -8.0,  Some(0.0),  Some(8.0),  Some(8.0),  0.2,  0.5,  *ATTACK_SETOFF_KIND_OFF,  *ATTACK_LR_CHECK_F,  false,  -1.5,  0.0,  0,  true,  true,  false,  false,  false,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_L,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_OBJECT);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_fly", game_fly);
    agent.acmd("game_hit", game_hit);
}
