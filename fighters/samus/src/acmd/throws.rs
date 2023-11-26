
use super::*;

#[acmd_script( agent = "samus", script = "game_throwb" , category = ACMD_GAME , low_priority)]
unsafe fn game_throwb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 50, 100, 0, 45, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_elec_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_elec_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
        CHECK_FINISH_CAMERA(fighter, 25, 15);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        REVERSE_LR(fighter);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    
}

#[acmd_script( agent = "samus", script = "game_throwhi", category = ACMD_GAME)]
unsafe fn game_throwhi(fighter: &mut L2CAgentBase) {
    let mut is_ice = false;
    let mut opponent_boma = fighter.module_accessor;
    if macros::is_excute(fighter) {
        is_ice = VarModule::is_flag(fighter.battle_object, vars::samus::instance::ICE_MODE);
        opponent_boma = fighter.get_grabbed_opponent_boma();
        
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 4.0, 92, 75, 0, 100, 0.0, 0.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 75, 0, 100, 0.0, 0.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 4.0, false);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true,false);
    }
    frame(fighter.lua_state_agent , 5.0);
    if macros::is_excute(fighter) {
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        macros::CHECK_FINISH_CAMERA(fighter, 1, 32);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.2);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 5.0, z: 0.0});
    }
    wait(fighter.lua_state_agent , 1.0);
    if macros::is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true,false);
        AttackModule::clear_all(fighter.module_accessor);
    }
    wait(fighter.lua_state_agent , 1.0);
    if macros::is_excute(fighter) {
        EffectModule::kill_all(opponent_boma, *EFFECT_SUB_ATTRIBUTE_NONE as u32, true, false);
    }
    frame(fighter.lua_state_agent , 9.0);
    FT_MOTION_RATE_RANGE(fighter,9.0,15.0,4.0);
    frame(fighter.lua_state_agent , 15.0);
    FT_MOTION_RATE(fighter,1.0);
    if macros::is_excute(fighter) {
        //let weapon = if is_ice {*FIGHTER_SAMUS_GENERATE_ARTICLE_MISSILE} else {*FIGHTER_SAMUS_GENERATE_ARTICLE_SUPERMISSILE};
        let weapon = *FIGHTER_SAMUS_GENERATE_ARTICLE_SUPERMISSILE;
        ArticleModule::generate_article(fighter.module_accessor, weapon, false, -1);
        ArticleModule::shoot_exist(fighter.module_accessor, weapon, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
        ArticleModule::set_visibility_whole(fighter.module_accessor, weapon, false, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST));
    }
}

#[acmd_script( agent = "samus", script = "effect_throwhi", category = ACMD_EFFECT)]
unsafe fn effect_throwhi(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent , 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent , 15.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("samus_missile_shot"), Hash40::new("haver"), 2.2, 0.379, -0.15, 0, 0, 90, 1, 0, 0, 0, 0, 0, 0, false);
    }
    wait(fighter.lua_state_agent , 1.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("samus_missile_straight_smoke"), Hash40::new("armr"), 0, 0, 4, 0, 0, 90, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent , 18.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "samus", script = "sound_throwhi", category = ACMD_SOUND)]
unsafe fn sound_throwhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent , 5.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    }
    frame(fighter.lua_state_agent , 15.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samus_special_s01"));
    }
}

#[acmd_script( agent = "samus", script = "expression_throwhi", category = ACMD_EXPRESSION)]
unsafe fn expression_throwhi(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_hide_gun") as i64);
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, true, -1);
        //ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, Hash40::new("throw_hi"), false, -1.0);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, Hash40::new("special_s"), false, -1.0);
        ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, 1.5);

        let mut is_ice = VarModule::is_flag(fighter.battle_object, vars::samus::instance::ICE_MODE);
        if is_ice {
            LinkModule::send_event_nodes(fighter.module_accessor, *LINK_NO_ARTICLE, Hash40::new_raw(0x1c5609e30f), 0);
        }
    }
    frame(fighter.lua_state_agent , 5.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
    frame(fighter.lua_state_agent , 15.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, 1.0);
    }
}

pub fn install() {
    install_acmd_scripts!(
        game_throwb,
        
        game_throwhi,
        effect_throwhi,
        sound_throwhi,
        expression_throwhi
    );
}

