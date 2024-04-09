use super::*;

unsafe extern "C" fn game_movewms(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::enable_safe_pos(boma);
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((agent.get_int(*WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        VarModule::set_flag(
            agent.battle_object, 
            vars::shotos::instance::IS_USE_EX_SPECIAL,
            VarModule::is_flag(owner_module_accessor.object(), vars::shotos::instance::IS_USE_EX_SPECIAL)
        );
        VarModule::set_flag(
            agent.battle_object, 
            vars::shotos::instance::IS_MAGIC_SERIES_CANCEL,
            VarModule::is_flag(owner_module_accessor.object(), vars::shotos::instance::IS_MAGIC_SERIES_CANCEL)
        );
        if VarModule::is_flag(owner_module_accessor.object(), vars::shotos::instance::IS_USE_EX_SPECIAL) {
            if VarModule::get_int(owner_module_accessor.object(), vars::shotos::instance::SPECIAL_N_EX_NUM) <= 0 {
                MeterModule::drain_direct(owner_module_accessor.object(), 2.0 * MeterModule::meter_per_level(owner_module_accessor.object()));
                VarModule::set_int(owner_module_accessor.object(), vars::shotos::instance::SPECIAL_N_EX_NUM, 2);
            } else {
                VarModule::dec_int(owner_module_accessor.object(), vars::shotos::instance::SPECIAL_N_EX_NUM);
            }
        }
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            let speed = KineticModule::get_sum_speed3f(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            KineticModule::mul_speed(boma, &Vector3f{x: 2.016 / speed.x.abs(), y: 0.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            agent.set_int(41, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

            ATTACK(agent, 0, 0, Hash40::new("top"), 15.4, 0, 10, 0, 45, 3.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
            ATTACK(agent, 1, 0, Hash40::new("top"), 15.4, 0, 10, 0, 45, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
            ATTACK(agent, 2, 0, Hash40::new("top"), 15.4, 60, 10, 0, 65, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        } else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 0, 10, 0, 45, 3.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
            ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 0, 10, 0, 45, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
            ATTACK(agent, 2, 0, Hash40::new("top"), 6.0, 60, 10, 0, 65, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        }
        if (VarModule::is_flag(owner_module_accessor.object(), vars::shotos::instance::IS_CURRENT_HADOKEN_AIR)) {
            let lr = PostureModule::lr(owner_module_accessor);
            GroundModule::set_rhombus_offset(boma, &Vector2f{x: -4.0 * lr, y: 3.0});
            KineticModule::reflect_speed(boma, &Vector3f{x: 0.26, y: lr * 0.97, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            VarModule::off_flag(owner_module_accessor.object(), vars::shotos::instance::IS_CURRENT_HADOKEN_AIR);
        }
        else{
            GroundModule::set_rhombus_offset(boma, &Vector2f{x: 0.0, y: 0.0});
        }
        ATK_SET_SHIELD_SETOFF_MUL_arg4(agent, 0, 1, 2, 1.13);
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 15.4, 0, 10, 0, 45, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
            ATTACK(agent, 1, 0, Hash40::new("top"), 15.4, 0, 10, 0, 45, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
            ATTACK(agent, 2, 0, Hash40::new("top"), 15.4, 60, 10, 0, 65, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        } else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 0, 10, 0, 45, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
            ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 0, 10, 0, 45, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
            ATTACK(agent, 2, 0, Hash40::new("top"), 6.0, 60, 10, 0, 65, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        }
        ATK_SET_SHIELD_SETOFF_MUL_arg4(agent, 0, 1, 2, 1.13);
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
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL) {
            // LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 1.0);
            EFFECT_FOLLOW(agent, Hash40::new("sys_thunder"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, false);
            EFFECT_FOLLOW(agent, Hash40::new("sys_thunder"), Hash40::new("top"), 0, 0, -0.5, 0, 0, 0, 1.25, false);
            EFFECT_FOLLOW(agent, Hash40::new("sys_thunder"), Hash40::new("top"), 0, 0, -1.0, 0, 0, 0, 1.5, false);
        }
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
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
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((agent.get_int(*WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        VarModule::set_flag(
            agent.battle_object, 
            vars::shotos::instance::IS_USE_EX_SPECIAL,
            VarModule::is_flag(owner_module_accessor.object(), vars::shotos::instance::IS_USE_EX_SPECIAL)
        );
        VarModule::set_flag(
            agent.battle_object, 
            vars::shotos::instance::IS_MAGIC_SERIES_CANCEL,
            VarModule::is_flag(owner_module_accessor.object(), vars::shotos::instance::IS_MAGIC_SERIES_CANCEL)
        );
        if VarModule::is_flag(owner_module_accessor.object(), vars::shotos::instance::IS_USE_EX_SPECIAL) {
            MeterModule::drain_direct(owner_module_accessor.object(), 2.0 * MeterModule::meter_per_level(owner_module_accessor.object()))
        }
        
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            let speed = KineticModule::get_sum_speed3f(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            KineticModule::mul_speed(boma, &Vector3f{x: 2.1 / speed.x.abs(), y: 0.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            agent.set_int(38, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

            ATTACK(agent, 0, 0, Hash40::new("top"), 1.1, 80, 10, 0, 42, 3.5, 0.0, -5.2, 0.0, Some(0.0), Some(-5.2), Some(0.0), 0.15, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
            ATTACK(agent, 1, 0, Hash40::new("top"), 1.1, 366, 10, 0, 40, 3.5, 0.0, 0.5, 0.0, Some(0.0), Some(-5.2), Some(0.0), 0.15, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY); 
        } else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 1.1, 80, 10, 0, 42, 3.5, 0.0, -5.2, 0.0, Some(0.0), Some(-5.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
            ATTACK(agent, 1, 0, Hash40::new("top"), 1.1, 366, 10, 0, 40, 3.5, 0.0, 0.5, 0.0, Some(0.0), Some(-5.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);    
        }
        if (VarModule::is_flag(owner_module_accessor.object(), vars::shotos::instance::IS_CURRENT_HADOKEN_AIR)) {
            let lr = PostureModule::lr(owner_module_accessor);
            GroundModule::set_rhombus_offset(boma, &Vector2f{x: -4.0 * lr, y: 3.0});
            KineticModule::reflect_speed(boma, &Vector3f{x: 0.26, y: lr * 0.97, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            VarModule::off_flag(owner_module_accessor.object(), vars::shotos::instance::IS_CURRENT_HADOKEN_AIR);
        }
        AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
    }
    wait(lua_state, 9.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 1.1, 80, 10, 0, 42, 1.15, 0.0, -0.9, 0.95, Some(0.0), Some(-0.9), Some(-5.15), 0.15, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
            ATTACK(agent, 1, 0, Hash40::new("top"), 1.1, 366, 10, 0, 40, 2.3, 0.0, 0.25, 0.3, Some(0.0), Some(0.25), Some(-4.5), 0.15, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
            ATTACK(agent, 2, 0, Hash40::new("top"), 1.1, 80, 10, 0, 42, 1.4, 0.0, -2.4, -1.1, Some(0.0), Some(-2.4), Some(-2.9), 0.15, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
            ATTACK(agent, 3, 0, Hash40::new("top"), 1.1, 366, 10, 0, 40, 2.8, 0.0, 0.0, -2.0, Some(0.0), Some(-1.0), Some(-2.0), 0.15, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        } else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 1.1, 80, 10, 0, 42, 1.15, 0.0, -0.9, 0.95, Some(0.0), Some(-0.9), Some(-5.15), 0.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
            ATTACK(agent, 1, 0, Hash40::new("top"), 1.1, 366, 10, 0, 40, 2.3, 0.0, 0.25, 0.3, Some(0.0), Some(0.25), Some(-4.5), 0.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
            ATTACK(agent, 2, 0, Hash40::new("top"), 1.1, 80, 10, 0, 42, 1.4, 0.0, -2.4, -1.1, Some(0.0), Some(-2.4), Some(-2.9), 0.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
            ATTACK(agent, 3, 0, Hash40::new("top"), 1.1, 366, 10, 0, 40, 2.8, 0.0, 0.0, -2.0, Some(0.0), Some(-1.0), Some(-2.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        }
        AttackModule::set_add_reaction_frame(boma, 0, 4.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 4.0, false);
    }
}

unsafe extern "C" fn effect_movespwms(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            EFFECT_FOLLOW(agent, Hash40::new("ryu_syakunetsu_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.35, false);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("top"), 0, 0, -1.0, 0, 0, 0, 3.0, false);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_impact"), Hash40::new("top"), 0, 0, -1.0, 0, 0, 0, 0.5, false);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_hit2"), Hash40::new("top"), 0, 0, -1.0, 180, 0, 0, 1.0, false);
        } else {
            EFFECT_FOLLOW(agent, Hash40::new("ryu_syakunetsu_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.65, false);
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
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 14.1, 75, 79, 0, 65, 6.0, 0.0, 0.0, -1.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
        } else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 9.3, 55, 60, 0, 58, 5.0, 0.0, 0.0, -1.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        }
    }
}

unsafe extern "C" fn effect_movespwms_last(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_flame"), Hash40::new("top"), 1.0, 0, 0, 0, 0, 0, 0.4, true);
            EFFECT_FOLLOW(agent, Hash40::new("sys_bomb_a"), Hash40::new("top"), 1.0, 0, 0, 0, 0, 0, 0.3, true);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_movew", game_movewms);
    agent.acmd("game_movem", game_movewms);
    agent.acmd("game_moves", game_movewms);

    agent.acmd("effect_movew", effect_movewms);
    agent.acmd("effect_movem", effect_movewms);
    agent.acmd("effect_moves", effect_movewms);

    agent.acmd("game_movespw", game_movespwms);
    agent.acmd("game_movespm", game_movespwms);
    agent.acmd("game_movesps", game_movespwms);

    agent.acmd("effect_movespw", effect_movespwms);
    agent.acmd("effect_movespm", effect_movespwms);
    agent.acmd("effect_movesps", effect_movespwms);

    agent.acmd("game_movespw_last", game_movespwms_last);
    agent.acmd("game_movespm_last", game_movespwms_last);
    agent.acmd("game_movesps_last", game_movespwms_last);

    agent.acmd("effect_movespw_last", effect_movespwms_last);
    agent.acmd("effect_movespm_last", effect_movespwms_last);
    agent.acmd("effect_movesps_last", effect_movespwms_last);
}
