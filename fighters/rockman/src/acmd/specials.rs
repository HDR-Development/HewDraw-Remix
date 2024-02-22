
use super::*;

unsafe extern "C" fn rockman_attack_11_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ArticleModule::generate_article_enable(boma, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_ROCKBUSTER, false, 0);
        WorkModule::on_flag(boma, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_SHOOT);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 135, 0, 40, 3.0, 0.0, 9.6, 5.5, Some(0.0), Some(3.7), Some(5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 361, 135, 0, 40, 3.0, 0.0, 8.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 361, 25, 0, 25, 2.5, 0.0, 3.2, 5.5, Some(0.0), Some(3.2), Some(8.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn rockman_attack_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ArticleModule::generate_article_enable(boma, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_ROCKBUSTER, false, 0);
        WorkModule::on_flag(boma, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_SHOOT);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 60, 90, 0, 60, 3.0, 0.0, 12.6, 5.5, Some(0.0), Some(6.7), Some(5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 60, 90, 0, 60, 3.0, 0.0, 9.6, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 16.0/(25.0 - 5.0));
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

unsafe extern "C" fn rockman_attack_s3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ArticleModule::generate_article_enable(boma, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_ROCKBUSTER, false, 0);
        WorkModule::on_flag(boma, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_SHOOT);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 135, 0, 40, 3.0, 0.0, 9.6, 5.5, Some(0.0), Some(3.7), Some(5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 361, 135, 0, 40, 3.0, 0.0, 8.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 361, 25, 0, 25, 2.5, 0.0, 3.2, 5.5, Some(0.0), Some(3.2), Some(8.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn rockman_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    FT_MOTION_RATE(agent, 1.0 / 7.0);
    frame(agent.lua_state_agent, 18.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_CHARGESHOT, false, -1);
        VarModule::off_flag(agent.battle_object, vars::rockman::status::CHARGE_SHOT_KEEP_CHARGE);
    }
}

unsafe extern "C" fn rockman_specialn_eff(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 8, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("rockman_chargeshot_elec"), Hash40::new("havel"), 0, 0, -1.5, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("rockman_chargeshot_shot"), Hash40::new("rockman_chargeshot_shot"), Hash40::new("top"), 0, 7.2, 9, 0, 0, 0, 1, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn rockman_specialn_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_rockman_smash_s02"));
    }
}

unsafe extern "C" fn rockman_specialn_exp(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x1f5b14bb65), *FIGHTER_ROCKMAN_ARM_LEFT, *FIGHTER_ROCKMAN_ARMFORM_ROCKBUSTER, 5);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x1f5b14bb65), *FIGHTER_ROCKMAN_ARM_RIGHT, *FIGHTER_ROCKMAN_ARMFORM_HAND, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 18.0);
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

unsafe extern "C" fn rockman_chargeshot_regular(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        let is_charge_max = 1.0 <= WorkModule::get_float(agent.module_accessor, *WEAPON_ROCKMAN_CHARGESHOT_INSTANCE_WORK_ID_FLOAT_HOLD_RATE);
        let damage;
        let bkb;
        let kbg;
        if is_charge_max {
            damage = 15.0;
            bkb = 40;
            kbg = 90;
        }
        else {
            damage = 9.0;
            bkb = 50;
            kbg = 85;
        }
        ATTACK(agent, 0, 0, Hash40::new("top"), damage, 361, kbg, 0, bkb, 2.6, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.32);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
}

unsafe extern "C" fn rockman_special_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        //VarModule::off_flag(fighter.battle_object, vars::rockman::DETONATE_CRASHBOMB);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        if !WorkModule::is_flag(boma, *FIGHTER_ROCKMAN_STATUS_SPECIAL_S_WORK_ID_FLAG_ALREADY_EXIST_CRASHBOMB){
            ArticleModule::generate_article_enable(boma, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_CRASHBOMB, false, 0);
        }
        else{
            //VarModule::on_flag(fighter.battle_object, vars::rockman::DETONATE_CRASHBOMB);
            ArticleModule::change_status(boma, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_CRASHBOMB, *WEAPON_ROCKMAN_CRASHBOMB_STATUS_KIND_EXPLODE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            FT_MOTION_RATE(fighter, 15.0 / (44.0 - 19.0));
        }
    }
    frame(lua_state, 44.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
}

unsafe extern "C" fn rockman_special_s_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        if !WorkModule::is_flag(boma, *FIGHTER_ROCKMAN_STATUS_SPECIAL_S_WORK_ID_FLAG_ALREADY_EXIST_CRASHBOMB){
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("rockman_clashbomb_start"), Hash40::new("rockman_clashbomb_start"), Hash40::new("top"), 0, 7, 11, 0, 0, 0, 0.8, true, *EF_FLIP_YZ);
        }
        else{
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("rockman_clashbomb_start"), Hash40::new("rockman_clashbomb_start"), Hash40::new("top"), 0, 7, 11, 0, 0, 0, 0.8, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("rockman_clashbomb_light"), Hash40::new("havel"), 0, 0, 1.2, 0, 0, 0, 1, true);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_ROCKMAN_STATUS_SPECIAL_S_WORK_ID_FLAG_ALREADY_EXIST_CRASHBOMB){
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 5.0, 0, 0, 0, 0, 0, 1.2, 3, 3, 3, 0, 0, 0, false);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        if !WorkModule::is_flag(boma, *FIGHTER_ROCKMAN_STATUS_SPECIAL_S_WORK_ID_FLAG_ALREADY_EXIST_CRASHBOMB){
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("rockman_clashbomb_fire"), Hash40::new("rockman_clashbomb_fire"), Hash40::new("top"), 0, 7.3, 9.8, 0, 0, 0, 1, true, *EF_FLIP_YZ);
            LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        else{
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("rockman_clashbomb_fire"), Hash40::new("rockman_clashbomb_fire"), Hash40::new("top"), 0, 7.3, 9.8, 0, 0, 0, 1, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0);
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("rockman_clashbomb_misfire"), Hash40::new("rockman_clashbomb_misfire"), Hash40::new("top"), 0, 7.5, 10, 0, 0, 0, 1, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0);
        }
    }
}

unsafe extern "C" fn rockman_special_air_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        //VarModule::off_flag(fighter.battle_object, vars::rockman::DETONATE_CRASHBOMB);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        if !WorkModule::is_flag(boma, *FIGHTER_ROCKMAN_STATUS_SPECIAL_S_WORK_ID_FLAG_ALREADY_EXIST_CRASHBOMB){
            ArticleModule::generate_article_enable(boma, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_CRASHBOMB, false, 0);
        }
        else{
            //VarModule::on_flag(fighter.battle_object, vars::rockman::DETONATE_CRASHBOMB);
            ArticleModule::change_status(boma, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_CRASHBOMB, *WEAPON_ROCKMAN_CRASHBOMB_STATUS_KIND_EXPLODE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
        
    }
    
}

unsafe extern "C" fn rockman_special_air_s_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        if !WorkModule::is_flag(boma, *FIGHTER_ROCKMAN_STATUS_SPECIAL_S_WORK_ID_FLAG_ALREADY_EXIST_CRASHBOMB){
            EFFECT_FOLLOW(fighter,  Hash40::new("rockman_clashbomb_start"), Hash40::new("top"), 0, 7, 10, 0, -90, 0, 0.62, true);
        }
        else{
            EFFECT_FOLLOW(fighter,  Hash40::new("rockman_clashbomb_start"), Hash40::new("top"), 0, 7, 10, 0, -90, 0, 0.62, true);
            LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0);
        }
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_ROCKMAN_STATUS_SPECIAL_S_WORK_ID_FLAG_ALREADY_EXIST_CRASHBOMB){
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 5.0, 0, 0, 0, 0, 0, 1.2, 3, 3, 3, 0, 0, 0, false);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        if !WorkModule::is_flag(boma, *FIGHTER_ROCKMAN_STATUS_SPECIAL_S_WORK_ID_FLAG_ALREADY_EXIST_CRASHBOMB){
            EFFECT_FOLLOW(fighter, Hash40::new("rockman_clashbomb_fire"), Hash40::new("top"), 0, 7.5, 11, 0, -0.0, 0, 1, true);
        }
        else{
            EFFECT_FOLLOW(fighter, Hash40::new("rockman_clashbomb_fire"), Hash40::new("top"), 0, 7.5, 11, 0, -0.0, 0, 1, true);
            LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0);
            EFFECT_FOLLOW(fighter, Hash40::new("rockman_clashbomb_fire"), Hash40::new("top"), 0, 7.5, 11, 0, -0.0, 0, 1, true);
            LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0);
        }
    }
}

unsafe extern "C" fn rockman_speciallw(agent: &mut L2CAgentBase) {
    FT_MOTION_RATE(agent, 5.0);
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        ArticleModule::generate_article_enable(agent.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_LEAFSHIELD, false, -1);
    }
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn rockman_specialairlw(agent: &mut L2CAgentBase) {
    FT_MOTION_RATE(agent, 5.0);
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        ArticleModule::generate_article_enable(agent.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_LEAFSHIELD, false, -1);
    }
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn rockman_leafshield_start(agent: &mut L2CAgentBase) {
    FT_MOTION_RATE(agent, 4.0 / 3.0);
    if is_excute(agent) {
        if !WorkModule::is_flag(agent.module_accessor, 0x20000006) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_0
            ATTACK(agent, 0, 0, Hash40::new("leafshield1"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(agent.module_accessor, 0x20000007) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_1
            ATTACK(agent, 1, 0, Hash40::new("leafshield2"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(agent.module_accessor, 0x20000008) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_2
            ATTACK(agent, 2, 0, Hash40::new("leafshield3"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(agent.module_accessor, 0x20000009) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_3
            ATTACK(agent, 3, 0, Hash40::new("leafshield4"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
    }
}

unsafe extern "C" fn rockman_leafshield_shield(agent: &mut L2CAgentBase) {
    FT_MOTION_RATE(agent, 4.0 / 3.0);
    if is_excute(agent) {
        if !WorkModule::is_flag(agent.module_accessor, 0x20000006) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_0
            ATTACK(agent, 0, 0, Hash40::new("leafshield1"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(agent.module_accessor, 0x20000007) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_1
            ATTACK(agent, 1, 0, Hash40::new("leafshield2"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(agent.module_accessor, 0x20000008) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_2
            ATTACK(agent, 2, 0, Hash40::new("leafshield3"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(agent.module_accessor, 0x20000009) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_3
            ATTACK(agent, 3, 0, Hash40::new("leafshield4"), 1.5, 361, 20, 0, 35, 1.3, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
    }
}

unsafe extern "C" fn rockman_leafshield_fly(agent: &mut L2CAgentBase) {
    FT_MOTION_RATE(agent, 4.0 / 3.0);
    if is_excute(agent) {
        if !WorkModule::is_flag(agent.module_accessor, 0x20000006) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_0
            ATTACK(agent, 0, 0, Hash40::new("leafshield1"), 3.8, 65, 70, 0, 50, 1.8, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 11, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(agent.module_accessor, 0x20000007) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_1
            ATTACK(agent, 1, 0, Hash40::new("leafshield2"), 3.8, 65, 70, 0, 50, 1.8, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 11, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(agent.module_accessor, 0x20000008) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_2
            ATTACK(agent, 2, 0, Hash40::new("leafshield3"), 3.8, 65, 70, 0, 50, 1.8, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 11, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        if !WorkModule::is_flag(agent.module_accessor, 0x20000009) { // WEAPON_ROCKMAN_LEAFSHIELD_INSTANCE_WORK_ID_FLAG_DEAD_3
            ATTACK(agent, 3, 0, Hash40::new("leafshield4"), 3.8, 65, 70, 0, 50, 1.8, -1.0, -0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 11, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
    }
}

pub fn install() {
    smashline::Agent::new("rockman_leafshield")
        .acmd("game_start", rockman_leafshield_start)
        .acmd("game_startreverse", rockman_leafshield_start)
        .acmd("game_shield", rockman_leafshield_shield)
        .acmd("game_shieldreverse", rockman_leafshield_shield)
        .acmd("game_fly", rockman_leafshield_fly)
        .acmd("game_flyreverse", rockman_leafshield_fly)
        .install();
    smashline::Agent::new("rockman")
        .acmd("game_attack11", rockman_attack_11_game)
        .acmd("game_attackairn", rockman_attack_air_n_game)
        .acmd("game_attacks3", rockman_attack_s3_game)
        .acmd("game_busterchargeshot", rockman_specialn)
        .acmd("game_busterairchargeshot", rockman_specialn)
        .acmd("effect_busterchargeshot", rockman_specialn_eff)
        .acmd("effect_busterairchargeshot", rockman_specialn_eff)
        .acmd("sound_busterchargeshot", rockman_specialn_snd)
        .acmd("sound_busterairchargeshot", rockman_specialn_snd)
        .acmd("expression_busterchargeshot", rockman_specialn_exp)
        .acmd("expression_busterairchargeshot", rockman_specialn_exp)
        .acmd("game_specials", rockman_special_s_game)
        .acmd("effect_specials", rockman_special_s_effect)
        .acmd("game_specialairs", rockman_special_air_s_game)
        .acmd("effect_specialairs", rockman_special_air_s_effect)
        .acmd("game_speciallw", rockman_speciallw)
        .acmd("game_specialairlw", rockman_specialairlw)
        .install();
    smashline::Agent::new("rockman_chargeshot")
        .acmd("game_regular", rockman_chargeshot_regular)
        .install();
}
