use super::*;

unsafe extern "C" fn samusd_attack_s3_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("handr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("handr"), *HIT_STATUS_NORMAL);
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 12.0, 361, 102, 0, 20, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 13.0, 361, 102, 0, 20, 3.5, 0.6, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 13.0, 361, 102, 0, 20, 3.5, 6.0, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("kneel"), 12.0, 361, 102, 0, 20, 3.5, 0.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    
}

unsafe extern "C" fn samusd_attack_s3_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc"), Hash40::new("top"), -2, 13.5, 5, 30, 0, 195, 1, true);
        let color_vec = match WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) {
            0 => Vector3f::new(0.1, 0.7, 3.0),//nor
            1 => Vector3f::new(0.55, 0.88, 0.0004),//g
            2 => Vector3f::new(1.25, 0.55, 1.5),//pur
            3 => Vector3f::new(0.84, 0.7, 0.03),//r
            4 => Vector3f::new(0.1, 1.0, 2.0),//y
            5 => Vector3f::new(0.9, 0.03, 0.03),//w
            6 => Vector3f::new(1.15, 0.65, 0.03),//blac
            7 => Vector3f::new(0.78, 0.5, 2.5),//pi
            _ => Vector3f::new(0.1, 0.7, 3.0)
        }; //matches glow color
        LAST_EFFECT_SET_COLOR(fighter, color_vec.x, color_vec.y, color_vec.z);
        LAST_EFFECT_SET_RATE(fighter, 1.0);
    }
}

unsafe extern "C" fn samusd_attack_s3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("handr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("handr"), *HIT_STATUS_NORMAL);
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 12.0, 361, 102, 0, 20, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 13.0, 361, 102, 0, 20, 3.5, 0.6, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 13.0, 361, 102, 0, 20, 3.5, 6.0, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("kneel"), 12.0, 361, 102, 0, 20, 3.5, 0.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    
}

unsafe extern "C" fn samusd_attack_s3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc"), Hash40::new("top"), -1.5, 11.5, 9, 2, 5, 165, 0.95, true);
        let color_vec = match WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) {
            0 => Vector3f::new(0.1, 0.7, 3.0),//nor
            1 => Vector3f::new(0.55, 0.88, 0.0004),//g
            2 => Vector3f::new(1.25, 0.55, 1.5),//pur
            3 => Vector3f::new(0.84, 0.7, 0.03),//r
            4 => Vector3f::new(0.1, 1.0, 2.0),//y
            5 => Vector3f::new(0.9, 0.03, 0.03),//w
            6 => Vector3f::new(1.15, 0.65, 0.03),//blac
            7 => Vector3f::new(0.78, 0.5, 2.5),//pi
            _ => Vector3f::new(0.1, 0.7, 3.0)
        }; //matches glow color
        LAST_EFFECT_SET_COLOR(fighter, color_vec.x, color_vec.y, color_vec.z);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
}

unsafe extern "C" fn samusd_attack_s3_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("handr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("handr"), *HIT_STATUS_NORMAL);
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 12.0, 361, 102, 0, 20, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 13.0, 361, 102, 0, 20, 3.5, 0.6, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 13.0, 361, 102, 0, 20, 3.5, 6.0, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("kneel"), 12.0, 361, 102, 0, 20, 3.5, 0.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    
}

unsafe extern "C" fn samusd_attack_s3_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc"), Hash40::new("top"), -2, 6, 6, -15, 0, 195, 1, true);
        let color_vec = match WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) {
            0 => Vector3f::new(0.1, 0.7, 3.0),//nor
            1 => Vector3f::new(0.55, 0.88, 0.0004),//g
            2 => Vector3f::new(1.25, 0.55, 1.5),//pur
            3 => Vector3f::new(0.84, 0.7, 0.03),//r
            4 => Vector3f::new(0.1, 1.0, 2.0),//y
            5 => Vector3f::new(0.9, 0.03, 0.03),//w
            6 => Vector3f::new(1.15, 0.65, 0.03),//blac
            7 => Vector3f::new(0.78, 0.5, 2.5),//pi
            _ => Vector3f::new(0.1, 0.7, 3.0)
        }; //matches glow color
        LAST_EFFECT_SET_COLOR(fighter, color_vec.x, color_vec.y, color_vec.z);
        LAST_EFFECT_SET_RATE(fighter, 1.0);
    }
}

unsafe extern "C" fn samusd_attack_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE(fighter, 0.8);
    //ground only pop up hitbox, f7
    frame(lua_state, 7.0); 
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 100, 100, 70, 0, 5.5, 0.0, 8.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    //the rest of the first kick, f8-10
    frame(lua_state, 8.0); 
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 100, 100, 70, 0, 5.0, 0.0, 8.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 366, 100, 40, 0, 4.5, 0.0, 22.0, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 95, 100, 45, 0, 4.0, 0.0, 18.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 4.0, 95, 100, 45, 0, 3.5, 0.0, 14.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 9.0); 
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 100, 100, 70, 0, 5.0, 0.0, 8.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 366, 100, 40, 0, 4.5, 0.0, 29.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 95, 100, 45, 0, 4.0, 0.0, 21.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 4.0, 95, 100, 45, 0, 3.5, 0.0, 16.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 11.0); 
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    //second kick (early strong hit), f12-16
    //the extra frames are to adjust for z axis alginment issues on the animation
    frame(lua_state, 13.0); 
    if is_excute(fighter) {
        ATTACK(fighter, 4, 0, Hash40::new("legr"), 6.0, 100, 110, 0, 75, 5.0, 12.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 5, 0, Hash40::new("legr"), 6.0, 100, 110, 0, 75, 4.5, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 6, 0, Hash40::new("legr"), 6.0, 100, 110, 0, 75, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 15.0); 
    if is_excute(fighter) {
        ATTACK(fighter, 4, 0, Hash40::new("legr"), 6.0, 100, 110, 0, 75, 5.0, 12.0, 0.0, -8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 5, 0, Hash40::new("legr"), 6.0, 100, 110, 0, 75, 4.5, 5.0, 0.0, -3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        //ATTACK(fighter, 6, 0, Hash40::new("legr"), 6.0, 100, 110, 0, 75, 3.5, 0.0, 0.0, -3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 17.0); 
    if is_excute(fighter) {
        ATTACK(fighter, 4, 0, Hash40::new("legr"), 6.0, 100, 110, 0, 75, 5.0, 12.0, 0.0, -9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 5, 0, Hash40::new("legr"), 6.0, 100, 110, 0, 75, 4.5, 5.0, 0.0, -4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 6, 0, Hash40::new("legr"), 6.0, 100, 110, 0, 75, 3.5, 0.0, 0.0, -3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    //second kick (late weak hit), f17-18
    frame(lua_state, 19.0); 
    if is_excute(fighter) {
        ATTACK(fighter, 4, 0, Hash40::new("legr"), 3.0, 110, 100, 0, 65, 5.0, 9.0, 0.0, -7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 5, 0, Hash40::new("legr"), 3.0, 110, 100, 0, 65, 4.5, 5.0, 0.0, -5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 6, 0, Hash40::new("legr"), 3.0, 110, 100, 0, 65, 3.5, 0.0, 0.0, -5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 22.0); 
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn samusd_attack_hi3_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samusd_swing_s"));
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samusd_swing_l"));
    }
    
}

unsafe extern "C" fn samusd_attack_hi3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0); 
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 17, 0, 10, 30, 90, 1.3, true);
        let color_vec = match WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) {
            0 => Vector3f::new(0.1, 0.7, 3.0),//nor
            1 => Vector3f::new(0.55, 0.88, 0.0004),//g
            2 => Vector3f::new(1.25, 0.55, 1.5),//pur
            3 => Vector3f::new(0.84, 0.7, 0.03),//r
            4 => Vector3f::new(0.1, 1.0, 2.0),//y
            5 => Vector3f::new(0.9, 0.03, 0.03),//w
            6 => Vector3f::new(1.15, 0.65, 0.03),//blac
            7 => Vector3f::new(0.78, 0.5, 2.5),//pi
            _ => Vector3f::new(0.1, 0.7, 3.0)
        }; //matches glow color
        LAST_EFFECT_SET_COLOR(fighter, color_vec.x, color_vec.y, color_vec.z);
        //LAST_EFFECT_SET_RATE(fighter, 1);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 17, 0, 25, 100, 100, 1.3, true);
        let color_vec = match WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) {
            0 => Vector3f::new(0.1, 0.7, 3.0),//nor
            1 => Vector3f::new(0.55, 0.88, 0.0004),//g
            2 => Vector3f::new(1.25, 0.55, 1.5),//pur
            3 => Vector3f::new(0.84, 0.7, 0.03),//r
            4 => Vector3f::new(0.1, 1.0, 2.0),//y
            5 => Vector3f::new(0.9, 0.03, 0.03),//w
            6 => Vector3f::new(1.15, 0.65, 0.03),//blac
            7 => Vector3f::new(0.78, 0.5, 2.5),//pi
            _ => Vector3f::new(0.1, 0.7, 3.0)
        }; //matches glow color
        LAST_EFFECT_SET_COLOR(fighter, color_vec.x, color_vec.y, color_vec.z);
        //LAST_EFFECT_SET_RATE(fighter, 1);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
    }
}

unsafe extern "C" fn samusd_attack_hi3_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }

    frame(lua_state, 7.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 6);
    }

    frame(lua_state, 13.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn samusd_attack_lw3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 8.0, 80, 65, 0, 70, 3.8, 0.0, 0.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 80, 65, 0, 70, 7.2, 0.0, 1.6, 14.4, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
    }
    frame(lua_state, 9.0);
    FT_MOTION_RATE_RANGE(fighter, 9.0, 36.0, 25.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

pub fn install() {
    smashline::Agent::new("samusd")
        .acmd("game_attacks3hi", samusd_attack_s3_hi_game)
        .acmd("effect_attacks3hi", samusd_attack_s3_hi_effect)
        .acmd("game_attacks3", samusd_attack_s3_game)
        .acmd("effect_attacks3", samusd_attack_s3_effect)
        .acmd("game_attacks3lw", samusd_attack_s3_lw_game)
        .acmd("effect_attacks3lw", samusd_attack_s3_lw_effect)
        .acmd("game_attackhi3", samusd_attack_hi3_game)
        .acmd("sound_attackhi3", samusd_attack_hi3_sound)
        .acmd("effect_attackhi3", samusd_attack_hi3_effect)
        .acmd("expression_attackhi3", samusd_attack_hi3_expression)
        .acmd("game_attacklw3", samusd_attack_lw3_game)
        .install();
}
