
use super::*;

unsafe extern "C" fn game_attack11(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ArticleModule::generate_article_enable(boma, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_ROCKBUSTER, false, 0);
        WorkModule::on_flag(boma, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_SHOOT);
        // ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 135, 0, 40, 3.0, 0.0, 9.6, 5.5, Some(0.0), Some(3.7), Some(5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        // ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 361, 135, 0, 40, 3.0, 0.0, 8.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        // ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 361, 25, 0, 25, 2.5, 0.0, 3.2, 5.5, Some(0.0), Some(3.2), Some(8.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ArticleModule::generate_article_enable(boma, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_ROCKBUSTER, false, 0);
        WorkModule::on_flag(boma, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_SHOOT);
        // ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 60, 90, 0, 60, 3.0, 0.0, 12.6, 5.5, Some(0.0), Some(6.7), Some(5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        // ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 60, 90, 0, 60, 3.0, 0.0, 9.6, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 16.0/(25.0 - 5.0));
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

unsafe extern "C" fn game_attacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ArticleModule::generate_article_enable(boma, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_ROCKBUSTER, false, 0);
        WorkModule::on_flag(boma, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_SHOOT);
        // ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 361, 135, 0, 40, 3.0, 0.0, 9.6, 5.5, Some(0.0), Some(3.7), Some(5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        // ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 361, 135, 0, 40, 3.0, 0.0, 8.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        // ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 25, 0, 25, 2.5, 0.0, 3.2, 5.5, Some(0.0), Some(3.2), Some(8.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_busterchargeshot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0 / 7.0);
    frame(lua_state, 18.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_CHARGESHOT, false, -1);
        VarModule::off_flag(agent.battle_object, vars::rockman::status::CHARGE_SHOT_KEEP_CHARGE);
    }
}

unsafe extern "C" fn effect_busterchargeshot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 8, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("rockman_chargeshot_elec"), Hash40::new("havel"), 0, 0, -1.5, 0, 0, 0, 1, true);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("rockman_chargeshot_shot"), Hash40::new("rockman_chargeshot_shot"), Hash40::new("top"), 0, 7.2, 9, 0, 0, 0, 1, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_busterchargeshot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_rockman_smash_s02"));
    }
}

unsafe extern "C" fn expression_busterchargeshot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x1f5b14bb65), *FIGHTER_ROCKMAN_ARM_LEFT, *FIGHTER_ROCKMAN_ARMFORM_ROCKBUSTER, 5);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x1f5b14bb65), *FIGHTER_ROCKMAN_ARM_RIGHT, *FIGHTER_ROCKMAN_ARMFORM_HAND, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(lua_state, 18.0);
    if WorkModule::get_float(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_HOLD_RATE) < 1.0 {
        if is_excute(agent) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_S);
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_beams"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
    else {
        if is_excute(agent) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_M);
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_beaml"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
}

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        //VarModule::off_flag(agent.battle_object, vars::rockman::DETONATE_CRASHBOMB);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        if !WorkModule::is_flag(boma, *FIGHTER_ROCKMAN_STATUS_SPECIAL_S_WORK_ID_FLAG_ALREADY_EXIST_CRASHBOMB){
            ArticleModule::generate_article_enable(boma, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_CRASHBOMB, false, 0);
        }
        else{
            //VarModule::on_flag(agent.battle_object, vars::rockman::DETONATE_CRASHBOMB);
            ArticleModule::change_status(boma, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_CRASHBOMB, *WEAPON_ROCKMAN_CRASHBOMB_STATUS_KIND_EXPLODE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            FT_MOTION_RATE(agent, 15.0 / (44.0 - 19.0));
        }
    }
    frame(lua_state, 44.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
    }
}

unsafe extern "C" fn effect_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        if !WorkModule::is_flag(boma, *FIGHTER_ROCKMAN_STATUS_SPECIAL_S_WORK_ID_FLAG_ALREADY_EXIST_CRASHBOMB){
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("rockman_clashbomb_start"), Hash40::new("rockman_clashbomb_start"), Hash40::new("top"), 0, 7, 11, 0, 0, 0, 0.8, true, *EF_FLIP_YZ);
        }
        else{
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("rockman_clashbomb_start"), Hash40::new("rockman_clashbomb_start"), Hash40::new("top"), 0, 7, 11, 0, 0, 0, 0.8, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_COLOR(agent, 0.15, 0.55, 10.0);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("rockman_clashbomb_light"), Hash40::new("havel"), 0, 0, 1.2, 0, 0, 0, 1, true);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_ROCKMAN_STATUS_SPECIAL_S_WORK_ID_FLAG_ALREADY_EXIST_CRASHBOMB){
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 5.0, 0, 0, 0, 0, 0, 1.2, 3, 3, 3, 0, 0, 0, false);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        if !WorkModule::is_flag(boma, *FIGHTER_ROCKMAN_STATUS_SPECIAL_S_WORK_ID_FLAG_ALREADY_EXIST_CRASHBOMB){
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("rockman_clashbomb_fire"), Hash40::new("rockman_clashbomb_fire"), Hash40::new("top"), 0, 7.3, 9.8, 0, 0, 0, 1, true, *EF_FLIP_YZ);
            LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        else{
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("rockman_clashbomb_fire"), Hash40::new("rockman_clashbomb_fire"), Hash40::new("top"), 0, 7.3, 9.8, 0, 0, 0, 1, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_COLOR(agent, 0.15, 0.55, 10.0);
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("rockman_clashbomb_misfire"), Hash40::new("rockman_clashbomb_misfire"), Hash40::new("top"), 0, 7.5, 10, 0, 0, 0, 1, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_COLOR(agent, 0.15, 0.55, 10.0);
        }
    }
}

unsafe extern "C" fn game_specialairs(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        //VarModule::off_flag(agent.battle_object, vars::rockman::DETONATE_CRASHBOMB);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        if !WorkModule::is_flag(boma, *FIGHTER_ROCKMAN_STATUS_SPECIAL_S_WORK_ID_FLAG_ALREADY_EXIST_CRASHBOMB){
            ArticleModule::generate_article_enable(boma, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_CRASHBOMB, false, 0);
        }
        else{
            //VarModule::on_flag(agent.battle_object, vars::rockman::DETONATE_CRASHBOMB);
            ArticleModule::change_status(boma, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_CRASHBOMB, *WEAPON_ROCKMAN_CRASHBOMB_STATUS_KIND_EXPLODE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
        
    }
    
}

unsafe extern "C" fn effect_specialairs(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if !WorkModule::is_flag(boma, *FIGHTER_ROCKMAN_STATUS_SPECIAL_S_WORK_ID_FLAG_ALREADY_EXIST_CRASHBOMB){
            EFFECT_FOLLOW(agent,  Hash40::new("rockman_clashbomb_start"), Hash40::new("top"), 0, 7, 10, 0, -90, 0, 0.62, true);
        }
        else{
            EFFECT_FOLLOW(agent,  Hash40::new("rockman_clashbomb_start"), Hash40::new("top"), 0, 7, 10, 0, -90, 0, 0.62, true);
            LAST_EFFECT_SET_COLOR(agent, 0.15, 0.55, 10.0);
        }
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_ROCKMAN_STATUS_SPECIAL_S_WORK_ID_FLAG_ALREADY_EXIST_CRASHBOMB){
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 5.0, 0, 0, 0, 0, 0, 1.2, 3, 3, 3, 0, 0, 0, false);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        if !WorkModule::is_flag(boma, *FIGHTER_ROCKMAN_STATUS_SPECIAL_S_WORK_ID_FLAG_ALREADY_EXIST_CRASHBOMB){
            EFFECT_FOLLOW(agent, Hash40::new("rockman_clashbomb_fire"), Hash40::new("top"), 0, 7.5, 11, 0, -0.0, 0, 1, true);
        }
        else{
            EFFECT_FOLLOW(agent, Hash40::new("rockman_clashbomb_fire"), Hash40::new("top"), 0, 7.5, 11, 0, -0.0, 0, 1, true);
            LAST_EFFECT_SET_COLOR(agent, 0.15, 0.55, 10.0);
            EFFECT_FOLLOW(agent, Hash40::new("rockman_clashbomb_fire"), Hash40::new("top"), 0, 7.5, 11, 0, -0.0, 0, 1, true);
            LAST_EFFECT_SET_COLOR(agent, 0.15, 0.55, 10.0);
        }
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    FT_MOTION_RATE(agent, 5.0);
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ArticleModule::generate_article_enable(agent.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_LEAFSHIELD, false, -1);
    }
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    FT_MOTION_RATE(agent, 5.0);
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ArticleModule::generate_article_enable(agent.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_LEAFSHIELD, false, -1);
    }
    FT_MOTION_RATE(agent, 1.0);
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attack11", game_attack11);

    agent.acmd("game_attackairn", game_attackairn);

    agent.acmd("game_attacks3", game_attacks3);

    agent.acmd("game_busterchargeshot", game_busterchargeshot);
    agent.acmd("effect_busterchargeshot", effect_busterchargeshot);
    agent.acmd("sound_busterchargeshot", sound_busterchargeshot);
    agent.acmd("expression_busterchargeshot", expression_busterchargeshot);

    agent.acmd("game_busterairchargeshot", game_busterchargeshot);
    agent.acmd("effect_busterairchargeshot", effect_busterchargeshot);
    agent.acmd("sound_busterairchargeshot", sound_busterchargeshot);
    agent.acmd("expression_busterairchargeshot", expression_busterchargeshot);

    agent.acmd("game_specials", game_specials);
    agent.acmd("effect_specials", effect_specials);

    agent.acmd("game_specialairs", game_specialairs);
    agent.acmd("effect_specialairs", effect_specialairs);

    agent.acmd("game_speciallw", game_speciallw);

    agent.acmd("game_specialairlw", game_specialairlw);
}
