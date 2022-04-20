
use super::*;

#[acmd_script( agent = "gaogaen", script = "game_specialn" , category = ACMD_GAME , low_priority)]
unsafe fn gaogaen_special_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        VarModule::set_int(boma.object(), vars::gaogaen::SPECIAL_N_STRENGTH_LEVEL, 0);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("top"), /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 7.0, /*X2*/ 0.0, /*Y2*/ 10.0, /*Z2*/ -7.0, /*Power*/ 0.0, /*Speed*/ 0.0, /*Max Damage*/ 80, false, /*Lifetime*/ 0.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
        //ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 367, 100, 40, 0, 4.2, 0.0, 11.0, 4.0, Some(0.0), Some(11.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        //ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 367, 100, 40, 0, 3.8, 0.0, 11.0, -2.0, Some(0.0), Some(11.0), Some(-12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        //ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 367, 100, 40, 0, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        
        ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 1.0, 367, 100, 40, 0, 4.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 14, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("shoulderr"), 1.0, 367, 100, 40, 0, 4.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 14, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        // Air-only
        ATTACK(fighter, 2, 0, Hash40::new("arml"), 1.0, 160, 100, 40, 0, 5.0, 3.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 14, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("armr"), 1.0, 160, 100, 40, 0, 5.0, 3.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 14, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        // Ground-only
        ATTACK(fighter, 4, 0, Hash40::new("arml"), 1.0, 125, 100, 40, 0, 5.0, 3.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 14, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 5, 0, Hash40::new("armr"), 1.0, 125, 100, 40, 0, 5.0, 3.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 14, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_START_ROTATION);
        WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_INVINCIBLE);
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_INVINCIBLE);
        HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_INVINCIBLE);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_INVINCIBLE);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_INVINCIBLE);
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
            WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_START_ROTATION);
            WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
            HIT_NODE(fighter, Hash40::new("body"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_INVINCIBLE);
        }
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
            HIT_RESET_ALL(fighter);
            AttackModule::set_size(boma, 0, 0.1);
            AttackModule::set_size(boma, 1, 0.1);
            AttackModule::set_size(boma, 2, 0.1);
            HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_INVINCIBLE);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if !(fighter.is_button_on(Buttons::Special) || fighter.is_button_on(Buttons::SpecialRaw)) {
            VarModule::set_int(boma.object(), vars::gaogaen::SPECIAL_N_STRENGTH_LEVEL, 1);
            FT_MOTION_RATE(fighter, 5.0/(53.0-14.0));
        }
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        if VarModule::get_int(boma.object(), vars::gaogaen::SPECIAL_N_STRENGTH_LEVEL) == 0 {
            if !(fighter.is_button_on(Buttons::Special) || fighter.is_button_on(Buttons::SpecialRaw)) {
                VarModule::set_int(boma.object(), vars::gaogaen::SPECIAL_N_STRENGTH_LEVEL, 2);
                FT_MOTION_RATE(fighter, 5.0/(53.0-28.0));
            }
            shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        }
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        if VarModule::get_int(boma.object(), vars::gaogaen::SPECIAL_N_STRENGTH_LEVEL) == 0 {
            if !(fighter.is_button_on(Buttons::Special) || fighter.is_button_on(Buttons::SpecialRaw)) {
                VarModule::set_int(boma.object(), vars::gaogaen::SPECIAL_N_STRENGTH_LEVEL, 3);
                FT_MOTION_RATE(fighter, 5.0/(53.0-42.0));
            }
        }
    }
    frame(lua_state, 53.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        if VarModule::get_int(boma.object(), vars::gaogaen::SPECIAL_N_STRENGTH_LEVEL) == 0 {
            VarModule::set_int(boma.object(), vars::gaogaen::SPECIAL_N_STRENGTH_LEVEL, 4);
        }
    }
    frame(lua_state, 56.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        if VarModule::get_int(boma.object(), vars::gaogaen::SPECIAL_N_STRENGTH_LEVEL) == 1 {
            ATTACK(fighter, 0, 1, Hash40::new("top"), 8.0, 72, 50, 0, 85, 6.0, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 1, Hash40::new("top"), 8.0, 72, 50, 0, 85, 7.0, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 1, Hash40::new("top"), 8.0, 72, 50, 0, 85, 4.0, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
        else{
            ATTACK(fighter, 0, 1, Hash40::new("top"), 10.0, 45, 80, 0, 70, 6.0, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 1, Hash40::new("top"), 10.0, 45, 80, 0, 70, 7.0, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 1, Hash40::new("top"), 10.0, 45, 80, 0, 70, 4.0, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
        
    }
    frame(lua_state, 58.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(boma);
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_END_ROTATION);
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
            HIT_RESET_ALL(fighter);
            AttackModule::clear_all(boma);
            WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_END_ROTATION);
        }
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT){
            if VarModule::get_int(boma.object(), vars::gaogaen::SPECIAL_N_STRENGTH_LEVEL) == 1 {
                FT_MOTION_RATE(fighter, 0.9);
            }
        }
    }
    frame(lua_state, 85.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_REQUEST_GRAVITY_DEFAULT);
    }
    
}

#[acmd_script( agent = "gaogaen", script = "game_specialairn" , category = ACMD_GAME , low_priority)]
unsafe fn gaogaen_special_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        VarModule::set_int(boma.object(), vars::gaogaen::SPECIAL_N_STRENGTH_LEVEL, 0);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("top"), /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 7.0, /*X2*/ 0.0, /*Y2*/ 10.0, /*Z2*/ -7.0, /*Power*/ 0.0, /*Speed*/ 0.0, /*Max Damage*/ 80, false, /*Lifetime*/ 0.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
        //ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 367, 100, 40, 0, 4.2, 0.0, 11.0, 4.0, Some(0.0), Some(11.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        //ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 367, 100, 40, 0, 3.8, 0.0, 11.0, -2.0, Some(0.0), Some(11.0), Some(-12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        //ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 367, 100, 40, 0, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        
        ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 1.0, 365, 100, 40, 0, 4.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 14, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("shoulderr"), 1.0, 365, 100, 40, 0, 4.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 14, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        // Air-only
        ATTACK(fighter, 2, 0, Hash40::new("arml"), 1.0, 367, 365, 40, 0, 5.0, 3.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 14, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("armr"), 1.0, 367, 365, 40, 0, 5.0, 3.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 14, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        // Ground-only
        ATTACK(fighter, 4, 0, Hash40::new("arml"), 1.0, 125, 100, 40, 0, 5.0, 3.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 14, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 5, 0, Hash40::new("armr"), 1.0, 125, 100, 40, 0, 5.0, 3.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 14, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_START_ROTATION);
        WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_INVINCIBLE);
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_INVINCIBLE);
        HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_INVINCIBLE);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_INVINCIBLE);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_INVINCIBLE);
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
            WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_START_ROTATION);
            WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
            HIT_NODE(fighter, Hash40::new("body"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_INVINCIBLE);
        }
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
            HIT_RESET_ALL(fighter);
            AttackModule::set_size(boma, 0, 0.1);
            AttackModule::set_size(boma, 1, 0.1);
            AttackModule::set_size(boma, 2, 0.1);
            HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_INVINCIBLE);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if !(fighter.is_button_on(Buttons::Special) || fighter.is_button_on(Buttons::SpecialRaw)) {
            VarModule::set_int(boma.object(), vars::gaogaen::SPECIAL_N_STRENGTH_LEVEL, 1);
            FT_MOTION_RATE(fighter, 5.0/(53.0-14.0));
        }
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        if VarModule::get_int(boma.object(), vars::gaogaen::SPECIAL_N_STRENGTH_LEVEL) == 0 {
            if !(fighter.is_button_on(Buttons::Special) || fighter.is_button_on(Buttons::SpecialRaw)) {
                VarModule::set_int(boma.object(), vars::gaogaen::SPECIAL_N_STRENGTH_LEVEL, 2);
                FT_MOTION_RATE(fighter, 5.0/(53.0-28.0));
            }
            shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        }
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        if VarModule::get_int(boma.object(), vars::gaogaen::SPECIAL_N_STRENGTH_LEVEL) == 0 {
            if !(fighter.is_button_on(Buttons::Special) || fighter.is_button_on(Buttons::SpecialRaw)) {
                VarModule::set_int(boma.object(), vars::gaogaen::SPECIAL_N_STRENGTH_LEVEL, 3);
                FT_MOTION_RATE(fighter, 5.0/(53.0-42.0));
            }
        }
    }
    frame(lua_state, 53.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        if VarModule::get_int(boma.object(), vars::gaogaen::SPECIAL_N_STRENGTH_LEVEL) == 0 {
            VarModule::set_int(boma.object(), vars::gaogaen::SPECIAL_N_STRENGTH_LEVEL, 4);
        }
    }
    frame(lua_state, 56.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        if VarModule::get_int(boma.object(), vars::gaogaen::SPECIAL_N_STRENGTH_LEVEL) == 1 {
            ATTACK(fighter, 0, 1, Hash40::new("top"), 8.0, 72, 50, 0, 85, 6.0, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 1, Hash40::new("top"), 8.0, 72, 50, 0, 85, 7.0, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 1, Hash40::new("top"), 8.0, 72, 50, 0, 85, 4.0, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
        else{
            ATTACK(fighter, 0, 1, Hash40::new("top"), 10.0, 45, 80, 0, 70, 6.0, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 1, Hash40::new("top"), 10.0, 45, 80, 0, 70, 7.0, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 1, Hash40::new("top"), 10.0, 45, 80, 0, 70, 4.0, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
        
    }
    frame(lua_state, 58.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(boma);
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_END_ROTATION);
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
            HIT_RESET_ALL(fighter);
            AttackModule::clear_all(boma);
            WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_END_ROTATION);
        }
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT){
            if VarModule::get_int(boma.object(), vars::gaogaen::SPECIAL_N_STRENGTH_LEVEL) == 1 {
                FT_MOTION_RATE(fighter, 0.9);
            }
        }
    }
    frame(lua_state, 85.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_REQUEST_GRAVITY_DEFAULT);
    }
    
}

#[acmd_script( agent = "gaogaen", script = "game_specialsstart" , category = ACMD_GAME , low_priority)]
unsafe fn gaogaen_special_s_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_MOVE_START);
        FT_MOTION_RATE(fighter, 0.8);

        VarModule::off_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_ALTERNATE_GRAB);
        VarModule::off_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_GROUND_GRAB);
        VarModule::off_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_AIR_GRAB);
        VarModule::off_flag(boma.object(), vars::gaogaen::IS_INPUT_SPECIAL_LW_LARIAT);
    }
    
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if ControlModule::get_stick_y(boma) < -0.5 {
            VarModule::on_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_ALTERNATE_GRAB);
            VarModule::on_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_GROUND_GRAB);
        }
        else if ControlModule::get_stick_y(boma) > 0.5 {
            VarModule::on_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_ALTERNATE_GRAB);
            VarModule::on_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_AIR_GRAB);
        }

        if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_ALTERNATE_GRAB){
            FT_MOTION_RATE(fighter, 6.0/(15.0-12.0));
        }
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        // Spawn the special windboxes if an alternate grab is detected
        if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_ALTERNATE_GRAB){
            // OTG Grab
            if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_GROUND_GRAB){
                ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 45, 100, 30, 0, 5.0, 0.0, 5.0, 2.0, Some(0.0), Some(5.0), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
                damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 9.99);
            }
            // Anti-air grab
            else if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_AIR_GRAB){
                ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 330, 100, 30, 0, 7.0, 0.0, 15.0, 0.0, Some(0.0), Some(15.0), Some(4.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
                HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_INVINCIBLE);
                HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
                HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
                HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
                HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
            }
        }
        // Otherwise produce the normal windbox
        else{
            // Original Windbox
            ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 350, 100, 30, 0, 5.0, 0.0, 7.0, 2.0, Some(0.0), Some(7.0), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_ALTERNATE_GRAB){
            // Regular grab windbox
            ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 350, 100, 30, 0, 5.0, 0.0, 7.0, 2.0, Some(0.0), Some(7.0), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        // Spawn the anti-air/OTG grab box if we've detected we hit the windbox
        if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_ALTERNATE_GRAB) {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                AttackModule::clear_all(boma);
                // OTG Grab
                if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_GROUND_GRAB){
                    CATCH(fighter, 0, Hash40::new("top"), 7.0, 0.0, 5.0, 2.0, Some(0.0), Some(5.0), Some(7.0), *FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED, *COLLISION_SITUATION_MASK_GA);
                }
                // Anti-air grab
                else if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_AIR_GRAB){
                    CATCH(fighter, 0, Hash40::new("top"), 7.0, 0.0, 15.0, 0.0, Some(0.0), Some(15.0), Some(7.0), *FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED, *COLLISION_SITUATION_MASK_GA);
                }
            }
        }
        // If the regular grab, then just clear the original windbox and spawn the grabbox
        else{
            // Clear original windbox
            AttackModule::clear_all(boma);
            CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 8.0, 2.0, Some(0.0), Some(8.0), Some(8.0), *FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED, *COLLISION_SITUATION_MASK_GA);
        }
    }
    wait(lua_state, 1.0);
    for _ in 0..12{
        // Loop the logic on frame 18 while the OTG/anti-air grabs should be able to proc...
        if is_excute(fighter) {
            // Spawn the anti-air/OTG grab box if we've detected we hit the windbox
            if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_ALTERNATE_GRAB) {
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                    AttackModule::clear_all(boma);
                    // OTG Grab
                    if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_GROUND_GRAB){
                        CATCH(fighter, 0, Hash40::new("top"), 7.0, 0.0, 5.0, 2.0, Some(0.0), Some(5.0), Some(7.0), *FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED, *COLLISION_SITUATION_MASK_GA);
                    }
                    // Anti-air grab
                    else if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_AIR_GRAB){
                        CATCH(fighter, 0, Hash40::new("top"), 7.0, 0.0, 15.0, 0.0, Some(0.0), Some(15.0), Some(7.0), *FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED, *COLLISION_SITUATION_MASK_GA);
                    }
                }
            }
        }
        wait(lua_state, 1.0);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_MOVE_END);
    }
    frame(lua_state, 44.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.8);
    }
    frame(lua_state, 67.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_WORK_ID_FLAG_AIR_CONTROL);
    }
    frame(lua_state, 74.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1);
    }
}

#[acmd_script( agent = "gaogaen", script = "effect_specialsstart" , category = ACMD_EFFECT , low_priority)]
unsafe fn gaogaen_special_s_start_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 9, 10, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        // OTG/Anti-air grab effects
        if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_ALTERNATE_GRAB) {
            // OTG grab
            if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_GROUND_GRAB){
                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("bust"), 0.0, 0, 0, 0, 0, 0, 0.25, true);
                LAST_EFFECT_SET_RATE(fighter, 0.5);
                
                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 1.0, 0, 0, 0, 0, 0, 0.3, true);
                LAST_EFFECT_SET_RATE(fighter, 0.5);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 7.0, 0, 0, 0, 0, 0, 0.35, true);
                LAST_EFFECT_SET_RATE(fighter, 0.5);

                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 1.0, 0, 0, 0, 0, 0, 0.3, true);
                LAST_EFFECT_SET_RATE(fighter, 0.5);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 7.0, 0, 0, 0, 0, 0, 0.35, true);
                LAST_EFFECT_SET_RATE(fighter, 0.5);

                EFFECT_FOLLOW(fighter, Hash40::new("gaogaen_belt_fire_appeal"), Hash40::new("feeler"), 0, 3, 0, 0, 0, 0, 0.7, true);
                LAST_EFFECT_SET_RATE(fighter, 1.1);
            }
            // Anti-air grab
            else if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_AIR_GRAB){
                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("bust"), 0.0, 0, 0, 0, 0, 0, 0.25, true);
                LAST_EFFECT_SET_RATE(fighter, 0.5);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("arml"), 1.0, 0, 0, 0, 0, 0, 0.3, true);
                LAST_EFFECT_SET_RATE(fighter, 0.5);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("arml"), 7.0, 0, 0, 0, 0, 0, 0.35, true);
                LAST_EFFECT_SET_RATE(fighter, 0.5);

                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("armr"), 1.0, 0, 0, 0, 0, 0, 0.3, true);
                LAST_EFFECT_SET_RATE(fighter, 0.5);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("armr"), 7.0, 0, 0, 0, 0, 0, 0.35, true);
                LAST_EFFECT_SET_RATE(fighter, 0.5);

                EFFECT_FOLLOW(fighter, Hash40::new("gaogaen_belt_fire_appeal"), Hash40::new("feeler"), 0, 3, 0, 0, 0, 0, 0.7, true);
                LAST_EFFECT_SET_RATE(fighter, 1.1);
            }
        }
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 18.0);
    for _ in 0..8{
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            // OTG/Anti-air grab effects
            if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_ALTERNATE_GRAB) {
                // OTG grab
                if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_GROUND_GRAB){
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("bust"), 0.0, 0, 0, 0, 0, 0, 0.25, true);
                    
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 1.0, 0, 0, 0, 0, 0, 0.3, true);
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 7.0, 0, 0, 0, 0, 0, 0.35, true);

                    EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 1.0, 0, 0, 0, 0, 0, 0.3, true);
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 7.0, 0, 0, 0, 0, 0, 0.35, true);
                }
                // Anti-air grab
                else if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_AIR_GRAB){
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("bust"), 0.0, 0, 0, 0, 0, 0, 0.3, true);

                    EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("arml"), 1.0, 0, 0, 0, 0, 0, 0.35, true);
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("arml"), 7.0, 0, 0, 0, 0, 0, 0.4, true);
        
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("armr"), 1.0, 0, 0, 0, 0, 0, 0.35, true);
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("armr"), 7.0, 0, 0, 0, 0, 0, 0.4, true);
                }
            }
        }
    wait(lua_state, 2.0);
    }
}

#[acmd_script( agent = "gaogaen", script = "game_specialslariat" , category = ACMD_GAME , low_priority)]
unsafe fn gaogaen_special_s_lariat_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    // Revenge's command dash attack
    if VarModule::is_flag(boma.object(), vars::gaogaen::IS_INPUT_SPECIAL_LW_LARIAT){
        frame(lua_state, 1.0);
        if is_excute(fighter) {
            KineticModule::add_speed(boma, &Vector3f::new(-1.0, 0.0, 0.0));
            FT_MOTION_RATE(fighter, 10.0/(4.0-1.0));
        }
        frame(lua_state, 3.0);
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, 3.0/(9.0-6.0));
        }
        frame(lua_state, 9.0);
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, 7.0/(20.0-9.0));
            ATTACK(fighter, 0, 0, Hash40::new("arml"), 10.0, 69, 50, 0, 90, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        }
        frame(lua_state, 20.0);
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, 15.0/(25.0-20.0));
            AttackModule::clear_all(boma);
            REVERSE_LR(fighter);
        }
        frame(lua_state, 25.0);
        if is_excute(fighter) {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                FT_MOTION_RATE(fighter, 1.0);
            }
            else{
                FT_MOTION_RATE(fighter, 1.5);
            }
        }
    }
    // Side special lariat
    else{
        if is_excute(fighter) {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 11.99);
            // OTG/Anti-air grab throw boxes
            if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_ALTERNATE_GRAB) {
                // OTG grab
                if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_GROUND_GRAB){
                    ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 15, 50, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
                }
                // Anti-air grab
                else if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_AIR_GRAB){
                    ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 77, 100, 0, 95, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
                }
            }
            // Regular grab
            else{
                ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 145, 474, 0, 20, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            }
            
            AttackModule::set_force_reaction(boma, 0, true, true);
            WorkModule::set_float(boma, 9.0, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLOAT_LARIAT_HIT_FRAME);
        }
        frame(lua_state, 9.0);
        if is_excute(fighter) {
            // OTG/Anti-air grab hitboxes
            if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_ALTERNATE_GRAB) {
                // OTG grab
                if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_GROUND_GRAB){
                    ATTACK(fighter, 0, 0, Hash40::new("arml"), 15.0, 15, 10, 0, 85, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
                }
                // Anti-air grab
                else if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_AIR_GRAB){
                    ATTACK(fighter, 0, 0, Hash40::new("arml"), 15.0, 105, 40, 0, 88, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
                }
            }
            // Regular grab
            else{
                ATTACK(fighter, 0, 0, Hash40::new("arml"), 20.0, 145, 40, 0, 88, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            }
            //CHECK_FINISH_CAMERA(fighter, 0, 0);
            //FighterCutInManager::set_throw_finish_zoom_rate(boma, 1.5);
            //FighterCutInManager::set_throw_finish_offset(boma, 0, 0, 0);
        }
        frame(lua_state, 14.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_DAMAGE_CUT);
        }
        frame(lua_state, 15.0);
        if is_excute(fighter) {
            if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_GROUND_GRAB){
                ATTACK(fighter, 1, 1, Hash40::new("top"), 0.0, 180, 100, 10, 0, 15.0, 0.0, 15.0, -5.0, Some(0.0), Some(15.0), Some(-15.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
            }
        }
        frame(lua_state, 20.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_GROUND_GRAB){
                //ATTACK(fighter, 1, 1, Hash40::new("top"), 0.0, 0, 100, 900, 0, 10.0, 0.0, 10.0, 10.0, Some(0.0), Some(10.0), Some(15.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 1, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
            }
            REVERSE_LR(fighter);
        }
        frame(lua_state, 50.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_ENABLE_GRAVITY);
        }
        frame(lua_state, 58.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_WORK_ID_FLAG_AIR_CONTROL);
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        }
    }
}

#[acmd_script( agent = "gaogaen", script = "game_specialairslariat" , category = ACMD_GAME , low_priority)]
unsafe fn gaogaen_special_air_s_lariat_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 11.99);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 145, 474, 0, 20, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        AttackModule::set_force_reaction(boma, 0, true, true);
        WorkModule::set_float(boma, 9.0, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLOAT_LARIAT_HIT_FRAME);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("arml"), 20.0, 145, 40, 0, 88, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        //CHECK_FINISH_CAMERA(fighter, 0, 0);
        //FighterCutInManager::set_throw_finish_zoom_rate(boma, 1.5);
        //FighterCutInManager::set_throw_finish_offset(boma, 0, 0, 0);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_DAMAGE_CUT);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        REVERSE_LR(fighter);
    }
    frame(lua_state, 50.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_ENABLE_GRAVITY);
    }
    frame(lua_state, 58.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_WORK_ID_FLAG_AIR_CONTROL);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
}

#[acmd_script( agent = "gaogaen", script = "game_specialsshoulder" , category = ACMD_GAME , low_priority)]
unsafe fn gaogaen_special_s_shoulder_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 11.99);
        if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_ALTERNATE_GRAB) {
            // OTG grab
            if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_GROUND_GRAB){
                ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 120, 150, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            }
            // Anti-air grab
            else if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_AIR_GRAB){
                ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 105, 280, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            }
        }
        // Regular grab
        else{
            ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 100, 100, 0, 72, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        AttackModule::set_force_reaction(boma, 0, true, true);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.75);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        // OTG/Anti-air grab hitboxes
        if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_ALTERNATE_GRAB) {
            // OTG grab
            if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_GROUND_GRAB){
                ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 105, 34, 0, 128, 6.0, 0.0, 6.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 1, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
            }
            // Anti-air grab
            else if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_S_AIR_GRAB){
                ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 105, 34, 0, 128, 6.0, 0.0, 6.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 1, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
            }
        }
        // Regular grab
        else{
            ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 105, 34, 0, 128, 6.0, 0.0, 6.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 1, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        //CHECK_FINISH_CAMERA_IF_NOT_HP_MODE(fighter, 0, 0);
        //FighterCutInManager::set_throw_finish_zoom_rate(boma, 1.5);
        //FighterCutInManager::set_throw_finish_offset(boma, 0, 0, 0);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_DAMAGE_CUT);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        REVERSE_LR(fighter);
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_ENABLE_GRAVITY);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 52.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_WORK_ID_FLAG_AIR_CONTROL);
    }
}

#[acmd_script( agent = "gaogaen", script = "game_specialairsshoulder" , category = ACMD_GAME , low_priority)]
unsafe fn gaogaen_special_air_s_shoulder_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 11.99);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 100, 100, 0, 72, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        AttackModule::set_force_reaction(boma, 0, true, true);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.75);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 105, 34, 0, 128, 6.0, 0.0, 6.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 1, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        //CHECK_FINISH_CAMERA_IF_NOT_HP_MODE(fighter, 0, 0);
        //FighterCutInManager::set_throw_finish_zoom_rate(boma, 1.5);
        //FighterCutInManager::set_throw_finish_offset(boma, 0, 0, 0);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_DAMAGE_CUT);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        REVERSE_LR(fighter);
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_ENABLE_GRAVITY);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 52.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_WORK_ID_FLAG_AIR_CONTROL);
    }
}

#[acmd_script( agent = "gaogaen", script = "game_specialhistart" , category = ACMD_GAME , low_priority)]
unsafe fn gaogaen_special_hi_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        VarModule::off_flag(boma.object(), vars::gaogaen::IS_HIT_SPECIAL_HI_RISE);
        VarModule::off_flag(boma.object(), vars::gaogaen::IS_INPUT_CROSS_CHOP_CANCEL);
        VarModule::off_flag(boma.object(), vars::gaogaen::SHOULD_CROSS_CHOP_DIVE_EARLY);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 11.99);
        FT_MOTION_RATE(fighter, 6.0/(6.0-4.0));
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        SA_SET(fighter, *SITUATION_KIND_AIR);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 81, 1, 0, 82, 7.0, 0.0, 9.0, 2.0, None, None, None, 1.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 87, 1, 0, 81, 7.0, 0.0, 9.0, 2.0, Some(0.0), Some(9.0), Some(6.0), 1.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 80, 1, 0, 80, 4.5, 0.0, 11.5, 2.0, Some(0.0), Some(11.5), Some(3.5), 1.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 5.0, 78, 1, 0, 90, 2.5, 0.0, 4.5, 0.0, Some(0.0), Some(4.5), Some(5.5), 1.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 4.0, 90, 1, 0, 91, 7.0, 0.0, 9.0, 2.5, Some(0.0), Some(9.0), Some(6.0), 1.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_force_reaction(boma, 0, true, false);
        AttackModule::set_force_reaction(boma, 1, true, false);
        AttackModule::set_force_reaction(boma, 2, true, false);
        AttackModule::set_force_reaction(boma, 3, true, false);
        AttackModule::set_force_reaction(boma, 4, true, false);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        FT_MOTION_RATE(fighter, 0.9);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        AttackModule::clear(boma, 2, false);
        AttackModule::clear(boma, 3, false);
        AttackModule::clear(boma, 4, false);
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 4.0, 66, 5, 0, 58, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 4.0, 66, 5, 0, 58, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 4.0, 45, 5, 0, 49, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 4.0, 45, 5, 0, 49, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 2.5, 30, 5, 0, 34, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 2.5, 30, 5, 0, 34, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 2.5, 5, 5, 0, 30, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 2.5, 5, 5, 0, 30, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::gaogaen::IS_INPUT_CROSS_CHOP_CANCEL){
            FT_MOTION_RATE(fighter, 1.5);
        }
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "gaogaen", script = "game_specialairhistart" , category = ACMD_GAME , low_priority)]
unsafe fn gaogaen_special_air_hi_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        VarModule::off_flag(boma.object(), vars::gaogaen::IS_HIT_SPECIAL_HI_RISE);
        VarModule::off_flag(boma.object(), vars::gaogaen::IS_INPUT_CROSS_CHOP_CANCEL);
        VarModule::off_flag(boma.object(), vars::gaogaen::SHOULD_CROSS_CHOP_DIVE_EARLY);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 11.99);
        FT_MOTION_RATE(fighter, 4.0/(6.0-4.0));
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        SA_SET(fighter, *SITUATION_KIND_AIR);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        if fighter.is_button_on(Buttons::Special) || fighter.is_button_on(Buttons::SpecialRaw) {
            VarModule::on_flag(boma.object(), vars::gaogaen::SHOULD_CROSS_CHOP_DIVE_EARLY);
            WorkModule::off_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        }
        else{
            WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        }
        
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::gaogaen::SHOULD_CROSS_CHOP_DIVE_EARLY){
            FT_MOTION_RATE(fighter, 3.0/(15.0-11.0));
            KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 81, 1, 0, 82, 7.0, 0.0, 9.0, 2.0, None, None, None, 1.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 87, 1, 0, 81, 7.0, 0.0, 9.0, 2.0, Some(0.0), Some(9.0), Some(6.0), 1.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 80, 1, 0, 80, 4.5, 0.0, 11.5, 2.0, Some(0.0), Some(11.5), Some(3.5), 1.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 3, 0, Hash40::new("top"), 5.0, 78, 1, 0, 90, 2.5, 0.0, 4.5, 0.0, Some(0.0), Some(4.5), Some(5.5), 1.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 4, 0, Hash40::new("top"), 4.0, 90, 1, 0, 91, 7.0, 0.0, 9.0, 2.5, Some(0.0), Some(9.0), Some(6.0), 1.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        if VarModule::is_flag(boma.object(), vars::gaogaen::SHOULD_CROSS_CHOP_DIVE_EARLY){
            KineticModule::clear_speed_all(boma);
        }
        else{
            HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
            HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
            FT_MOTION_RATE(fighter, 0.9);
            AttackModule::clear(boma, 2, false);
            AttackModule::clear(boma, 3, false);
            AttackModule::clear(boma, 4, false);
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 4.0, 66, 5, 0, 58, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("arml"), 4.0, 66, 5, 0, 58, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::gaogaen::SHOULD_CROSS_CHOP_DIVE_EARLY){
            FT_MOTION_RATE(fighter, 2.0/(24.0-15.0));
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 4.0, 45, 5, 0, 49, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("arml"), 4.0, 45, 5, 0, 49, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }        
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        if VarModule::is_flag(boma.object(), vars::gaogaen::SHOULD_CROSS_CHOP_DIVE_EARLY){
            
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 2.5, 30, 5, 0, 34, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("arml"), 2.5, 30, 5, 0, 34, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        if !VarModule::is_flag(boma.object(), vars::gaogaen::SHOULD_CROSS_CHOP_DIVE_EARLY){
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 2.5, 5, 5, 0, 30, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("arml"), 2.5, 5, 5, 0, 30, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::gaogaen::SHOULD_CROSS_CHOP_DIVE_EARLY){
            FT_MOTION_RATE(fighter, 0.75);
            KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::gaogaen::IS_INPUT_CROSS_CHOP_CANCEL){
            FT_MOTION_RATE(fighter, 1.25);
        }
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "gaogaen", script = "effect_specialairhistart" , category = ACMD_EFFECT , low_priority)]
unsafe fn gaogaen_special_air_hi_start_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("gaogaen_belt_fire2"), Hash40::new("feeler"), 0, 3, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::gaogaen::SHOULD_CROSS_CHOP_DIVE_EARLY) {
            EFFECT(fighter, Hash40::new("sys_hit_fire"), Hash40::new("bust"), 0.0, 0.0, 0.0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(fighter, 1.25);

            EFFECT_FOLLOW(fighter, Hash40::new("gaogaen_belt_fire_appeal"), Hash40::new("feeler"), 0, 3, 0, 0, 0, 0, 0.8, true);
            LAST_EFFECT_SET_RATE(fighter, 1.25);

            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("bust"), 0.0, 0, 0, 0, 0, 0, 0.4, true);
            LAST_EFFECT_SET_RATE(fighter, 0.25);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 1.0, 0, 0, 0, 0, 0, 0.4, true);
            LAST_EFFECT_SET_RATE(fighter, 0.25);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 7.0, 0, 0, 0, 0, 0, 0.4, true);
            LAST_EFFECT_SET_RATE(fighter, 0.25);

            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 1.0, 0, 0, 0, 0, 0, 0.4, true);
            LAST_EFFECT_SET_RATE(fighter, 0.25);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 7.0, 0, 0, 0, 0, 0, 0.4, true);
            LAST_EFFECT_SET_RATE(fighter, 0.25);
        }
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("gaogaen_chop_arc"), Hash40::new("gaogaen_chop_arc"), Hash40::new("trans"), 7, 14.5, -2, -111, -28, 77, 1.2, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("gaogaen_chop_arc"), Hash40::new("gaogaen_chop_arc"), Hash40::new("trans"), 7, 14.5, 0, -48, 23, 71, 1.2, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("gaogaen_chop_line_start"), Hash40::new("trans"), 0, 22, 0, 0, 0, 0, 1, true);
    }
}

#[acmd_script( agent = "gaogaen", script = "game_specialairhiturn" , category = ACMD_GAME , low_priority)]
unsafe fn gaogaen_special_air_hi_turn_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::gaogaen::IS_INPUT_CROSS_CHOP_CANCEL){
            FT_MOTION_RATE(fighter, 1.25);
        }
        else if VarModule::is_flag(boma.object(), vars::gaogaen::SHOULD_CROSS_CHOP_DIVE_EARLY) {
            FT_MOTION_RATE(fighter, 1.5);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::gaogaen::IS_INPUT_CROSS_CHOP_CANCEL){
            FT_MOTION_RATE(fighter, 1.5);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::gaogaen::IS_INPUT_CROSS_CHOP_CANCEL){
            FT_MOTION_RATE(fighter, 1.75);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::gaogaen::IS_INPUT_CROSS_CHOP_CANCEL){
            FT_MOTION_RATE(fighter, 1.0);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::gaogaen::IS_INPUT_CROSS_CHOP_CANCEL){
            FT_MOTION_RATE(fighter, 1.0);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        if VarModule::is_flag(boma.object(), vars::gaogaen::IS_INPUT_CROSS_CHOP_CANCEL){
            VarModule::off_flag(boma.object(), vars::gaogaen::IS_INPUT_CROSS_CHOP_CANCEL);
            VarModule::on_flag(boma.object(), vars::common::UP_SPECIAL_CANCEL);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
        }
        else{
            WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_HI_FLAG_FALL_TYPE_CHECK);
        }
    }
}

#[acmd_script( agent = "gaogaen", script = "effect_specialairhiturn" , category = ACMD_EFFECT , low_priority)]
unsafe fn gaogaen_special_air_hi_turn_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::gaogaen::IS_INPUT_CROSS_CHOP_CANCEL){
            EFFECT(fighter, Hash40::new("sys_hit_aura"), Hash40::new("bust"), 0.0, 0.0, 0.0, 0, 0, 0, 0.45, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(fighter, 1.25);

            EFFECT_FOLLOW(fighter, Hash40::new("gaogaen_belt_fire_appeal"), Hash40::new("feeler"), 0, 3, 0, 0, 0, 0, 0.75, true);
            LAST_EFFECT_SET_COLOR(fighter, 0.3, 0.3, 1.5);
            LAST_EFFECT_SET_RATE(fighter, 1.25);

            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("bust"), 0.0, 0, 0, 0, 0, 0, 0.4, true);
            LAST_EFFECT_SET_RATE(fighter, 0.25);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("arml"), 1.0, 0, 0, 0, 0, 0, 0.4, true);
            LAST_EFFECT_SET_RATE(fighter, 0.25);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("arml"), 7.0, 0, 0, 0, 0, 0, 0.4, true);
            LAST_EFFECT_SET_RATE(fighter, 0.25);

            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("armr"), 1.0, 0, 0, 0, 0, 0, 0.4, true);
            LAST_EFFECT_SET_RATE(fighter, 0.25);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("armr"), 7.0, 0, 0, 0, 0, 0, 0.4, true);
            LAST_EFFECT_SET_RATE(fighter, 0.25);
        }
        else if VarModule::is_flag(boma.object(), vars::gaogaen::SHOULD_CROSS_CHOP_DIVE_EARLY) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("bust"), 0.0, 0, 0, 0, 0, 0, 0.4, true);
            LAST_EFFECT_SET_RATE(fighter, 0.25);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 1.0, 0, 0, 0, 0, 0, 0.4, true);
            LAST_EFFECT_SET_RATE(fighter, 0.25);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 7.0, 0, 0, 0, 0, 0, 0.4, true);
            LAST_EFFECT_SET_RATE(fighter, 0.25);

            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 1.0, 0, 0, 0, 0, 0, 0.4, true);
            LAST_EFFECT_SET_RATE(fighter, 0.25);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 7.0, 0, 0, 0, 0, 0, 0.4, true);
            LAST_EFFECT_SET_RATE(fighter, 0.25);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(boma.object(), vars::gaogaen::IS_INPUT_CROSS_CHOP_CANCEL){
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 8, 5, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

#[acmd_script( agent = "gaogaen", script = "game_specialairhifall" , category = ACMD_GAME , low_priority)]
unsafe fn gaogaen_special_air_hi_fall_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        if VarModule::is_flag(boma.object(), vars::gaogaen::SHOULD_CROSS_CHOP_DIVE_EARLY) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 45, 90, 0, 70, 7.0, 0.0, 12.0, 1.0, Some(0.0), Some(7.0), Some(3.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 70, 100, 0, 50, 7.0, 0.0, 12.0, 1.0, Some(0.0), Some(7.0), Some(3.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.423);
        if !VarModule::is_flag(boma.object(), vars::gaogaen::SHOULD_CROSS_CHOP_DIVE_EARLY) {
            WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_HI_FLAG_DISABLE_OPPONENT_PASSIVE);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 312, 100, 175, 0, 6.0, 0.0, 8.0, 2.0, Some(0.0), Some(6.0), Some(3.5), 1.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 13.0, 54, 48, 0, 135, 6.0, 0.0, 8.0, 2.0, Some(0.0), Some(3.0), Some(4.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        }
        
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::gaogaen::SHOULD_CROSS_CHOP_DIVE_EARLY) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 70, 80, 100, 50, 6.0, 0.0, 8.0, 2.0, Some(0.0), Some(6.0), Some(3.5), 1.2, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 312, 100, 100, 0, 6.0, 0.0, 8.0, 2.0, Some(0.0), Some(6.0), Some(3.5), 1.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(boma.object(), vars::gaogaen::SHOULD_CROSS_CHOP_DIVE_EARLY) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 312, 100, 80, 0, 6.0, 0.0, 8.0, 2.0, Some(0.0), Some(6.0), Some(3.5), 1.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(boma.object(), vars::gaogaen::SHOULD_CROSS_CHOP_DIVE_EARLY) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 312, 100, 60, 0, 6.0, 0.0, 8.0, 2.0, Some(0.0), Some(6.0), Some(3.5), 1.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(boma.object(), vars::gaogaen::SHOULD_CROSS_CHOP_DIVE_EARLY) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 312, 100, 40, 0, 6.0, 0.0, 8.0, 2.0, Some(0.0), Some(6.0), Some(3.5), 1.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        }
    }
}

#[acmd_script( agent = "gaogaen", script = "game_specialhibound" , category = ACMD_GAME , low_priority)]
unsafe fn gaogaen_special_hi_bound_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
        if VarModule::is_flag(boma.object(), vars::gaogaen::SHOULD_CROSS_CHOP_DIVE_EARLY) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 85, 0, 70, 8.0, 0.0, 4.0, 8.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 361, 85, 0, 70, 10.0, 0.0, 6.0, 11.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 361, 85, 0, 70, 8.0, 0.0, 4.0, 8.0, Some(0.0), Some(4.0), Some(17.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_PUNCH);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("top"), 10.8, 361, 72, 0, 89, 8.0, 0.0, 4.0, 8.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 361, 80, 0, 85, 10.0, 0.0, 6.0, 11.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 5.5, 361, 80, 0, 85, 8.0, 0.0, 4.0, 8.0, Some(0.0), Some(4.0), Some(17.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_PUNCH);
        }
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "gaogaen", script = "game_speciallwstart" , category = ACMD_GAME , low_priority)]
unsafe fn gaogaen_special_lw_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let mut damage_multiplier = 1.25;
    let mut damage_to_soak = 25.0;
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        //FT_MOTION_RATE(fighter, 0.25);
        VarModule::off_flag(boma.object(), vars::gaogaen::IS_SPECIAL_LW_COMMAND_DASH);
        VarModule::off_flag(fighter.object(), vars::gaogaen::IS_ENABLE_SPECIAL_LW_LARIAT_INPUT);
        VarModule::off_flag(fighter.object(), vars::gaogaen::IS_INPUT_SPECIAL_LW_LARIAT);
        VarModule::off_flag(fighter.object(), vars::gaogaen::DID_SPECIAL_LW_COMMAND_DASH_TANK_DAMAGE);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW){
            VarModule::on_flag(boma.object(), vars::gaogaen::IS_SPECIAL_LW_COMMAND_DASH);
        }
        if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_LW_COMMAND_DASH){
            KineticModule::add_speed(boma, &Vector3f::new(2.7, 0.0, 0.0));
            DamageModule::set_damage_mul(boma, 2.5);
            //damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, (damage_multiplier * damage_to_soak) - 0.01);
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        }
        else{
            WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_LW_FLAG_STANCE_START);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.5);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_LW_COMMAND_DASH){
            VarModule::on_flag(fighter.object(), vars::gaogaen::IS_ENABLE_SPECIAL_LW_LARIAT_INPUT);
        }
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_LW_COMMAND_DASH){
            VarModule::off_flag(fighter.object(), vars::gaogaen::IS_ENABLE_SPECIAL_LW_LARIAT_INPUT);
            if VarModule::is_flag(boma.object(), vars::gaogaen::IS_INPUT_SPECIAL_LW_LARIAT){
                damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
                DamageModule::set_damage_mul(boma, 1.0);
                REVERSE_LR(fighter);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_S_LARIAT, false);
            }
            else if VarModule::is_flag(fighter.object(), vars::gaogaen::DID_SPECIAL_LW_COMMAND_DASH_TANK_DAMAGE){
                FT_MOTION_RATE(fighter, 0.4);
            }
        }
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_LW_COMMAND_DASH){
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
            DamageModule::set_damage_mul(boma, 1.0);
        }
        else{
            WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_LW_FLAG_STANCE_END);
            FT_MOTION_RATE(fighter, 0.25);
        }
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 46.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.object(), vars::gaogaen::DID_SPECIAL_LW_COMMAND_DASH_TANK_DAMAGE){
            FT_MOTION_RATE(fighter, 1.0);
        }
    }
}

#[acmd_script( agent = "gaogaen", script = "effect_speciallwstart" , category = ACMD_EFFECT , low_priority)]
unsafe fn gaogaen_special_lw_start_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("gaogaen_revenge_pose"), Hash40::new("top"), 0, 11, 3, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        FLASH(fighter, 1, 1, 1, 0.75);
    }
    wait(lua_state, 1.0);
    for _ in 0..2{
        if is_excute(fighter) {
            FLASH(fighter, 0.7, 0.7, 0.7, 0.5);

            if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_LW_COMMAND_DASH){
                EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -3, 8, 0, 0, 180, 0, 0.85, true, *EF_FLIP_YZ, 0.6);
                EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 4, 11, -1, 0, 180, 0, 0.85, true, *EF_FLIP_YZ, 0.6);
                EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 3, -6, 0, 180, 0, 0.9, true, *EF_FLIP_YZ, 0.6);

                EFFECT(fighter, Hash40::new("sys_hit_fire"), Hash40::new("bust"), 0.0, 0.0, 2.0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
                LAST_EFFECT_SET_RATE(fighter, 1.25);

                EFFECT_FOLLOW(fighter, Hash40::new("gaogaen_belt_fire_appeal"), Hash40::new("feeler"), 0, 3, 0, 0, 0, 0, 0.5, true);
                LAST_EFFECT_SET_RATE(fighter, 1.25);

                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("bust"), 0.0, 0, 0, 0, 0, 0, 0.5, true);
                LAST_EFFECT_SET_RATE(fighter, 0.5);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 1.0, 0, 0, 0, 0, 0, 0.4, true);
                LAST_EFFECT_SET_RATE(fighter, 0.5);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("kneel"), 6.0, 0, 0, 0, 0, 0, 0.6, true);
                LAST_EFFECT_SET_RATE(fighter, 0.5);

                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 1.0, 0, 0, 0, 0, 0, 0.4, true);
                LAST_EFFECT_SET_RATE(fighter, 0.5);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("kneer"), 6.0, 0, 0, 0, 0, 0, 0.6, true);
                LAST_EFFECT_SET_RATE(fighter, 0.5);
            }
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            FLASH(fighter, 0.67, 0, 0.78, 0.31);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            COL_NORMAL(fighter);

            if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_LW_COMMAND_DASH){
                EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -3, 8, 0, 0, 180, 0, 0.85, true, *EF_FLIP_YZ, 0.6);
                EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 4, 11, -1, 0, 180, 0, 0.85, true, *EF_FLIP_YZ, 0.6);
                EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 3, -6, 0, 180, 0, 0.9, true, *EF_FLIP_YZ, 0.6);
            }
        }
        wait(lua_state, 2.0);
    }
    if is_excute(fighter) {
        FLASH(fighter, 0.7, 0.7, 0.7, 0.5);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.67, 0, 0.78, 0.31);

        if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_LW_COMMAND_DASH){
            EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -3, 8, 0, 0, 180, 0, 0.85, true, *EF_FLIP_YZ, 0.6);
            EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 4, 11, -1, 0, 180, 0, 0.85, true, *EF_FLIP_YZ, 0.6);
            EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 3, -6, 0, 180, 0, 0.9, true, *EF_FLIP_YZ, 0.6);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.7, 0.7, 0.7, 0.5);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);

        if VarModule::is_flag(boma.object(), vars::gaogaen::IS_SPECIAL_LW_COMMAND_DASH){
            EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -3, 8, 0, 0, 180, 0, 0.85, true, *EF_FLIP_YZ, 0.6);
            EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 4, 11, -1, 0, 180, 0, 0.85, true, *EF_FLIP_YZ, 0.6);
            EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 3, -6, 0, 180, 0, 0.9, true, *EF_FLIP_YZ, 0.6);
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        gaogaen_special_n_game,
        gaogaen_special_air_n_game,
        gaogaen_special_s_start_game,
        gaogaen_special_s_start_effect,
        gaogaen_special_s_lariat_game,
        gaogaen_special_air_s_lariat_game,
        gaogaen_special_s_shoulder_game,
        gaogaen_special_air_s_shoulder_game,
        gaogaen_special_hi_start_game,
        gaogaen_special_air_hi_start_game,
        gaogaen_special_air_hi_start_effect,
        gaogaen_special_air_hi_turn_game,
        gaogaen_special_air_hi_turn_effect,
        gaogaen_special_air_hi_fall_game,
        gaogaen_special_hi_bound_game,
        gaogaen_special_lw_start_game,
        gaogaen_special_lw_start_effect,
    );
}

