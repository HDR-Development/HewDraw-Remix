
use super::*;



unsafe extern "C" fn ryu_attack_s4_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
        VarModule::off_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
        VarModule::off_flag(fighter.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE);
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL) {
            VarModule::on_flag(fighter.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE);
            VarModule::off_flag(fighter.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            if MeterModule::drain(fighter.battle_object, 1) {
                 VarModule::on_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
            }
        }
        if !(VarModule::is_flag(fighter.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE) || VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL)) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        else{
            if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL){
                FT_MOTION_RATE(fighter, 7.0/(15.0-6.0));
            }
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL){
            if VarModule::is_flag(fighter.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE) {
                ATTACK(fighter, 0, 0, Hash40::new("kneel"), 12.0, 75, 10, 0, 140, 4.0, -1.5, -2.0, -1.5, Some(-1.5), Some(1.5), Some(-1.5), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
                ATTACK(fighter, 1, 0, Hash40::new("kneel"), 12.0, 75, 10, 0, 140, 4.0, -6.2, -2.0, -1.5, Some(-6.2), Some(1.5), Some(-1.5), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
                ATTACK(fighter, 2, 0, Hash40::new("kneel"), 12.0, 75, 10, 0, 140, 4.5, 4.3, -2.0, -1.5, Some(4.3), Some(1.5), Some(-1.5), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
                AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
                AttackModule::set_add_reaction_frame(boma, 1, 10.0, false);
                AttackModule::set_add_reaction_frame(boma, 2, 10.0, false);
            }
            else {
                ATTACK(fighter, 0, 0, Hash40::new("kneel"), 17.0, 80, 10, 0, 120, 3.7, -1.5, -1.0, -1.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
                ATTACK(fighter, 1, 0, Hash40::new("kneel"), 17.0, 80, 10, 0, 120, 3.7, -6.2, -1.0, -1.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
                ATTACK(fighter, 2, 0, Hash40::new("kneel"), 17.0, 80, 10, 0, 120, 4.0, 4.3, -1.7, -1.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
                AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
                AttackModule::set_add_reaction_frame(boma, 1, 10.0, false);
                AttackModule::set_add_reaction_frame(boma, 2, 10.0, false);
            }
        }
        else{
            MeterModule::watch_damage(fighter.battle_object, true);
            if VarModule::is_flag(fighter.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE) {
                ATTACK(fighter, 0, 0, Hash40::new("kneel"), 10.0, 361, 45, 0, 60, 4.0, -1.5, -2.0, -1.5, Some(-1.5), Some(1.5), Some(-1.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
                ATTACK(fighter, 1, 0, Hash40::new("kneel"), 10.0, 361, 45, 0, 60, 4.0, -6.2, -2.0, -1.5, Some(-6.2), Some(1.5), Some(-1.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
                ATTACK(fighter, 2, 0, Hash40::new("kneel"), 10.0, 361, 45, 0, 60, 4.5, 4.3, -2.0, -1.5, Some(4.3), Some(1.5), Some(-1.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
             }
             else {
                ATTACK(fighter, 0, 0, Hash40::new("kneel"), 14.0, 361, 118, 0, 26, 3.7, -1.5, -1.0, -1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
                ATTACK(fighter, 1, 0, Hash40::new("kneel"), 14.0, 361, 118, 0, 26, 3.7, -6.2, -1.0, -1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
                ATTACK(fighter, 2, 0, Hash40::new("kneel"), 16.0, 361, 118, 0, 26, 4.0, 4.3, -1.7, -1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
             }
        }
        
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(fighter.battle_object, false);
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE) && !VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL){
            FT_MOTION_RATE(fighter, 0.8);
        }
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL){
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                if VarModule::is_flag(fighter.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE){
                    FT_MOTION_RATE(fighter, 0.5);
                }
                else{
                    FT_MOTION_RATE(fighter, 0.3);
                }
            }
        }
    }
    frame(lua_state, 50.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE) && !VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL){
            FT_MOTION_RATE(fighter, 0.5);
        }
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL){
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                if VarModule::is_flag(fighter.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE){
                    FT_MOTION_RATE(fighter, 0.4);
                }
                else{
                    FT_MOTION_RATE(fighter, 0.5);
                }
            }
            else{
                FT_MOTION_RATE(fighter, 0.5);
            }
        }
    }
    
}


unsafe extern "C" fn ryu_attack_s4_s_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL){
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 1.4, true);
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("neck"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("kneel"), 4, 0, 0, 0, 0, 0, 1.1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("kneer"), 4, 0, 0, 0, 0, 0, 1.1, true);
            
            COL_PRI(fighter, 200);
            FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            COL_NORMAL(fighter);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL){
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("ryu_savingattack_line_r"), Hash40::new("ryu_savingattack_line_l"), Hash40::new("top"), -15.0, 10.0, 0, 15, 0, 180, 1.15, false, *EF_FLIP_NONE);
            LAST_EFFECT_SET_RATE(fighter, 1.25);

            COL_PRI(fighter, 200);
            FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -2, 10, 1, -12, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("ryu_attack_line"), Hash40::new("ryu_attack_line"), Hash40::new("top"), -2, 10, 1, -12, 0, 0, 0.7, true, *EF_FLIP_YZ);
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 12.5, 14, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 360, true, 0.5);
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            COL_NORMAL(fighter);
        }
    }
    wait(lua_state, 2.0);
    for _ in 0..4 {
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                COL_PRI(fighter, 200);
                FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
            }
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
            }
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL){
                FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
            }
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                COL_NORMAL(fighter);
            }
        }
        wait(lua_state, 2.0);
    }
}


unsafe extern "C" fn ryu_attack_hi4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
        VarModule::off_flag(fighter.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE);
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL) {
            VarModule::on_flag(fighter.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE);
            VarModule::off_flag(fighter.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL);
        }
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        MeterModule::watch_damage(fighter.battle_object, true);
        if VarModule::is_flag(fighter.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE) {
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.0, 77, 80, 0, 80, 4.5, 3.0, 0.0, 0.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 6.0, 77, 80, 0, 80, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("shoulderr"), 6.0, 77, 80, 0, 80, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 3, 0, Hash40::new("top"), 6.0, 77, 80, 0, 80, 5.0, 0.0, 7.0, 10.5, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(boma, 0, -2.0, false);
            AttackModule::set_add_reaction_frame(boma, 1, -2.0, false);
            AttackModule::set_add_reaction_frame(boma, 2, -2.0, false);
            AttackModule::set_add_reaction_frame(boma, 3, -2.0, false);
         }
         else {
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 12.0, 80, 105, 0, 50, 4.5, 3.0, 0.0, 0.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 12.0, 80, 105, 0, 50, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("shoulderr"), 12.0, 80, 105, 0, 50, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
            //ATTACK(fighter, 3, 0, Hash40::new("top"), 12.0, 80, 110, 0, 50, 5.3, 0.0, 9.0, 8.5, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(boma, 0, -2.0, false);
            AttackModule::set_add_reaction_frame(boma, 1, -2.0, false);
            AttackModule::set_add_reaction_frame(boma, 2, -2.0, false);
            AttackModule::set_add_reaction_frame(boma, 3, -2.0, false);
         }
        WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 3, false);
        MeterModule::watch_damage(fighter.battle_object, true);
        if VarModule::is_flag(fighter.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE) {
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 5.0, 77, 80, 0, 80, 4.5, 3.0, 0.0, 0.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 5.0, 77, 80, 0, 80, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("shoulderr"), 5.0, 77, 80, 0, 80, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(boma, 0, -2.0, false);
            AttackModule::set_add_reaction_frame(boma, 1, -2.0, false);
            AttackModule::set_add_reaction_frame(boma, 2, -2.0, false);
            AttackModule::set_add_reaction_frame(boma, 3, -2.0, false);
         }
         else {
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 10.0, 80, 110, 0, 50, 4.5, 3.0, 0.0, 0.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0, 80, 110, 0, 50, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("shoulderr"), 10.0, 80, 110, 0, 50, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(boma, 0, -2.0, false);
            AttackModule::set_add_reaction_frame(boma, 1, -2.0, false);
            AttackModule::set_add_reaction_frame(boma, 2, -2.0, false);
            AttackModule::set_add_reaction_frame(boma, 3, -2.0, false);
         }
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(fighter.battle_object, false);
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    wait(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
    }
    
}


unsafe extern "C" fn ryu_attack_lw4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
        VarModule::off_flag(fighter.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE);
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL) {
            VarModule::on_flag(fighter.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE);
            VarModule::off_flag(fighter.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, true);
        WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        if VarModule::is_flag(fighter.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE) {
            VarModule::off_flag(fighter.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE);
            ATTACK(fighter, 0, 0, Hash40::new("legr"), 9.0, 40, 50, 0, 45, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 1, 0, Hash40::new("kneer"), 9.0, 40, 50, 0, 45, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 2, 0, Hash40::new("kneer"), 9.0, 40, 50, 0, 45, 4.0, 5.5, 0.0, 0.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 3, 0, Hash40::new("legl"), 7.0, 40, 50, 0, 45, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 4, 0, Hash40::new("kneel"), 7.0, 40, 50, 0, 45, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
         }
         else {
            ATTACK(fighter, 0, 0, Hash40::new("legl"), 10.0, 44, 50, 0, 45, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 1, 0, Hash40::new("kneel"), 10.0, 44, 50, 0, 45, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 2, 0, Hash40::new("legr"), 10.0, 44, 50, 0, 45, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 3, 0, Hash40::new("kneer"), 12.0, 87, 50, 0, 60, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 4, 0, Hash40::new("kneer"), 12.0, 87, 50, 0, 60, 3.5, 5.5, 0.0, 0.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame(boma, 3, 7.0, false);
            AttackModule::set_add_reaction_frame(boma, 4, 7.0, false);
         }
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(fighter.battle_object, false);
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    wait(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
    }
    
}




pub fn install() {
    smashline::Agent::new("ryu")
        .acmd("game_attacks4", ryu_attack_s4_s_game)
        .acmd("effect_attacks4", ryu_attack_s4_s_effect)
        .acmd("game_attackhi4", ryu_attack_hi4_game)
        .acmd("game_attacklw4", ryu_attack_lw4_game)
        .install();
}
