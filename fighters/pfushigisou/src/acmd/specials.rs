
use super::*;


#[acmd_script( agent = "pfushigisou", script = "game_specials" , category = ACMD_GAME , low_priority)]
unsafe fn pfushigisou_special_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.battle_object, vars::common::SPECIAL_PROJECTILE_SPAWNED) {
            ArticleModule::generate_article(boma, *FIGHTER_PFUSHIGISOU_GENERATE_ARTICLE_LEAFCUTTER, false, 0);
        }
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.900);
    }
    frame(lua_state, 49.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
    }
    
}

#[acmd_script( agent = "pfushigisou", script = "game_specialairs" , category = ACMD_GAME , low_priority)]
unsafe fn pfushigisou_special_air_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_PFUSHIGISOU_GENERATE_ARTICLE_LEAFCUTTER, false, 0);
        VarModule::on_flag(fighter.battle_object, vars::common::SPECIAL_PROJECTILE_SPAWNED);
     }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.900);
    }
    frame(lua_state, 49.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
    }
    
}

#[acmd_script( agent = "pfushigisou", script = "game_specialhi" , category = ACMD_GAME , low_priority)]
unsafe fn pfushigisou_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PFUSHIGISOU_STATUS_SPECIAL_HI_FLAG_SET_ANGLE);
        ATTACK(fighter, 0, 0, Hash40::new("viner2"), 11.0, 50, 85, 0, 50, 4.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("viner3"), 11.0, 50, 85, 0, 50, 4.2, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 2, 0, Hash40::new("viner4"), 11.0, 50, 85, 0, 50, 3.8, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 3, 0, Hash40::new("viner5"), 11.0, 50, 85, 0, 50, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 4, 0, Hash40::new("viner5"), 13.0, 50, 87, 0, 70, 6.5, 8.8, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "pfushigisou", script = "game_specialairhi" , category = ACMD_GAME , low_priority)]
unsafe fn pfushigisou_special_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_PFUSHIGISOU_GENERATE_ARTICLE_VINE, false, 0);
        ArticleModule::set_visibility_whole(boma, *FIGHTER_PFUSHIGISOU_GENERATE_ARTICLE_VINE, false, app::ArticleOperationTarget(0));
        WorkModule::on_flag(boma, *FIGHTER_PFUSHIGISOU_STATUS_SPECIAL_HI_SET_MAP_COLL_OFFSET);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PFUSHIGISOU_STATUS_SPECIAL_HI_FLAG_SET_ANGLE);
        ATTACK(fighter, 0, 0, Hash40::new("viner2"), 11.0, 50, 85, 0, 50, 5.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("viner3"), 11.0, 50, 85, 0, 50, 4.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 2, 0, Hash40::new("viner4"), 11.0, 50, 85, 0, 50, 4.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 3, 0, Hash40::new("viner5"), 11.0, 50, 85, 0, 50, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 4, 0, Hash40::new("viner5"), 13.0, 50, 87, 0, 70, 6.5, 8.8, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    
}

#[acmd_script( agent = "pfushigisou", script = "game_solar_beam", category = ACMD_GAME, low_priority)]
unsafe fn solar_beam_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.boma(), 6.0);
    }
    
    frame(lua_state, 65.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.boma(), 1.0);
    }
    frame(lua_state, 70.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 50, 87, 0, 70, 3.5, 0.0, 11.0, 14.0, Some(0.0), Some(111.0), Some(114.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 20.0);
    if is_excute(fighter) {
        MotionModule::set_frame_sync_anim_cmd(fighter.boma(), 230.0, true, true, false);
        MotionModule::set_rate(fighter.boma(), 2.0);
        AttackModule::clear_all(fighter.boma());
    }
}

#[acmd_script( agent = "pfushigisou", script = "effect_solar_beam", category = ACMD_EFFECT, low_priority)]
unsafe fn solar_beam_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 70.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("finptrainer_solar_beam"), Hash40::new("top"), 0, 8.5, 11, -45, 0, 0, 1, true);
        EffectModule::set_scale_last(fighter.boma(), &Vector3f::new(0.66, 1.0, 0.66));
        LAST_EFFECT_SET_RATE(fighter, 10);
    }
    wait(lua_state, 15.0);
    if is_excute(fighter) {
        LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
}

#[acmd_script( agent = "pfushigisou", script = "sound_solar_beam", category = ACMD_SOUND, low_priority)]
unsafe fn solar_beam_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 70.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_step_electrop"));
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        pfushigisou_special_s_game,
        pfushigisou_special_air_s_game,
        pfushigisou_special_hi_game,
        pfushigisou_special_air_hi_game,
        solar_beam_effect,
        solar_beam_game,
        solar_beam_sound
    );
}

