
use super::*;


#[acmd_script( agent = "ryu", scripts = ["game_specialn", "game_specialairn"] , category = ACMD_GAME , low_priority)]
unsafe fn game_specialn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if boma.is_button_on(Buttons::Guard | Buttons::GuardHold)
        && !VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL)
        && !ArticleModule::is_exist(boma, *FIGHTER_RYU_GENERATE_ARTICLE_HADOKEN) {
            WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FAILED);
        } else {
            WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SHOOT);
            if fighter.kind() != *FIGHTER_KIND_KIRBY 
            && !VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                MeterModule::add(fighter.battle_object, 2.0);
            }
        }
    }
    frame(lua_state, 14.0);
    if WorkModule::is_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FAILED) {
        FT_MOTION_RATE_RANGE(fighter, 14.0, 58.0, 18.0);
    } else {
        FT_MOTION_RATE_RANGE(fighter, 14.0, 58.0, 36.0);
    }
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    frame(lua_state, 58.0);
    FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "ryu", scripts = ["effect_specialn", "effect_specialairn"] , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_specialn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if WorkModule::is_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 11, -7, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 4.0);
    if !WorkModule::is_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FAILED) {
        if is_excute(fighter) {
            if fighter.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_INT_TYPE) == 0 {
                EFFECT_FOLLOW(fighter, Hash40::new("ryu_hadoken_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
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
                EFFECT_FOLLOW(fighter, Hash40::new("ryu_hadoken_shot"), Hash40::new("top"), 0, 11.5, 10, 0, 0, 0, 1, true);
            } else {
                EFFECT_FOLLOW(fighter, Hash40::new("ryu_syakunetsu_shot"), Hash40::new("top"), 0, 11.5, 10, 0, 0, 0, 1, true);
            }
        }
        frame(lua_state, 12.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("ryu_hadoken_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
            if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
            }
        }
        for _ in 0..6 {
            wait(lua_state, 3.0);
            if is_excute(fighter)
            && VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
            }
            wait(lua_state, 3.0);
            if is_excute(fighter)
            && VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
            }
        }
        wait(lua_state, 3.0);
        if is_excute(fighter)
        && VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            COL_NORMAL(fighter);
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


#[acmd_script( agent = "ryu_hadoken", scripts = ["game_movew", "game_movem", "game_moves" ], category = ACMD_GAME, low_priority )]
unsafe fn game_movewms(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
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
        let attr = if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL) {
            Hash40::new("collision_attr_elec")
        } else {
            Hash40::new("collision_attr_normal")
        };
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 9.5, 0, 10, 0, 67, 3.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 9.5, 0, 10, 0, 67, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 9.5, 60, 10, 0, 66, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        } else {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 9.5, 0, 10, 0, 67, 3.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 9.5, 0, 10, 0, 67, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 9.5, 60, 10, 0, 66, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        }
        ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 1.13);
    }
    wait(lua_state, 6.0);
    if is_excute(fighter) {
        let attr = if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL) {
            Hash40::new("collision_attr_elec")
        } else {
            Hash40::new("collision_attr_normal")
        };
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 9.5, 0, 10, 0, 57, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 9.5, 0, 10, 0, 57, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 9.5, 60, 10, 0, 57, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        } else {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 9.5, 0, 10, 0, 57, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 9.5, 0, 10, 0, 57, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 9.5, 60, 10, 0, 57, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        }
        ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 1.13);
    }
}

#[acmd_script( agent = "ryu_hadoken", scripts = ["effect_movew", "effect_movem", "effect_moves"] , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_movemws(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_hadoken_bullet2"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.65, false);
        }
        else{
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_hadoken_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, false);
        }
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            // LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 1.0);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_thunder"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, false);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_thunder"), Hash40::new("top"), 0, 0, -0.5, 0, 0, 0, 1.5, false);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_thunder"), Hash40::new("top"), 0, 0, -1.0, 0, 0, 0, 2.0, false);
        }
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("top"), 0, 0, -1.0, 0, 0, 0, 3.0, false);
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_impact"), Hash40::new("top"), 0, 0, -1.0, 0, 0, 0, 0.5, false);
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_hit2"), Hash40::new("top"), 0, 0, -1.0, 180, 0, 0, 1.0, false);
        }
    }
}

#[acmd_script( agent = "ryu_hadoken", scripts = ["game_movespw", "game_movespm", "game_movesps" ], category = ACMD_GAME, low_priority )]
unsafe fn game_movespwms(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
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
        
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            // mul speed of the projectile without extending distance
            let mul = 1.75;
            KineticModule::mul_speed(fighter.module_accessor, &Vector3f{x: mul, y: 1.0, z: 1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            WorkModule::set_int(
                boma, 
                ((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE) as f32) / mul) as i32, 
                *WEAPON_INSTANCE_WORK_ID_INT_LIFE
            );
            ATTACK(fighter, 0, 0, Hash40::new("top"), 1.1, 80, 10, 0, 42, 3.5, 0.0, -5.2, 0.0, Some(0.0), Some(-5.2), Some(0.0), 0.15, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 1.1, 366, 10, 0, 40, 3.5, 0.0, 0.5, 0.0, Some(0.0), Some(-5.2), Some(0.0), 0.15, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY); 
        } else {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 1.1, 80, 10, 0, 42, 3.5, 0.0, -5.2, 0.0, Some(0.0), Some(-5.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 1.1, 366, 10, 0, 40, 3.5, 0.0, 0.5, 0.0, Some(0.0), Some(-5.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);    
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

#[acmd_script( agent = "ryu_hadoken", scripts = ["effect_movespw", "effect_movespm", "effect_movesps"], category = ACMD_EFFECT , low_priority)]
unsafe fn effect_movespwms(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "ryu_hadoken", scripts = ["game_movespw_last", "game_movespm_last", "game_movesps_last"], category = ACMD_GAME, low_priority )]
unsafe fn game_movespwms_last(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "ryu_hadoken", scripts = ["effect_movespw_last", "effect_movespm_last", "effect_movesps_last"], category = ACMD_EFFECT, low_priority )]
unsafe fn effect_movespwms_last(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_flame"), Hash40::new("top"), 1.0, 0, 0, 0, 0, 0, 0.4, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_bomb_a"), Hash40::new("top"), 1.0, 0, 0, 0, 0, 0, 0.3, true);
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        game_specialn,
        effect_specialn,

        game_movewms,
        effect_movemws,

        game_movespwms,
        effect_movespwms,

        game_movespwms_last,
        effect_movespwms_last
    );
}
