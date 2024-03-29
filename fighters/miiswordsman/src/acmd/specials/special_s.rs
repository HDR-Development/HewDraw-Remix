use super::*;

// ==================================================================================================
// ======================================== AIRBORNE ASSAULT ========================================
// ==================================================================================================

unsafe extern "C" fn game_specials1start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let end_frame = MotionModule::end_frame(boma);
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.4);
    }
    
}

unsafe extern "C" fn game_specialairs1start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let end_frame = MotionModule::end_frame(boma);
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.4);
    }
    
}

unsafe extern "C" fn game_specials1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {

    }
    
}

unsafe extern "C" fn game_specialairs1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {

    }
    
}
/*
unsafe fn game_specials1hit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 12.5, 44, 75, 0, 56, 4.5, 0.0, 2.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 12.5, 44, 75, 0, 56, 4.5, 0.0, 6.5, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 12.5, 44, 75, 0, 56, 4.5, 0.0, 11.5, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    
}
*/

// =============================================================================================
// ===================================== KINETIC SLASH =========================================
// =============================================================================================

unsafe extern "C" fn game_specials2dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;

    frame(lua_state, 10.0);
    if is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 0.5);
    }
}

unsafe extern "C" fn game_specials2attack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 55, 88, 0, 55, 5.0, 0.0, 5.5, 16.0, Some(0.0), Some(5.5), Some(14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 55, 88, 0, 55, 3.5, 0.0, 5.5, 15.0, Some(0.0), Some(5.5), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_ATTACK_END);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn effect_specials2attack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        // Vanilla Effects
        //EFFECT_FOLLOW_WORK(agent, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        //LAST_EFFECT_SET_COLOR(agent, 1, 0.72, 1.1);
        
        //EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_blastwind_stab"), Hash40::new("top"), -0.0, 5, 12, 0, 0, 0, 1.1, true);
        //EFFECT_DETACH_KIND(agent, Hash40::new("miiswordsman_blastwind_stab"), -1);
        EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 11.4, 0, 0, 0, 0, 2, true);
        EFFECT_DETACH_KIND(agent, Hash40::new("sys_smash_flash_s"), -1);
        EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 5, 7, 0, 0, 0, 1, 0, 0, 10, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.511, 0.264, 1.3);
        LAST_EFFECT_SET_RATE(agent, 0.7);
        EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 5, -3, 0, 0, 0, 0.8, 0, 0, 10, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.511, 0.264, 1.3);
        LAST_EFFECT_SET_RATE(agent, 0.7);
        

        // Sword Trail
        //AFTER_IMAGE4_ON_WORK_arg29(agent, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD, 5, Hash40::new("haver"), 0, 0.2, 0, Hash40::new("haver"), -0.0, 10.8, 0, true, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_counter_arc"), Hash40::new("top"), -1, 7, 7.5, 0.0, -180.0, 25.3, 1.1, true);
	    LAST_EFFECT_SET_RATE(agent, 2.5);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        //EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0.0, 10, 6, -10.6, -159.5, 55.3, 1.1, true);
	    //LAST_EFFECT_SET_RATE(agent, 1.4);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 9.25, 0, 0, 0, 180, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 5);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        //EFFECT_FOLLOW_WORK(agent, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        //LAST_EFFECT_SET_COLOR(agent, 2.5, 2.5, 2.5);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        //EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 6, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        //EFFECT_OFF_KIND_WORK(agent, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, false, false);
    }
}

unsafe extern "C" fn game_specialairs2dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 10.0);
    if is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 0.5);
    }
}

unsafe extern "C" fn game_specialairs2attack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 55, 88, 0, 55, 5.0, 0.0, 5.5, 16.0, Some(0.0), Some(5.5), Some(14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 55, 88, 0, 55, 3.5, 0.0, 5.5, 15.0, Some(0.0), Some(5.5), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_ATTACK_END);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn effect_specialairs2attack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        // Vanilla Effects
        //EFFECT_FOLLOW_WORK(agent, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        //LAST_EFFECT_SET_COLOR(agent, 1, 0.72, 1.1);
        
    
        EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 11.4, 0, 0, 0, 0, 2, true);
        EFFECT_DETACH_KIND(agent, Hash40::new("sys_smash_flash_s"), -1);
        EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 5, 7, 0, 0, 0, 1, 0, 0, 10, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.511, 0.264, 1.3);
        LAST_EFFECT_SET_RATE(agent, 0.7);
        EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 5, -3, 0, 0, 0, 0.8, 0, 0, 10, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.511, 0.264, 1.3);
        LAST_EFFECT_SET_RATE(agent, 0.7);
        

        // Sword Trail
        //AFTER_IMAGE4_ON_WORK_arg29(agent, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD, 5, Hash40::new("haver"), 0, 0.2, 0, Hash40::new("haver"), -0.0, 10.8, 0, true, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);

    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_counter_arc"), Hash40::new("top"), -1, 7, 7.5, 0.0, -180.0, 25.3, 1.1, true);
	    LAST_EFFECT_SET_RATE(agent, 2.5);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        //EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0.0, 10, 6, -10.6, -159.5, 55.3, 1.1, true);
	    //LAST_EFFECT_SET_RATE(agent, 1.4);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 9.25, 0, 0, 0, 180, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 5);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        //EFFECT_FOLLOW_WORK(agent, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        //LAST_EFFECT_SET_COLOR(agent, 2.5, 2.5, 2.5);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        //EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 6, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        //EFFECT_OFF_KIND_WORK(agent, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, false, false);
    }
}

// =============================================================================================
// ======================================== CHAKRAM ============================================
// =============================================================================================

unsafe extern "C" fn game_specials3_1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let end_frame = MotionModule::end_frame(boma);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.27);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_REQUEST_GENERATE);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_CHECK_MOTION_HI_LW);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
            WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
        else {
            WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK){
            FT_MOTION_RATE(agent, 1.25);
        }
        else{
            FT_MOTION_RATE(agent, 1.1);
        }
        ArticleModule::shoot_exist(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_CHAKRAM, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }  
}

unsafe extern "C" fn game_specials3_1hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let end_frame = MotionModule::end_frame(boma);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.27);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_REQUEST_GENERATE);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_CHECK_MOTION_HI_LW);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
            WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
        else {
            WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK){
            FT_MOTION_RATE(agent, 1.25);
        }
        else{
            FT_MOTION_RATE(agent, 1.1);
        }
        ArticleModule::shoot_exist(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_CHAKRAM, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }     
}

unsafe extern "C" fn game_specials3_1lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let end_frame = MotionModule::end_frame(boma);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.27);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_REQUEST_GENERATE);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_CHECK_MOTION_HI_LW);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
            WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
        else {
            WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK){
            FT_MOTION_RATE(agent, 1.25);
        }
        else{
            FT_MOTION_RATE(agent, 1.1);
        }
        ArticleModule::shoot_exist(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_CHAKRAM, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }    
}

unsafe extern "C" fn game_specialairs3_1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let end_frame = MotionModule::end_frame(boma);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.27);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_REQUEST_GENERATE);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_CHECK_MOTION_HI_LW);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
            WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
        else {
            WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK){
            FT_MOTION_RATE(agent, 1.25);
        }
        else{
            FT_MOTION_RATE(agent, 1.1);
        }
        ArticleModule::shoot_exist(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_CHAKRAM, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }     
}

unsafe extern "C" fn game_specialairs3_1hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let end_frame = MotionModule::end_frame(boma);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.27);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_REQUEST_GENERATE);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_CHECK_MOTION_HI_LW);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
            WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
        else {
            WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK){
            FT_MOTION_RATE(agent, 1.25);
        }
        else{
            FT_MOTION_RATE(agent, 1.1);
        }
        ArticleModule::shoot_exist(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_CHAKRAM, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }    
}

unsafe extern "C" fn game_specialairs3_1lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let end_frame = MotionModule::end_frame(boma);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.27);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_REQUEST_GENERATE);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_CHECK_MOTION_HI_LW);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
            WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
        else {
            WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK){
            FT_MOTION_RATE(agent, 1.25);
        }
        else{
            FT_MOTION_RATE(agent, 1.1);
        }
        ArticleModule::shoot_exist(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_CHAKRAM, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }     
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specials1start", game_specials1start);
    agent.acmd("game_specialairs1start", game_specialairs1start);
    agent.acmd("game_specials1", game_specials1);
    agent.acmd("game_specialairs1", game_specialairs1);

    agent.acmd("game_specials2dash", game_specials2dash);
    agent.acmd("game_specials2attack", game_specials2attack);
    agent.acmd("effect_specials2attack", effect_specials2attack);
    agent.acmd("game_specialairs2dash", game_specialairs2dash);
    agent.acmd("game_specialairs2attack", game_specialairs2attack);
    agent.acmd("effect_specialairs2attack", effect_specialairs2attack);

    agent.acmd("game_specials3_1", game_specials3_1);
    agent.acmd("game_specials3_1hi", game_specials3_1hi);
    agent.acmd("game_specials3_1lw", game_specials3_1lw);
    agent.acmd("game_specialairs3_1", game_specialairs3_1);
    agent.acmd("game_specialairs3_1hi", game_specialairs3_1hi);
    agent.acmd("game_specialairs3_1lw", game_specialairs3_1lw);
}