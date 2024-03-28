use super::*;

unsafe extern "C" fn game_shoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        let mut is_dragonized = false;
        let minmin_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        if sv_battle_object::is_active(minmin_id) {
            let minmin = utils::util::get_battle_object_from_id(minmin_id);
            let minmin_boma = &mut *(*minmin).module_accessor;
            is_dragonized = WorkModule::get_int(minmin_boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_ARM_L_BIG_FRAME) > 0;
        }
        let damage = if is_dragonized {9.0} else {6.0};
        let sfx_level = if is_dragonized {*ATTACK_SOUND_LEVEL_L} else {*ATTACK_SOUND_LEVEL_M};
        let range = if is_dragonized {30.0} else {25.0};
        let size = if is_dragonized {2.8} else {1.3};

        ATTACK(agent, 0, 0, Hash40::new("top"), damage, 361, 75, 0, 70, size, 0.0, 0.0, 2.0, Some(0.0), Some(0.0), Some(range), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), sfx_level, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        AttackModule::disable_tip(boma);
    }
}

unsafe extern "C" fn effect_beam(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        let mut is_dragonized = false;
        let minmin_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        if sv_battle_object::is_active(minmin_id) {
            let minmin = utils::util::get_battle_object_from_id(minmin_id);
            let minmin_boma = &mut *(*minmin).module_accessor;
            is_dragonized = WorkModule::get_int(minmin_boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_ARM_L_BIG_FRAME) > 0;
        }
        let effect = if is_dragonized {Hash40::new("tantan_dragon_beam2_body")} else {Hash40::new("tantan_dragon_beam1_body")};
        let offset = if is_dragonized {-1.0} else {-2.5};
        EFFECT_FOLLOW(agent, effect, Hash40::new("top"), 0, 0, offset, 0, 90, 180, 1, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_shoot", game_shoot);
    agent.acmd("game_bigshoot", game_shoot);
    agent.acmd("effect_beam", effect_beam);
    agent.acmd("effect_bigbeam", effect_beam);
}
