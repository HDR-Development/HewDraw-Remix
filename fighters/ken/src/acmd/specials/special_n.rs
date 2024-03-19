
use super::*;

unsafe extern "C" fn ken_special_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        boma.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    frame(lua_state, 12.0);
    // checking SITUATION_KIND_AIR so we don't get a ground hadouken on the 1 frame of landing
    // I could just rewrite the status script to prevent this but thats a lot.
    if is_excute(fighter) && !boma.is_prev_situation(*SITUATION_KIND_AIR) {
        VarModule::off_flag(fighter.battle_object, vars::shotos::instance::IS_CURRENT_HADOKEN_AIR);
        fighter.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SHOOT);
        if fighter.kind() != *FIGHTER_KIND_KIRBY 
        && !VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            MeterModule::add(fighter.battle_object, 2.0 * MeterModule::damage_gain_mul(fighter.battle_object));
        }
    }
    frame(lua_state, 14.0);
    if fighter.is_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FAILED) {
        FT_MOTION_RATE_RANGE(fighter, 14.0, 58.0, 18.0);
    } else if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
        FT_MOTION_RATE_RANGE(fighter, 14.0, 58.0, 26.0);
    } else {
        FT_MOTION_RATE_RANGE(fighter, 14.0, 58.0, 36.0);
    }
    if is_excute(fighter) {
        fighter.on_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        fighter.off_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        fighter.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    frame(lua_state, 58.0);
    FT_MOTION_RATE(fighter, 1.0);
}

unsafe extern "C" fn ken_special_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        boma.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, vars::shotos::instance::IS_CURRENT_HADOKEN_AIR);
        fighter.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SHOOT);
        if fighter.kind() != *FIGHTER_KIND_KIRBY 
        && !VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            MeterModule::add(fighter.battle_object, 0.7 * MeterModule::damage_gain_mul(fighter.battle_object));
        }
    }
    frame(lua_state, 15.0);
    if fighter.is_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FAILED) {
        FT_MOTION_RATE_RANGE(fighter, 15.0, 70.0, 18.0);
    } else if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
        FT_MOTION_RATE_RANGE(fighter, 15.0, 70.0, 26.0);
    } else {
        FT_MOTION_RATE_RANGE(fighter, 15.0, 70.0, 36.0);
    }
    if is_excute(fighter) {
        boma.on_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        boma.off_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        boma.off_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    frame(lua_state, 70.0);
    FT_MOTION_RATE(fighter, 1.0);
}

unsafe extern "C" fn sound_specialn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if boma.is_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FAILED) {
        wait(lua_state, 10.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_ken_special_n03"));
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("vc_ken_special_n02"));
        }
    } else{
        if !boma.is_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
            if is_excute(fighter) {
                PLAY_SE(fighter, Hash40::new("se_ken_special_n01"));
            }
            wait(lua_state, 10.0);
            if is_excute(fighter) {
                PLAY_SE(fighter, Hash40::new("se_ken_special_n03"));
            }
            wait(lua_state, 2.0);
            if is_excute(fighter) {
                PLAY_SE(fighter, Hash40::new("vc_ken_special_n01"));
            }
        } else {
            if is_excute(fighter) {
                PLAY_SE(fighter, Hash40::new("se_ken_command_success"));
                PLAY_SE(fighter, Hash40::new("se_ken_special_n01"));
            }
            wait(lua_state, 10.0);
            if is_excute(fighter) {
                PLAY_SE(fighter, Hash40::new("se_ken_special_n03"));
            }
            wait(lua_state, 2.0);
            if is_excute(fighter) {
                PLAY_SE(fighter, Hash40::new("vc_ken_special_n01_command"));
            }
        }
    }
}

unsafe extern "C" fn effect_specialn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if boma.is_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 11, -6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 4.0);
    if !boma.is_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FAILED) {
        if is_excute(fighter) {
            if fighter.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_INT_TYPE) == 0 {
                EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
            } else {
                EFFECT_FOLLOW(fighter, Hash40::new("ryu_syakunetsu_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
            }
            if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
            }
        }
        frame(lua_state, 6.0);
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
            }
        }
        frame(lua_state, 8.0);
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
            } else {
                FLASH(fighter, 0.392, 1, 1, 0.353);
            }
        }
        frame(lua_state, 10.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
            }
        }
        frame(lua_state, 11.0);
        if is_excute(fighter) {
            COL_NORMAL(fighter);
            if fighter.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_INT_TYPE) == 0 {
                EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_shot"), Hash40::new("top"), 0, 11.5, 14.5, 0, 0, 0, 1, true);
            } else {
                EFFECT_FOLLOW(fighter, Hash40::new("ryu_syakunetsu_shot"), Hash40::new("top"), 0, 11.5, 10, 0, 0, 0, 1, true);
            }
        }
        frame(lua_state, 12.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("ken_hadoken_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
            if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
            }
        }
        for _ in 0..6 {
            wait(lua_state, 3.0);
            if is_excute(fighter) {
                if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                    FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
                }
            }
            wait(lua_state, 3.0);
            if is_excute(fighter) {
                if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                    FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
                }
            }
        }
        wait(lua_state, 3.0);
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                COL_NORMAL(fighter);
            }
        }
    }
    else{
        frame(lua_state, 10.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 12, 0, -4, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(lua_state, 12.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_misfire"), Hash40::new("throw"), 0, 0, 0, 90, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

unsafe extern "C" fn effect_specialairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if boma.is_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    if !boma.is_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FAILED) {
        frame(lua_state, 4.0);
        if is_excute(fighter) {
            if fighter.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_INT_TYPE) == 0 {
                EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_hold"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, true);
            } else {
                EFFECT_FOLLOW(fighter, Hash40::new("ryu_syakunetsu_hold"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, true);
            }
            if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
            }
        }
        frame(lua_state, 6.0);
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
            }
        }
        frame(lua_state, 8.0);
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
            } else {
                FLASH(fighter, 0.392, 1, 1, 0.353);
            }
        }
        frame(lua_state, 10.0);
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
            }
        }
        frame(lua_state, 11.0);
        if is_excute(fighter) {
            COL_NORMAL(fighter);
            if fighter.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_INT_TYPE) == 0 {
                EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_shot"), Hash40::new("top"), 0, 6, 11, 30, 0, 0, 1, true);
            } else {
                EFFECT_FOLLOW(fighter, Hash40::new("ryu_syakunetsu_shot"), Hash40::new("top"), 0, 6, 11, 30, 0, 0, 1, true);
            }
        }
        frame(lua_state, 12.0);
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
            }
        }
        frame(lua_state, 14.0);
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
            }
        }
        frame(lua_state, 16.0);
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                COL_NORMAL(fighter);
            }
        }
    } else {
        frame(lua_state, 14.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_misfire"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

unsafe extern "C" fn game_movewms(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        AttackModule::enable_safe_pos(boma);
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((fighter.get_int(*WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        VarModule::set_flag(
            fighter.battle_object, 
            vars::shotos::instance::IS_USE_EX_SPECIAL,
            VarModule::is_flag(owner_module_accessor.object(), vars::shotos::instance::IS_USE_EX_SPECIAL)
        );
        VarModule::set_flag(
            fighter.battle_object, 
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
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            let speed = KineticModule::get_sum_speed3f(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            KineticModule::mul_speed(fighter.module_accessor, &Vector3f{x: 2.016 / speed.x.abs(), y: 0.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            fighter.set_int(41, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

            ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 0, 10, 0, 45, 3.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 13.0, 0, 10, 0, 45, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 13.0, 60, 10, 0, 65, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        } else {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 0, 10, 0, 45, 3.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 0, 10, 0, 45, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 60, 10, 0, 65, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
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
        ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 1.13);
    }
    wait(lua_state, 6.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 0, 10, 0, 45, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 13.0, 0, 10, 0, 45, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 13.0, 60, 10, 0, 65, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        } else {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 0, 10, 0, 45, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 0, 10, 0, 45, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 60, 10, 0, 65, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        }
        ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 1.13);
    }
}

unsafe extern "C" fn effect_movewms(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if !boma.is_flag(*WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, false);
        } else {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullet2"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, false);
        }
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("ken_hadoken_bullethand_l"), Hash40::new("ken_hadoken_bullethand_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, true, *EF_FLIP_YZ);
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("ken_hadoken_bullethand_l"), Hash40::new("ken_hadoken_bullethand_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, true, *EF_FLIP_YZ);
            EFFECT_FOLLOW(fighter, Hash40::new("ken_savingattack_aura"), Hash40::new("top"), 0, -5.0, -1.0, 0, 0, 0, 3.0, false);
            EFFECT_FOLLOW(fighter, Hash40::new("ken_savingattack_impact"), Hash40::new("top"), 0, 0, -1.0, 0, 0, 0, 0.5, false);
            EFFECT_FOLLOW(fighter, Hash40::new("ken_savingattack_hit2"), Hash40::new("top"), 0, 0, -1.0, 180, 0, 0, 1.0, false);
        }
    }
}

unsafe extern "C" fn sound_movewms(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ken_special_n04"));
    }
}

unsafe extern "C" fn game_movespwms(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((fighter.get_int(*WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        VarModule::set_flag(
            fighter.battle_object, 
            vars::shotos::instance::IS_USE_EX_SPECIAL,
            VarModule::is_flag(owner_module_accessor.object(), vars::shotos::instance::IS_USE_EX_SPECIAL)
        );
        VarModule::set_flag(
            fighter.battle_object, 
            vars::shotos::instance::IS_MAGIC_SERIES_CANCEL,
            VarModule::is_flag(owner_module_accessor.object(), vars::shotos::instance::IS_MAGIC_SERIES_CANCEL)
        );
        if VarModule::is_flag(owner_module_accessor.object(), vars::shotos::instance::IS_USE_EX_SPECIAL) {
            MeterModule::drain_direct(owner_module_accessor.object(), 2.0 * MeterModule::meter_per_level(owner_module_accessor.object()))
        }
        
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            let speed = KineticModule::get_sum_speed3f(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            KineticModule::mul_speed(fighter.module_accessor, &Vector3f{x: 2.1 / speed.x.abs(), y: 0.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            fighter.set_int(38, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

            ATTACK(fighter, 0, 0, Hash40::new("top"), 1.1, 80, 10, 0, 42, 3.5, 0.0, -5.2, 0.0, Some(0.0), Some(-5.2), Some(0.0), 0.15, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 1.1, 366, 10, 0, 40, 3.5, 0.0, 0.5, 0.0, Some(0.0), Some(-5.2), Some(0.0), 0.15, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY); 
        } else {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 1.1, 80, 10, 0, 42, 3.5, 0.0, -5.2, 0.0, Some(0.0), Some(-5.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 1.1, 366, 10, 0, 40, 3.5, 0.0, 0.5, 0.0, Some(0.0), Some(-5.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);    
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
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 1.1, 80, 10, 0, 42, 1.15, 0.0, -0.9, 0.95, Some(0.0), Some(-0.9), Some(-5.15), 0.15, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 1.1, 366, 10, 0, 40, 2.3, 0.0, 0.25, 0.3, Some(0.0), Some(0.25), Some(-4.5), 0.15, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 1.1, 80, 10, 0, 42, 1.4, 0.0, -2.4, -1.1, Some(0.0), Some(-2.4), Some(-2.9), 0.15, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 3, 0, Hash40::new("top"), 1.1, 366, 10, 0, 40, 2.8, 0.0, 0.0, -2.0, Some(0.0), Some(-1.0), Some(-2.0), 0.15, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        } else {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 1.1, 80, 10, 0, 42, 1.15, 0.0, -0.9, 0.95, Some(0.0), Some(-0.9), Some(-5.15), 0.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 1.1, 366, 10, 0, 40, 2.3, 0.0, 0.25, 0.3, Some(0.0), Some(0.25), Some(-4.5), 0.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 1.1, 80, 10, 0, 42, 1.4, 0.0, -2.4, -1.1, Some(0.0), Some(-2.4), Some(-2.9), 0.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 3, 0, Hash40::new("top"), 1.1, 366, 10, 0, 40, 2.8, 0.0, 0.0, -2.0, Some(0.0), Some(-1.0), Some(-2.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        }
        AttackModule::set_add_reaction_frame(boma, 0, 4.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 4.0, false);
    }
}

unsafe extern "C" fn effect_movespwms(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_syakunetsu_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.35, false);
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("top"), 0, 0, -1.0, 0, 0, 0, 3.0, false);
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_impact"), Hash40::new("top"), 0, 0, -1.0, 0, 0, 0, 0.5, false);
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_hit2"), Hash40::new("top"), 0, 0, -1.0, 180, 0, 0, 1.0, false);
        } else {
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_syakunetsu_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.65, false);
        }
    }
}

unsafe extern "C" fn game_movespwms_last(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 14.1, 75, 79, 0, 65, 6.0, 0.0, 0.0, -1.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
        } else {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 9.3, 55, 60, 0, 58, 5.0, 0.0, 0.0, -1.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        }
    }
}

unsafe extern "C" fn effect_movespwms_last(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_flame"), Hash40::new("top"), 1.0, 0, 0, 0, 0, 0, 0.4, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_bomb_a"), Hash40::new("top"), 1.0, 0, 0, 0, 0, 0, 0.3, true);
        }
    }
}

unsafe extern "C" fn sound_movespwms(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ken_special_n04"));
    }
}

pub fn install() {
    smashline::Agent::new("ken")
        .acmd("game_specialn", ken_special_n_game)
        .acmd("game_specialairn", ken_special_air_n_game)
        .acmd("sound_specialn", sound_specialn)
        .acmd("sound_specialairn", sound_specialn)
        .acmd("effect_specialn", effect_specialn)
        .acmd("effect_specialairn", effect_specialairn)
        .install();
    smashline::Agent::new("ken_hadoken")
        .acmd("game_movew", game_movewms)
        .acmd("game_movem", game_movewms)
        .acmd("game_moves", game_movewms)
        .acmd("effect_movew", effect_movewms)
        .acmd("effect_movem", effect_movewms)
        .acmd("effect_moves", effect_movewms)
        .acmd("sound_movew", sound_movewms)
        .acmd("sound_movem", sound_movewms)
        .acmd("sound_moves", sound_movewms)
        .acmd("game_movespw", game_movespwms)
        .acmd("game_movespm", game_movespwms)
        .acmd("game_movesps", game_movespwms)
        .acmd("game_movespw_last", game_movespwms_last)
        .acmd("game_movespm_last", game_movespwms_last)
        .acmd("game_movesps_last", game_movespwms_last)
        .acmd("effect_movespw", effect_movespwms)
        .acmd("effect_movespm", effect_movespwms)
        .acmd("effect_movesps", effect_movespwms)
        .acmd("effect_movespw_last", effect_movespwms_last)
        .acmd("effect_movespm_last", effect_movespwms_last)
        .acmd("effect_movesps_last", effect_movespwms_last)
        .acmd("sound_movespw", sound_movespwms)
        .acmd("sound_movespm", sound_movespwms)
        .acmd("sound_movesps", sound_movespwms)
        .install();
}
