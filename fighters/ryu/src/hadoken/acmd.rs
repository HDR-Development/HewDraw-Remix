use super::*;

unsafe extern "C" fn game_movewms(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::enable_safe_pos(boma);
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((agent.get_int(*WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        VarModule::set_flag(
            agent.battle_object, 
            vars::shotos::instance::EX_SPECIAL_USED,
            VarModule::is_flag(owner_module_accessor.object(), vars::shotos::instance::EX_SPECIAL_USED)
        );
        VarModule::set_flag(
            agent.battle_object, 
            vars::shotos::instance::MAGIC_SERIES_CANCEL,
            VarModule::is_flag(owner_module_accessor.object(), vars::shotos::instance::MAGIC_SERIES_CANCEL)
        );

        let hitlag_mul = if VarModule::is_flag(agent.battle_object, vars::shotos::instance::MAGIC_SERIES_CANCEL) {
            1.4
        } else {
            1.0
        };
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 0, 16, 0, 38, 3.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4 * hitlag_mul, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 0, 16, 0, 38, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4 * hitlag_mul, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 60, 16, 0, 38, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4 * hitlag_mul, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        GroundModule::set_rhombus_offset(boma, &Vector2f{x: 0.0, y: 0.0});
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::EX_SPECIAL_USED) {
            let speed = KineticModule::get_sum_speed3f(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            KineticModule::mul_speed(boma, &Vector3f{x: 2.3 / speed.x.abs(), y: 0.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            agent.set_int(35, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
            agent.set_int(35, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        }
    }

    wait(lua_state, 6.0);
    if is_excute(agent) {
        let hitlag_mul = if VarModule::is_flag(agent.battle_object, vars::shotos::instance::MAGIC_SERIES_CANCEL) {
            1.4
        } else {
            1.0
        };
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 0, 16, 0, 38, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4 * hitlag_mul, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 0, 16, 0, 38, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4 * hitlag_mul, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 60, 16, 0, 38, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4 * hitlag_mul, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
    }
}

unsafe extern "C" fn effect_movewms(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if agent.is_flag(*WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
            EFFECT_FOLLOW(agent, Hash40::new("ryu_hadoken_bullet2"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.65, false);
        }
        else{
            EFFECT_FOLLOW(agent, Hash40::new("ryu_hadoken_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, false);
        }
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::MAGIC_SERIES_CANCEL) {
            // LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 1.0);
            EFFECT_FOLLOW(agent, Hash40::new("sys_thunder"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, false);
            EFFECT_FOLLOW(agent, Hash40::new("sys_thunder"), Hash40::new("top"), 0, 0, -0.5, 0, 0, 0, 1.25, false);
            EFFECT_FOLLOW(agent, Hash40::new("sys_thunder"), Hash40::new("top"), 0, 0, -1.0, 0, 0, 0, 1.5, false);
        }
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::EX_SPECIAL_USED) {
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("top"), 0, 0, -1.0, 0, 0, 0, 3.0, false);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_impact"), Hash40::new("top"), 0, 0, -1.0, 0, 0, 0, 0.5, false);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_hit2"), Hash40::new("top"), 0, 0, -1.0, 180, 0, 0, 1.0, false);
        }
    }
}

unsafe extern "C" fn game_movespwms(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::enable_safe_pos(boma);
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((agent.get_int(*WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        VarModule::set_flag(
            agent.battle_object, 
            vars::shotos::instance::EX_SPECIAL_USED,
            VarModule::is_flag(owner_module_accessor.object(), vars::shotos::instance::EX_SPECIAL_USED)
        );
        VarModule::set_flag(
            agent.battle_object, 
            vars::shotos::instance::MAGIC_SERIES_CANCEL,
            VarModule::is_flag(owner_module_accessor.object(), vars::shotos::instance::MAGIC_SERIES_CANCEL)
        );
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 366, 100, 45, 0, 3.5, 0.0, -5.2, 0.0, Some(0.0), Some(-5.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.5, 366, 100, 45, 0, 3.5, 0.0, 0.5, 0.0, Some(0.0), Some(-5.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);    
        AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
        GroundModule::set_rhombus_offset(boma, &Vector2f{x: 0.0, y: 0.0});
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::EX_SPECIAL_USED) {
            let speed = KineticModule::get_sum_speed3f(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            KineticModule::mul_speed(boma, &Vector3f{x: 0.5 / speed.x.abs(), y: 0.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            agent.set_int(160, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
            agent.set_int(160, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        }
    }

    wait(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 366, 100, 45, 0, 1.15, 0.0, -0.9, 0.95, Some(0.0), Some(-0.9), Some(-5.15), 0.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.5, 366, 100, 45, 0, 2.3, 0.0, 0.25, 0.3, Some(0.0), Some(0.25), Some(-4.5), 0.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 2, 0, Hash40::new("top"), 0.5, 366, 100, 45, 0, 1.4, 0.0, -2.4, -1.1, Some(0.0), Some(-2.4), Some(-2.9), 0.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 3, 0, Hash40::new("top"), 0.5, 366, 100, 45, 0, 2.8, 0.0, 0.0, -2.0, Some(0.0), Some(-1.0), Some(-2.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        AttackModule::set_add_reaction_frame(boma, 0, 4.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 4.0, false);
        AttackModule::set_add_reaction_frame(boma, 2, 4.0, false);
        AttackModule::set_add_reaction_frame(boma, 3, 4.0, false);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
        AttackModule::set_no_finish_camera(boma, 2, true, false);
        AttackModule::set_no_finish_camera(boma, 3, true, false);
    }
}

unsafe extern "C" fn effect_movespwms(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ryu_syakunetsu_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.65, false);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::EX_SPECIAL_USED) {
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("top"), 0, 0, -1.0, 0, 0, 0, 3.0, false);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_impact"), Hash40::new("top"), 0, 0, -1.0, 0, 0, 0, 0.5, false);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_hit2"), Hash40::new("top"), 0, 0, -1.0, 180, 0, 0, 1.0, false);
        }
    }
}

unsafe extern "C" fn game_movespwms_last(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.8, 60, 60, 0, 58, 5.0, 0.0, 0.0, -1.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
    }
}

unsafe extern "C" fn effect_movespwms_last(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {

    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_movew", game_movewms, Priority::Low);
    agent.acmd("game_movem", game_movewms, Priority::Low);
    agent.acmd("game_moves", game_movewms, Priority::Low);

    agent.acmd("effect_movew", effect_movewms, Priority::Low);
    agent.acmd("effect_movem", effect_movewms, Priority::Low);
    agent.acmd("effect_moves", effect_movewms, Priority::Low);

    agent.acmd("game_movespw", game_movespwms, Priority::Low);
    agent.acmd("game_movespm", game_movespwms, Priority::Low);
    agent.acmd("game_movesps", game_movespwms, Priority::Low);

    agent.acmd("effect_movespw", effect_movespwms, Priority::Low);
    agent.acmd("effect_movespm", effect_movespwms, Priority::Low);
    agent.acmd("effect_movesps", effect_movespwms, Priority::Low);

    agent.acmd("game_movespw_last", game_movespwms_last, Priority::Low);
    agent.acmd("game_movespm_last", game_movespwms_last, Priority::Low);
    agent.acmd("game_movesps_last", game_movespwms_last, Priority::Low);

    agent.acmd("effect_movespw_last", effect_movespwms_last, Priority::Low);
    agent.acmd("effect_movespm_last", effect_movespwms_last, Priority::Low);
    agent.acmd("effect_movesps_last", effect_movespwms_last, Priority::Low);
}
