use super::*;

use vars::pickel_forge::instance::*;

unsafe extern "C" fn game_wait(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        if !boma.is_flag(*WEAPON_PICKEL_FORGE_INSTANCE_WORK_ID_FLAG_INIT_ATTACK) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 70, 78, 0, 58, 6.4, 0.0, 4.6, 0.0, Some(0.0), Some(4.8), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 5.0, 0.0, 5.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 10, true, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
            AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        }
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_77_kanatoko_fall"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_fall(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
        let pos_y = PostureModule::pos_y(boma);
        VarModule::set_float(boma.object(), START_Y_POS, pos_y);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, true);
        WorkModule::on_flag(boma, *WEAPON_PICKEL_FORGE_INSTANCE_WORK_ID_FLAG_UPDATE_ATTACK);
    }
}

unsafe extern "C" fn game_fallattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner = utils::util::get_battle_object_from_id(owner_id);
    let owner_boma = &mut *(*owner).module_accessor;
    if is_excute(agent) {
        if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_PICKEL 
        && !owner_boma.is_motion_one_of(&[
            Hash40::new("attack_air_lw"),
            Hash40::new("attack_air_lw_2"),
            Hash40::new("attack_air_lw_fall")
        ]) {
            wait(lua_state, 2.0);
        }
        let fall_distance = VarModule::get_float(boma.object(), START_Y_POS) - PostureModule::pos_y(boma);
        let dmg = 8.0 + (fall_distance / 5.0);
        let kbg = 100.0 - (fall_distance / 1.5);
        let sfx = if fall_distance >= 16.0 { *ATTACK_SOUND_LEVEL_L } else { *ATTACK_SOUND_LEVEL_M };
        ATTACK(agent, 0, 0, Hash40::new("top"), dmg, 70, kbg, 0, 62, 6.4, 0.0, 4.6, 0.0, Some(0.0), Some(4.8), Some(0.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), sfx, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), dmg, 58, kbg, 0, 62, 6.4, 0.0, 4.6, 0.0, Some(0.0), Some(4.8), Some(0.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), sfx, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
        AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
}

unsafe extern "C" fn game_fallattackride(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let fall_distance = VarModule::get_float(boma.object(), START_Y_POS) - PostureModule::pos_y(boma);
    let dmg = 12.0 + (fall_distance / 5.0);
    let kbg = 100.0 - (fall_distance / 1.5);
    let sfx = if fall_distance >= 16.0 { *ATTACK_SOUND_LEVEL_L } else { *ATTACK_SOUND_LEVEL_M };
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), dmg, 70, kbg, 0, 62, 6.4, 0.0, 4.6, 0.0, Some(0.0), Some(4.8), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), sfx, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), dmg, 58, kbg, 0, 62, 6.4, 0.0, 4.6, 0.0, Some(0.0), Some(4.8), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), sfx, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
        AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_wait", game_wait, Priority::Low);

    agent.acmd("game_fall", game_fall, Priority::Low);

    agent.acmd("game_fallattack", game_fallattack, Priority::Low);

    agent.acmd("game_fallattackride", game_fallattackride, Priority::Low);
}
