use super::*;

unsafe extern "C" fn game_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let pickel_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let pickel = pickel_boma.object();
    if is_excute(agent) {
        AttackModule::disable_tip(boma);
        VarModule::off_flag(pickel, vars::pickel::instance::IS_CURRENT_ATTACK_LW3_SOUL_FIRE);
        if VarModule::is_flag(pickel, vars::common::instance::IS_HEAVY_ATTACK){
            VarModule::on_flag(pickel, vars::pickel::instance::IS_CURRENT_ATTACK_LW3_SOUL_FIRE);
        } else {
            VarModule::off_flag(pickel, vars::pickel::instance::IS_CURRENT_ATTACK_LW3_SOUL_FIRE);
        }
        if VarModule::is_flag(pickel, vars::pickel::instance::IS_CURRENT_ATTACK_LW3_SOUL_FIRE) {
            //FT_MOTION_RATE(agent, 1.25);
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.8, 366, 100, 40, 0, 3.2, 0.0, 2.8, 2.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 8, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.8, 366, 100, 40, 0, 3.2, 0.0, 2.8, -2.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 8, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        } else {
            FT_MOTION_RATE(agent, 0.75);
            ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 70, 60, 0, 75, 3.2, 0.0, 2.8, 2.0, None, None, None, 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 70, 60, 0, 75, 3.2, 0.0, 2.8, -2.0, None, None, None, 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        }
        AttackModule::enable_safe_pos(boma);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        if VarModule::is_flag(pickel, vars::pickel::instance::IS_CURRENT_ATTACK_LW3_SOUL_FIRE){
            FT_MOTION_RATE(agent, 1.0);
            ATTACK(agent, 0, 1, Hash40::new("top"), 6.4, 54, 116, 0, 42, 3.5, 0.0, 2.8, 2.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 1, Hash40::new("top"), 6.4, 54, 116, 0, 42, 3.5, 0.0, 2.8, -2.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        }
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}

unsafe extern "C" fn effect_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let pickel_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let pickel = pickel_boma.object();
    if is_excute(agent) {
        if VarModule::is_flag(pickel, vars::pickel::instance::IS_CURRENT_ATTACK_LW3_SOUL_FIRE){
            EFFECT_FOLLOW(agent, Hash40::new("pickel_fire_soot"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
            EFFECT_FOLLOW(agent, Hash40::new("pickel_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
            LAST_EFFECT_SET_COLOR(agent, 0.137, 0.85, 0.85);
        } else {
            EFFECT_FOLLOW(agent, Hash40::new("pickel_fire_soot"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
            EFFECT_FOLLOW(agent, Hash40::new("pickel_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
            LAST_EFFECT_SET_COLOR(agent, 1.0, 0.467, 0.0);
        }
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("pickel_fire_soot"), -1);
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("pickel_fire"), -1);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacklw3", game_attacklw3);
    agent.acmd("effect_attacklw3", effect_attacklw3);
}
