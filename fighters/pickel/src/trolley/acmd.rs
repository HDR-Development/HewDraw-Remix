use super::*;

unsafe extern "C" fn game_specialsdriveemptypartial(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *WEAPON_PICKEL_TROLLEY_INSTANCE_WORK_ID_FLAG_NO_ATTACK_HIT_MOTION);
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 60, 64, 0, 66, 1.0, 0.0, 5.0, 6.0, Some(0.0), Some(0.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 60, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_ignore_ground_shield(boma, 0, true);
        ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 60, 64, 0, 66, 1.0, 0.0, 5.0, -6.0, Some(0.0), Some(0.0), Some(-6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 60, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_ignore_ground_shield(boma, 1, true);
        ATTACK(agent, 2, 0, Hash40::new("top"), 7.0, 60, 64, 0, 66, 1.0, 0.0, 0.0, 6.0, Some(0.0), Some(0.0), Some(-6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 60, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_ignore_ground_shield(boma, 2, true);
        ATTACK(agent, 3, 0, Hash40::new("top"), 7.0, 60, 64, 0, 66, 3.5, 0.0, 3.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 60, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_IIE, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 4, 0, Hash40::new("top"), 7.0, 60, 64, 0, 66, 3.5, 0.0, 3.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, f32::NAN, 0.0, 60, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_down_only(boma, 4, true);
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 5, 0, Hash40::new("top"), 7.0, 60, 64, 0, 66, 1.5, 0.0, 3.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, f32::NAN, 0.0, 60, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        WorkModule::off_flag(boma, *WEAPON_PICKEL_TROLLEY_INSTANCE_WORK_ID_FLAG_NO_ATTACK_HIT_MOTION);
    }
}

unsafe extern "C" fn game_pearlfly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 88, 100, 70, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
}

unsafe extern "C" fn effect_pearlfly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER);
        let owner = utils::util::get_battle_object_from_id(owner_id as u32);
        let owner_boma = &mut *(*owner).module_accessor;
        let lr = PostureModule::lr(owner_boma) as i32;
        EFFECT_FOLLOW(agent, Hash40::new("pickel_icon_ender_pearl"), Hash40::new("top"), 0, 0, -10 * lr, 0, 0, 0, 1.75, true);
    }
}

unsafe extern "C" fn sound_pearlfly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_pickel_throw_catch01"))
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialsdriveemptypartial", game_specialsdriveemptypartial, Priority::Low);

    agent.acmd("game_pearlfly", game_pearlfly, Priority::Low);
    agent.acmd("effect_pearlfly", effect_pearlfly, Priority::Low);
    agent.acmd("sound_pearlfly", sound_pearlfly, Priority::Low);
}
