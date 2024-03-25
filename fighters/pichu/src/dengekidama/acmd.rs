use super::*;

unsafe extern "C" fn game_regular(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if !VarModule::is_flag(owner_module_accessor.object(), vars::pichu::instance::IS_CHARGE_ATTACK) {
        if is_excute(agent) {
            ModelModule::set_scale(boma, 1.0);
            ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 75, 50, 0, 35, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        }
    } else {
        if is_excute(agent) {
            ModelModule::set_scale(boma, 1.5);
            FT_MOTION_RATE(agent, 0.5);
            ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 75, 50, 0, 35, 6.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        }
    }
}

unsafe extern "C" fn effect_regular(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("pichu_dengeki"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, true);
        if VarModule::is_flag(owner_module_accessor.object(), vars::pichu::instance::IS_CHARGE_ATTACK) {
            LAST_EFFECT_SET_COLOR(agent, 0.8,1.0,0.2)
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_regular", game_regular);
    agent.acmd("effect_regular", effect_regular);
}
