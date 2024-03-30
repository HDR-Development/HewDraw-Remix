use super::*;

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    frame(lua_state, 61.0);
    if is_excute(agent) {
        if VarModule::get_int(owner_module_accessor.object(), vars::master::status::AYMR_CHARGE_LEVEL) > 0 {
            WorkModule::set_int(boma, 0, *WEAPON_MASTER_AXE_INSTANCE_WORK_ID_INT_CRITICAL_ATTACK_ID);
        }
        if VarModule::get_int(owner_module_accessor.object(), vars::master::status::AYMR_CHARGE_LEVEL) == 0 {
            ATTACK(agent, 0, 0, Hash40::new("haver"), 18.0, 51, 83, 0, 60, 5.7, 0.0, 14.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
        }
        else if VarModule::get_int(owner_module_accessor.object(), vars::master::status::AYMR_CHARGE_LEVEL) == 1 {
            // Ground-only
            ATTACK(agent, 0, 0, Hash40::new("haver"), 30.0, 51, 83, 0, 60, 5.7, 0.0, 14.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
            // Air-only
            ATTACK(agent, 1, 0, Hash40::new("haver"), 30.0, 275, 34, 0, 20, 5.7, 0.0, 14.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
        }
        else {
            // Ground-only
            ATTACK(agent, 0, 0, Hash40::new("haver"), 30.0, 270, 0, 0, 60, 5.7, 0.0, 14.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
            // Air-only
            ATTACK(agent, 1, 0, Hash40::new("haver"), 60.0, 275, 50, 0, 20, 5.7, 0.0, 14.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);

        }
    }
    frame(lua_state, 67.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_speciallwhit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if is_excute(agent) {
        if VarModule::get_int(owner_module_accessor.object(), vars::master::status::AYMR_CHARGE_LEVEL) == 2 {
            EFFECT_FOLLOW(agent, Hash40::new("master_axeflare_sp1"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1.0, true);
            EFFECT_FOLLOW(agent, Hash40::new("master_axeflare_sp2"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1.0, true);
            EFFECT_FOLLOW(agent, Hash40::new("master_axeflare_sp3"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1.0, true);
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("master_axe_aura"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        else {
            EFFECT_FOLLOW(agent, Hash40::new("master_axeflare_sp1"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("master_axeflare_sp2"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("master_axeflare_sp3"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("master_axe_aura"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("master_axe_slash_edge"), -1);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        if VarModule::get_int(owner_module_accessor.object(), vars::master::status::AYMR_CHARGE_LEVEL) == 2 {
            EFFECT_FOLLOW(agent, Hash40::new("master_axeflare_sp1_end"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1.0, true);
            EFFECT_FOLLOW(agent, Hash40::new("master_axeflare_sp2_end"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1.0, true);
            EFFECT_FOLLOW(agent, Hash40::new("master_axeflare_sp3_end"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1.0, true);
        }
        else {
            EFFECT_FOLLOW(agent, Hash40::new("master_axeflare_sp1_end"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("master_axeflare_sp2_end"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("master_axeflare_sp3_end"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare_sp1"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare_sp2"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_axeflare_sp3"), false, true);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axe_aura"), false, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_speciallw", game_speciallw);
    agent.acmd("game_specialairlw", game_speciallw);
    agent.acmd("effect_speciallwhit", effect_speciallwhit);
}
