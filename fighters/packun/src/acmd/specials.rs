
use super::*;


#[acmd_script( agent = "packun", script = "game_specialsshoot" , category = ACMD_GAME , low_priority)]
unsafe fn packun_special_s_shoot_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let stance = VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE);
    let charged = WorkModule::get_int(boma, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT) == 75;
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 10.0, 3.0);
        //println!("charge amount is {}", charge);
    } 
    if stance == 0 {
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, (9.0/5.0));
        }
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, 1.0);
            if charged {
                ATTACK(fighter, 0, 0, Hash40::new("mouth"), 12.0, 30, 66, 0, 60, 9.0, 2.0, 0.0, 0.0, Some(5.5), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BITE);
            }
            else {
                ATTACK(fighter, 0, 0, Hash40::new("mouth"), 8.0, 30, 66, 0, 60, 9.0, 2.0, 0.0, 0.0, Some(5.5), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BITE);
            }
        }

        wait(lua_state, 5.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
    else if stance == 1 {
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, (8.0/5.0));
        }
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, 1.0);
            if charged {
                ATTACK(fighter, 0, 0, Hash40::new("mouth"), 2.0, 365, 66, 0, 10, 9.0, 2.0, 0.0, 0.0, Some(5.5), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BITE);
            }
            else {
                ATTACK(fighter, 0, 0, Hash40::new("mouth"), 1.0, 365, 66, 0, 10, 9.0, 2.0, 0.0, 0.0, Some(5.5), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BITE);
            }
        }

        wait(lua_state, 16.0);
        if is_excute(fighter) {
            if charged {
                ATTACK(fighter, 0, 0, Hash40::new("mouth"), 3.0, 55, 66, 0, 65, 9.0, 2.0, 0.0, 0.0, Some(5.5), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BITE);
                AttackModule::set_poison_param(boma, 0, 361, 45, 2.5, false);
            }
            else {
                ATTACK(fighter, 0, 0, Hash40::new("mouth"), 2.0, 50, 66, 0, 70, 9.0, 2.0, 0.0, 0.0, Some(5.5), Some(0.0), Some(0.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BITE);
                AttackModule::set_poison_param(boma, 0, 361, 45, 1.0, false);
            }
        }

        wait(lua_state, 3.0);
        if is_excute(fighter){
            AttackModule::clear_all(boma);
        }
    }
    else if stance == 2 {
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, (15.0/5.0));
        }
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, 1.0);
            if charged {
                ATTACK(fighter, 0, 0, Hash40::new("mouth"), 16.0, 290, 100, 5, 0, 9.0, 2.0, 0.0, 0.0, Some(5.5), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
                AttackModule::set_add_reaction_frame(boma, 0, 30.0, false);
            }
            else {
                ATTACK(fighter, 0, 0, Hash40::new("mouth"), 10.0, 290, 100, 5, 0, 9.0, 2.0, 0.0, 0.0, Some(5.5), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            }
        }

        wait(lua_state, 3.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
}

#[acmd_script( agent = "packun", script = "game_specialairsshoot" , category = ACMD_GAME , low_priority)]
unsafe fn packun_special_air_s_shoot_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let stance = VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE);
    let charged = WorkModule::get_int(boma, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT) == 75;
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 10.0, 3.0);
        //println!("charge amount is {}", charge);
    } 
    if stance == 0 {
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, (9.0/5.0));
        }
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, 1.0);
            if charged {
                ATTACK(fighter, 0, 0, Hash40::new("mouth"), 12.0, 30, 66, 0, 60, 9.0, 2.0, 0.0, 0.0, Some(5.5), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BITE);
            }
            else {
                ATTACK(fighter, 0, 0, Hash40::new("mouth"), 8.0, 30, 66, 0, 60, 9.0, 2.0, 0.0, 0.0, Some(5.5), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BITE);
            }
        }

        wait(lua_state, 5.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
    else if stance == 1 {
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, (8.0/5.0));
        }
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, 1.0);
            if charged {
                ATTACK(fighter, 0, 0, Hash40::new("mouth"), 2.0, 365, 66, 0, 10, 9.0, 2.0, 0.0, 0.0, Some(5.5), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BITE);
            }
            else {
                ATTACK(fighter, 0, 0, Hash40::new("mouth"), 1.0, 365, 66, 0, 10, 9.0, 2.0, 0.0, 0.0, Some(5.5), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BITE);
            }
        }

        wait(lua_state, 16.0);
        if is_excute(fighter) {
            if charged {
                ATTACK(fighter, 0, 0, Hash40::new("mouth"), 3.0, 55, 66, 0, 65, 9.0, 2.0, 0.0, 0.0, Some(5.5), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BITE);
                AttackModule::set_poison_param(boma, 0, 361, 45, 2.5, false);
            }
            else {
                ATTACK(fighter, 0, 0, Hash40::new("mouth"), 2.0, 50, 66, 0, 70, 9.0, 2.0, 0.0, 0.0, Some(5.5), Some(0.0), Some(0.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BITE);
                AttackModule::set_poison_param(boma, 0, 361, 45, 1.0, false);
            }
        }

        wait(lua_state, 3.0);
        if is_excute(fighter){
            AttackModule::clear_all(boma);
        }
    }
    else if stance == 2 {
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, (15.0/5.0));
        }
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, 1.0);
            if charged {
                ATTACK(fighter, 0, 0, Hash40::new("mouth"), 16.0, 290, 100, 5, 0, 9.0, 2.0, 0.0, 0.0, Some(5.5), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
                AttackModule::set_add_reaction_frame(boma, 0, 30.0, false);
            }
            else {
                ATTACK(fighter, 0, 0, Hash40::new("mouth"), 10.0, 290, 100, 5, 0, 9.0, 2.0, 0.0, 0.0, Some(5.5), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            }
        }

        wait(lua_state, 3.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
}

#[acmd_script( agent = "packun", script = "effect_specialsend" , category = ACMD_EFFECT , low_priority)]
unsafe fn packun_special_s_end_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let boma = fighter.boma();
    let stance = VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        if stance == 1 {
            EFFECT_FOLLOW(fighter, Hash40::new("packun_poison_max"), Hash40::new("mouth"), 3, -1, 0, 0, 0, 0, 1, true);
        }
        if stance == 2 {
            LANDING_EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("mouth"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("packun_poison_max"), -1);
    }
}

#[acmd_script( agent = "packun", script = "effect_specialsshoot" , category = ACMD_EFFECT , low_priority)]
unsafe fn packun_special_s_shoot_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let boma = fighter.boma();
    let stance = VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        if stance == 1 {
            EFFECT(fighter, Hash40::new("packun_poison_breath2"), Hash40::new("mouth"), 3, -0.6, 0, 0, 90, -100, 2, 0, 0, 0, 0, 0, 0, true);
        }
        else if stance == 0 {
            EFFECT(fighter, Hash40::new("packun_spikeball_shoot"), Hash40::new("mouth"), 3, -0.6, 0, 0, 90, -100, 1, 0, 0, 0, 0, 0, 0, true);
        }
        else {
            EFFECT_FOLLOW_NO_STOP_FLIP(fighter, Hash40::new("packun_bite_line"), Hash40::new("packun_bite_line"), Hash40::new("top"), -5, 11, 19, 0, -130, 35, 1, true, *EF_FLIP_YZ);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            if stance == 2 {
                EFFECT_FOLLOW_FLIP(fighter, Hash40::new("packun_bite_line2"), Hash40::new("packun_bite_line2"), Hash40::new("top"), -12, 9, 20, 10, 50, 10, 0.8, true, *EF_FLIP_YZ);
                EFFECT_FOLLOW_FLIP(fighter, Hash40::new("packun_bite"), Hash40::new("packun_bite"), Hash40::new("top"), -9, 11, 18, 0, -120, 20, 1, true, *EF_FLIP_YZ);
        
            }
        }
    }
}

#[acmd_script( agent = "packun", script = "effect_specialairsend", category = ACMD_EFFECT, low_priority )]
unsafe fn packun_special_air_s_end_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let stance = VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE);
    if is_excute(fighter) {
        if stance == 1 {
            EFFECT_FOLLOW(fighter, Hash40::new("packun_poison_max"), Hash40::new("mouth"), 3, -1, 0, 0, 0, 0, 1, true);
        }
        if stance == 2 {
            LANDING_EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("mouth"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("packun_poison_max"), -1);
    }
}

#[acmd_script( agent = "packun", script = "effect_specialairsshoot", category = ACMD_EFFECT, low_priority )]
unsafe fn packun_special_air_s_shoot_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let stance = VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE);
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        if stance == 1 {
            EFFECT(fighter, Hash40::new("packun_poison_breath2"), Hash40::new("mouth"), 3, -0.6, 0, 0, 90, -100, 2, 0, 0, 0, 0, 0, 0, true);
        }
        else if stance == 0 {
            EFFECT(fighter, Hash40::new("packun_spikeball_shoot"), Hash40::new("mouth"), 3, -0.6, 0, 0, 90, -100, 1, 0, 0, 0, 0, 0, 0, true);
        }
        else {
            EFFECT_FOLLOW_NO_STOP_FLIP(fighter, Hash40::new("packun_bite_line"), Hash40::new("packun_bite_line"), Hash40::new("top"), -5, 11, 19, 0, -130, 35, 1, true, *EF_FLIP_YZ);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            if stance == 2 {
                EFFECT_FOLLOW_FLIP(fighter, Hash40::new("packun_bite_line2"), Hash40::new("packun_bite_line2"), Hash40::new("top"), -12, 9, 20, 10, 50, 10, 0.8, true, *EF_FLIP_YZ);
                EFFECT_FOLLOW_FLIP(fighter, Hash40::new("packun_bite"), Hash40::new("packun_bite"), Hash40::new("top"), -9, 11, 18, 0, -120, 20, 1, true, *EF_FLIP_YZ);
        
            }
        }
    }
}

#[acmd_script( agent = "packun", script = "game_specialhi" , category = ACMD_GAME , low_priority)]
unsafe fn packun_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.500);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        ATTACK(fighter, 0, 1, Hash40::new("top"), 1.2, 367, 100, 40, 0, 4.0, 0.0, 6.0, -8.0, Some(0.0), Some(6.0), Some(8.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 0, false);
        ATTACK(fighter, 3, 0, Hash40::new("arml"), 1.2, 367, 100, 20, 0, 5.5, 3.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 4, 0, Hash40::new("armr"), 1.2, 367, 100, 20, 0, 5.5, 3.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 1.2, 367, 100, 0, 20, 3.5, 6.0, 0.0, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("armr"), 1.2, 367, 100, 0, 20, 3.5, 6.0, 0.0, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("arml"), 1.2, 367, 100, 0, 20, 5.0, 1.5, 0.0, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 4, 0, Hash40::new("armr"), 1.2, 367, 100, 0, 20, 5.0, 1.5, 0.0, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 58.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 4.0, 367, 60, 0, 90, 3.0, 8.0, 0.0, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("armr"), 4.0, 367, 60, 0, 90, 3.0, 8.0, 0.0, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("arml"), 0.8, 367, 90, 0, 15, 4.0, 2.5, 0.0, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 4, 0, Hash40::new("armr"), 0.8, 367, 90, 0, 15, 4.0, 2.5, 0.0, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 64.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 7.0, 75, 70, 0, 70, 4.0, 8.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("armr"), 7.0, 75, 70, 0, 70, 4.0, 8.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("arml"), 7.0, 75, 70, 0, 70, 5.0, 2.5, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 4, 0, Hash40::new("armr"), 7.0, 75, 70, 0, 70, 5.0, 2.5, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 70.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        FT_MOTION_RATE(fighter, 0.500);
    }
    frame(lua_state, 86.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 3.030);
    }
    frame(lua_state, 90.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_HI_DIVE);
    }
    
}

#[acmd_script( agent = "packun", script = "game_specialairhi" , category = ACMD_GAME , low_priority)]
unsafe fn packun_special_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.500);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        ATTACK(fighter, 0, 1, Hash40::new("top"), 1.2, 367, 100, 40, 0, 4.0, 0.0, 6.0, -8.0, Some(0.0), Some(6.0), Some(8.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 0, false);
        ATTACK(fighter, 3, 0, Hash40::new("arml"), 1.2, 367, 100, 20, 0, 5.5, 3.0, 0.0, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 4, 0, Hash40::new("armr"), 1.2, 367, 100, 20, 0, 5.5, 3.0, 0.0, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 1.2, 367, 100, 0, 20, 3.5, 6.0, 0.0, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("armr"), 1.2, 367, 100, 0, 20, 3.5, 6.0, 0.0, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("arml"), 1.2, 367, 100, 0, 20, 5.0, 1.5, 0.0, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 4, 0, Hash40::new("armr"), 1.2, 367, 100, 0, 20, 5.0, 1.5, 0.0, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 58.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 4.0, 367, 60, 0, 90, 3.0, 8.0, 0.0, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("armr"), 4.0, 367, 60, 0, 90, 3.0, 8.0, 0.0, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("arml"), 0.8, 367, 90, 0, 15, 4.0, 2.5, 0.0, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 4, 0, Hash40::new("armr"), 0.8, 367, 90, 0, 15, 4.0, 2.5, 0.0, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 64.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 7.0, 75, 70, 0, 70, 4.0, 8.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("armr"), 7.0, 75, 70, 0, 70, 4.0, 8.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("arml"), 7.0, 75, 70, 0, 70, 5.0, 2.5, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 4, 0, Hash40::new("armr"), 7.0, 75, 70, 0, 70, 5.0, 2.5, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 70.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        FT_MOTION_RATE(fighter, 0.500);
    }
    frame(lua_state, 86.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 3.030);
    }
    frame(lua_state, 90.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_HI_DIVE);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        packun_special_s_shoot_game,
        packun_special_air_s_shoot_game,
        packun_special_s_end_effect,
        packun_special_s_shoot_effect,
        packun_special_air_s_end_effect,
        packun_special_air_s_shoot_effect,
        packun_special_hi_game,
        packun_special_air_hi_game,
    );
}

