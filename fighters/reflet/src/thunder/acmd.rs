use super::*;

unsafe extern "C" fn game_shoot2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
    ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"),0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 70, 35, 0, 50, 3.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_MAGIC);
    AttackModule::enable_safe_pos(boma);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
    ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 70, 35, 0, 50, 3.2, 0.0, -0.7, 0.0, Some(0.0), Some(0.7), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_MAGIC);
    }
}

unsafe extern "C" fn game_gigaspark(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
    ControlModule::set_rumble(boma, Hash40::new("rbkind_explosionm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    ATTACK(agent, 0, 0, Hash40::new("top"), 2.4, 110, 100, 20, 0, 6.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 0.2, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_MAGIC);
    ATTACK(agent, 1, 0, Hash40::new("top"), 2.4, 366, 100, 16, 0, 2.0, 0.0, 12.0, 12.0, Some(0.0), Some(-12.0), Some(-12.0), 0.2, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_MAGIC);
    ATTACK(agent, 2, 0, Hash40::new("top"), 2.4, 366, 100, 16, 0, 2.0, 0.0, 12.0, -12.0, Some(0.0), Some(-12.0), Some(12.0), 0.2, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_MAGIC);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
    ATTACK(agent, 0, 1, Hash40::new("top"), 3.0, 50, 166, 0, 75, 3.0, 0.0, 12.0, 12.0, Some(0.0), Some(-12.0), Some(-12.0), 0.6, 1.0,*ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_MAGIC);
    ATTACK(agent, 1, 1, Hash40::new("top"), 3.0, 50, 166, 0, 75, 3.0, 0.0, 12.0, -12.0, Some(0.0), Some(-12.0), Some(12.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_MAGIC);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
    AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_tron0(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(agent.boma(), *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if is_excute(agent) {
        let damage = match owner_module_accessor.kind() {
            0x38 => 1.5 + VarModule::get_float(owner_module_accessor.object(), vars::reflet::instance::THUNDER_CHARGE) / 2.0,
            0x6 => 1.5 + VarModule::get_float(owner_module_accessor.object(), vars::kirby::instance::THUNDER_CHARGE) / 2.0,
            _ => 4.0
        };
        ATTACK(agent, 0, 0, Hash40::new("top"), damage, 45, 145, 0, 75, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -0.5, 0.0, 4, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_MAGIC);
        AttackModule::set_no_finish_camera(agent.boma(), 0, true, false);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_shoot2", game_shoot2, Priority::Low);

    agent.acmd("game_gigaspark", game_gigaspark, Priority::Low);
    
    agent.acmd("game_tron0", game_tron0, Priority::Low);
}
