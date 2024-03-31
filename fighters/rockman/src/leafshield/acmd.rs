use super::*;

unsafe extern "C" fn game_start(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 4.0 / 3.0);
    if is_excute(agent) {
        if !WorkModule::is_flag(boma, 0x20000006) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_0
            ATTACK(agent, 0, 0, Hash40::new("leafshield1"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(boma, 0x20000007) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_1
            ATTACK(agent, 1, 0, Hash40::new("leafshield2"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(boma, 0x20000008) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_2
            ATTACK(agent, 2, 0, Hash40::new("leafshield3"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(boma, 0x20000009) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_3
            ATTACK(agent, 3, 0, Hash40::new("leafshield4"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
    }
}

unsafe extern "C" fn game_shield(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 4.0 / 3.0);
    if is_excute(agent) {
        if !WorkModule::is_flag(boma, 0x20000006) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_0
            ATTACK(agent, 0, 0, Hash40::new("leafshield1"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(boma, 0x20000007) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_1
            ATTACK(agent, 1, 0, Hash40::new("leafshield2"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(boma, 0x20000008) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_2
            ATTACK(agent, 2, 0, Hash40::new("leafshield3"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(boma, 0x20000009) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_3
            ATTACK(agent, 3, 0, Hash40::new("leafshield4"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
    }
}

unsafe extern "C" fn game_fly(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 4.0 / 3.0);
    if is_excute(agent) {
        if !WorkModule::is_flag(boma, 0x20000006) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_0
            ATTACK(agent, 0, 0, Hash40::new("leafshield1"), 3.8, 65, 70, 0, 50, 1.8, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 11, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(boma, 0x20000007) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_1
            ATTACK(agent, 1, 0, Hash40::new("leafshield2"), 3.8, 65, 70, 0, 50, 1.8, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 11, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(boma, 0x20000008) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_2
            ATTACK(agent, 2, 0, Hash40::new("leafshield3"), 3.8, 65, 70, 0, 50, 1.8, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 11, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(boma, 0x20000009) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_3
            ATTACK(agent, 3, 0, Hash40::new("leafshield4"), 3.8, 65, 70, 0, 50, 1.8, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 11, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_start", game_start);

    agent.acmd("game_startreverse", game_start);

    agent.acmd("game_shield", game_shield);

    agent.acmd("game_shieldreverse", game_shield);

    agent.acmd("game_fly", game_fly);

    agent.acmd("game_flyreverse", game_fly);
}
