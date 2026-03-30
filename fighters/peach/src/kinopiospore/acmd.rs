use super::*;

unsafe extern "C" fn game_shot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let peach = utils::util::get_battle_object_from_id(owner_id);
        let peach_boma: &mut BattleObjectModuleAccessor = &mut *(*peach).module_accessor;
        if peach_boma.kind() != *FIGHTER_KIND_PEACH || VarModule::is_flag(peach_boma.object(), vars::peach::instance::SPECIAL_N_AUTOFIRE) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 48, 195, 0, 5, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 2, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        } else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 48, 195, 0, 40, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
            AttackModule::set_force_reaction(boma, 0, true, false);
        }
    }
}

unsafe extern "C" fn effect_shot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("peach_kinopio_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_shot", game_shot, Priority::Low);
    agent.acmd("effect_shot", effect_shot, Priority::Low);
}