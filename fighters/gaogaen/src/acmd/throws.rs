
use super::*;

#[acmd_script( agent = "gaogaen", script = "game_throwb" , category = ACMD_GAME , low_priority)]
unsafe fn gaogaen_throw_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
         VarModule::off_flag(boma.object(), vars::common::IS_HEAVY_ATTACK);
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH){
            VarModule::on_flag(boma.object(), vars::common::IS_HEAVY_ATTACK);
        }
        // Kill throw
        if VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK){
            ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 40, 490, 0, 20, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        // Techchase throw
        else{
            ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 32, 60, 0, 85, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(fighter, 15.0/(16.0-11.0));
        }
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(fighter, 1.0);
        }
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        // Kill throw
        if VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK){
            ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 32, 80, 0, 60, 7.0, 0.0, 2.7, 2.6, Some(0.0), Some(2.7), Some(-3.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        }
        // Techchase throw
        else{
            ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 32, 80, 0, 60, 7.0, 0.0, 2.7, 2.6, Some(0.0), Some(2.7), Some(-3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
        AttackModule::set_catch_only_all(boma, true, false);
        CHECK_FINISH_CAMERA(fighter, -4, 5);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(fighter, 1.0);
        }
        else{
            FT_MOTION_RATE(fighter, 0.9);
        }
    }
    frame(lua_state, 50.0);
    if is_excute(fighter) {
        REVERSE_LR(fighter);
    }
}

#[acmd_script( agent = "gaogaen", script = "effect_throwb" , category = ACMD_EFFECT , low_priority)]
unsafe fn gaogaen_throw_b_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK){
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0.0, 13.0, 20.0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(fighter, 0.8);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK){
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("bust"), 0.0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_RATE(fighter, 0.5);
            
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 1.0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_RATE(fighter, 0.5);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 7.0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_RATE(fighter, 0.5);

            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 1.0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_RATE(fighter, 0.5);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 7.0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_RATE(fighter, 0.5);

            EFFECT_FOLLOW(fighter, Hash40::new("gaogaen_belt_fire_appeal"), Hash40::new("feeler"), 0, 3, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(fighter, 1.25);

            EFFECT(fighter, Hash40::new("sys_hit_fire"), Hash40::new("top"), 0.0, 8.0, 10.0, 0, 0, 0, 0.35, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(fighter, 1.25);
        }
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(lua_state, 27.0);
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        if VarModule::is_flag(boma.object(), vars::common::IS_HEAVY_ATTACK){
            EFFECT(fighter, Hash40::new("sys_bomb_b"), Hash40::new("top"), 10, 0, -5.0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

#[acmd_script( agent = "gaogaen", script = "game_throwhi" , category = ACMD_GAME , low_priority)]
unsafe fn gaogaen_throw_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 11.0, 88, 70, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        //FT_CATCH_STOP(fighter, 14, 1);
        CHECK_FINISH_CAMERA(fighter, 1, 20);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 88, 70, 0, 65, 7.0, 0.0, 10.0, 0.0, Some(0.0), Some(10.0), Some(0.0), 4.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(boma, true, false);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        FT_MOTION_RATE(fighter, 0.9);
    }
}


pub fn install() {
    install_acmd_scripts!(
        gaogaen_throw_b_game,
        gaogaen_throw_b_effect,
        gaogaen_throw_hi_game,
    );
}

