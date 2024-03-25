use super::*;

unsafe extern "C" fn game_fall(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
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
    let owner_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let pickel = utils::util::get_battle_object_from_id(owner_id);
    let fall_distance = VarModule::get_float(pickel, vars::pickel::instance::FORGE_START_Y_POS) - PostureModule::pos_y(boma);
    if is_excute(agent) {
        if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_PICKEL {
            let pickel = utils::util::get_battle_object_from_id(owner_id);
            let pickel_boma = &mut *(*pickel).module_accessor;
            if pickel_boma.is_motion_one_of(&[Hash40::new("attack_air_lw"),
            Hash40::new("attack_air_lw_2"),
            Hash40::new("attack_air_lw_fall"),]){
                //below hitbox shows for 1 frame if this isnt here lol
           } else {
                wait(lua_state, 2.0);
                ATTACK(agent, 0, 0, Hash40::new("top"), 5.0 + (fall_distance / 3.2) , 70, (80.0 - (fall_distance / 2.0)), 0, 62, 6.4, 0.0, 4.6, 0.0, Some(0.0), Some(4.8), Some(0.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
                ATTACK(agent, 1, 0, Hash40::new("top"), 5.0 + (fall_distance / 3.2) , 58, (80.0 - fall_distance), 0, 62, 6.4, 0.0, 4.6, 0.0, Some(0.0), Some(4.8), Some(0.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
                AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
            }
        } else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 5.0 + (fall_distance / 3.2) , 70, (80.0 - (fall_distance / 2.0)), 0, 62, 6.4, 0.0, 4.6, 0.0, Some(0.0), Some(4.8), Some(0.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 5.0 + (fall_distance / 3.2) , 58, (80.0 - (fall_distance / 2.0)), 0, 62, 6.4, 0.0, 4.6, 0.0, Some(0.0), Some(4.8), Some(0.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
            AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        }
    }
}

unsafe extern "C" fn game_fallattackride(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let pickel = utils::util::get_battle_object_from_id(owner_id);
    let pickel_boma = &mut *(*pickel).module_accessor;
    let fall_distance = VarModule::get_float(pickel, vars::pickel::instance::FORGE_START_Y_POS) - PostureModule::pos_y(boma);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0 + (fall_distance / 3.2), 70, (80.0 - (fall_distance / 2.0)), 0, 62, 6.4, 0.0, 4.6, 0.0, Some(0.0), Some(4.8), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0 + (fall_distance / 3.2), 58, (80.0 - (fall_distance / 2.0)), 0, 62, 6.4, 0.0, 4.6, 0.0, Some(0.0), Some(4.8), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
        AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_fall", game_fall);

    agent.acmd("game_fallattack", game_fallattack);

    agent.acmd("game_fallattackride", game_fallattackride);
}
