
use super::*;

#[acmd_script( agent = "samus", script = "game_specialhi" , category = ACMD_GAME , low_priority)]
unsafe fn special_hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        boma.select_cliff_hangdata_from_name("special_hi");
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_ACC_X);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 92, 100, 200, 0, 3.2, 0.0, 0.0, 5.0, Some(0.0), Some(0.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 105, 100, 200, 0, 3.2, 0.0, 0.0, -5.0, Some(0.0), Some(0.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 92, 100, 200, 0, 3.2, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 3.0, 105, 100, 200, 0, 3.2, 0.0, 9.0, -5.0, Some(0.0), Some(9.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 92, 100, 180, 0, 3.2, 0.0, 2.0, 5.0, Some(0.0), Some(2.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 98, 100, 180, 0, 3.2, 0.0, 2.0, -5.0, Some(0.0), Some(2.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 92, 100, 80, 0, 3.0, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 98, 100, 80, 0, 3.0, 0.0, 9.0, -5.0, Some(0.0), Some(9.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 84, 100, 100, 0, 3.2, 0.0, 2.0, 5.0, Some(0.0), Some(2.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 90, 100, 100, 0, 3.2, 0.0, 2.0, -5.0, Some(0.0), Some(2.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 84, 100, 40, 0, 3.0, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 90, 100, 40, 0, 3.0, 0.0, 9.0, -5.0, Some(0.0), Some(9.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 82, 100, 40, 0, 3.2, 0.0, 2.0, 5.0, Some(0.0), Some(2.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 90, 100, 40, 0, 3.2, 0.0, 2.0, -5.0, Some(0.0), Some(2.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 82, 100, 20, 0, 3.0, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 90, 100, 20, 0, 3.0, 0.0, 9.0, -5.0, Some(0.0), Some(9.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 70, 200, 0, 56, 10.0, 0.0, 6.5, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SAMUS_SCREW_FINISH, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    
}

#[acmd_script( agent = "samus", script = "game_specialairhi" , category = ACMD_GAME , low_priority)]
unsafe fn special_air_hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        boma.select_cliff_hangdata_from_name("special_hi");
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 93, 100, 115, 0, 3.2, 0.0, 2.0, 5.0, Some(0.0), Some(2.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 96, 100, 115, 0, 3.2, 0.0, 2.0, -5.0, Some(0.0), Some(2.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 93, 100, 60, 0, 3.0, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 96, 100, 60, 0, 3.0, 0.0, 9.0, -5.0, Some(0.0), Some(9.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 96, 100, 70, 0, 3.2, 0.0, 2.0, 5.0, Some(0.0), Some(2.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 98, 100, 70, 0, 3.2, 0.0, 2.0, -5.0, Some(0.0), Some(2.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 96, 100, 40, 0, 3.0, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 98, 100, 40, 0, 3.0, 0.0, 9.0, -5.0, Some(0.0), Some(9.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 70, 250, 0, 50, 10.0, 0.0, 6.5, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SAMUS_SCREW_FINISH, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    
}

#[acmd_script( agent = "samus", scripts = ["game_specialnstart","game_specialairnstart"], category = ACMD_GAME, low_priority)]
unsafe fn game_specialnstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent , 14.0);
    if macros::is_excute(fighter) {
        if !VarModule::is_flag(fighter.battle_object, vars::samus::instance::ICE_MODE) {
            ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, false, -1);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_BULLET_DISP);
        }
    }
}
#[acmd_script( agent = "samus", scripts = ["effect_specialnhold","effect_specialairnhold"], category = ACMD_EFFECT, low_priority)]
unsafe fn effect_specialnhold(fighter: &mut L2CAgentBase) {
    let mut is_ice = false;
    frame(fighter.lua_state_agent , 1.0);
    if macros::is_excute(fighter) {
        is_ice = VarModule::is_flag(fighter.battle_object, vars::samus::instance::ICE_MODE);
        if !is_ice {
            EFFECT_FOLLOW(fighter, Hash40::new("samus_cshot_hold"), Hash40::new("armr"), 7.98, -0.506, -0.251, -91.273, -1.797, 176.373, 1, true);
            EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        }
        else{
            EFFECT_FOLLOW(fighter, Hash40::new("samus_cshot_hold"), Hash40::new("armr"), 7.98, -0.506, -0.251, -91.273, -1.797, 176.373, 0.75, true);
            EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
            LAST_EFFECT_SET_COLOR(fighter,0.25, 1.5,1.0);

            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_ice"), Hash40::new("armr"), 7.0, 0.0, 0.0, 0, 0, -90, 0.075, true);
        }
    }
    frame(fighter.lua_state_agent , 2.0);
    if macros::is_excute(fighter) {
        if MotionModule::motion_kind(fighter.module_accessor) == Hash40::new("effect_specialairnhold").hash {
            for i in 0..100 {
                if macros::is_excute(fighter) {
                    macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 1, 12, 0, 12, 0, 0, 0, false);
                    if is_ice {
                        //let size = 1.0+(i as f32 / 6.0);
                        //macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_starrod_splash"), Hash40::new("armr"), 7.5, 0, 0, 0, 0, 0, size, true);
                        //LAST_EFFECT_SET_COLOR(fighter,0.375, 1.0,1.0);
                    }
                }
                wait(fighter.lua_state_agent , 12.0);
            }
        }
    }
}
#[acmd_script( agent = "samus", scripts = ["sound_specialnhold","sound_specialairnhold"], category = ACMD_SOUND, low_priority)]
unsafe fn sound_specialnhold(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent , 1.0);
    if macros::is_excute(fighter) {
        if !VarModule::is_flag(fighter.battle_object, vars::samus::instance::ICE_MODE) {
            macros::PLAY_STATUS(fighter, Hash40::new("se_samus_special_n01"));
        }
        else{
            macros::PLAY_STATUS(fighter, Hash40::new("se_samus_special_n01"));
        }
    }
}

#[acmd_script( agent = "samus", scripts = ["game_specialnice","game_specialairnice"], category = ACMD_GAME, low_priority)]
unsafe fn game_specialnice(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent , 3.0);
    let mut damage=1.0 as f32;
    let mut size=1.0 as f32;
    let mut kbg=1.0 as f32;
    let mut bkb=1.0 as f32;
    if macros::is_excute(fighter) {
        let charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT) as f32;
        let charge_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("cshot_charge_frame")) as f32;
        let c = charge / charge_max;

        use interpolation::Lerp;

        let length = Lerp::lerp(&0.0,&9.0,&c);
        damage = Lerp::lerp(&7.0,&21.0,&c);
        let angle = if c < 0.5 {361 as u64} else {45 as u64};
        kbg = Lerp::lerp(&35.0,&70.0,&c);
        bkb = Lerp::lerp(&35.0,&55.0,&c);
        size = Lerp::lerp(&1.5,&5.0,&c);
        if c < 1.0 {
            let level = if c < 0.5 {*ATTACK_SOUND_LEVEL_S} else {*ATTACK_SOUND_LEVEL_M};
            let offset = 13.0-((1.0-c)*7.5);

            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), damage as f32, angle, kbg as i32, 0, bkb as i32, size, 0.0, 10.0, offset as f32, Some(0.0), Some(10.0), Some(14.0+length), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -1.0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), level, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
        }
        else{
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), damage as f32, angle, (kbg as i32)-10, 0, (bkb as i32)-20, size as f32, -1.0, 10.0, 14.0, Some(0.0), Some(10.0), Some(14.0+length), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_ENERGY);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), damage as f32, angle, (kbg as i32)-10, 0, (bkb as i32)-20, size as f32, -1.0, 10.0, 14.0, Some(0.0), Some(10.0), Some(14.0+length), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_ENERGY);
        }
    }
    wait(fighter.lua_state_agent , 3.0);
    if macros::is_excute(fighter) {
        AttackModule::set_size(fighter.module_accessor,0,(size as f32)*1.25 );
    }
    wait(fighter.lua_state_agent , 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}
#[acmd_script( agent = "samus", scripts = ["effect_specialnice","effect_specialairnice"], category = ACMD_EFFECT, low_priority)]
unsafe fn effect_specialnice(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent , 3.0);
    if macros::is_excute(fighter) {
        let charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT) as f64;
        let charge_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("cshot_charge_frame")) as f64;
        let c = charge / charge_max;
        use interpolation::Lerp;

        let length = Lerp::lerp(&1.0,&3.0,&c);
        EFFECT(fighter, Hash40::new("sys_muzzleflash"), Hash40::new("armr"), 7.9, 0.0, 0.0, 0, 0, 0, length/1.25, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter,0.25, 0.875,1.0);
        LAST_EFFECT_SET_RATE(fighter,0.75);
        LAST_EFFECT_SET_SCALE_W(fighter,length/2.0,length,length/2.0);

        if c >= 0.5 {
            macros::EFFECT(fighter, Hash40::new("samus_cshot_shot"), Hash40::new("top"), 6, 6, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(fighter.lua_state_agent , 4.0);
    if macros::is_excute(fighter) {
        if MotionModule::motion_kind(fighter.module_accessor) == Hash40::new("effect_specialairnice").hash {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::FLASH(fighter, 1, 0.753, 1, 0.706);
        }
    }
    frame(fighter.lua_state_agent , 5.0);
    if macros::is_excute(fighter) {
        macros::FLASH_FRM(fighter, 10, 0.314, 0.314, 0.314, 0);
    }
    frame(fighter.lua_state_agent , 15.0);
    if macros::is_excute(fighter) {
        macros::COL_NORMAL(fighter);
    }
}
#[acmd_script( agent = "samus", scripts = ["sound_specialnice","sound_specialairnice"], category = ACMD_SOUND, low_priority)]
unsafe fn sound_specialnice(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_samus_special_n01"));
    }
    frame(fighter.lua_state_agent , 3.0);
    if macros::is_excute(fighter) {
        let charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT) as f32;
        let charge_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("cshot_charge_frame"));
        let c = charge / charge_max;
        if c <= 0.25 {
            macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_samus_special_n02"));
        }
        else if c <= 0.625 {
            macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_samus_special_n03"));
        }
        else if c <= 0.875 {
            macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_samus_special_n04"));
        }
        else {
            macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_samus_special_n05"));
        }
    }
}
#[acmd_script( agent = "samus", scripts = ["expression_specialnice","expression_specialairnice"], category = ACMD_EXPRESSION, low_priority)]
unsafe fn expression_specialnice(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent , 1.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    install_acmd_scripts!(
        special_hi,
        special_air_hi,
               
        game_specialnstart,
        effect_specialnhold,
        sound_specialnhold,

        game_specialnice,
        effect_specialnice,
        sound_specialnice,
        expression_specialnice,
    );
}

