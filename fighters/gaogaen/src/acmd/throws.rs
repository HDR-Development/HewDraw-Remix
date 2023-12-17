
use super::*;

#[acmd_script( agent = "gaogaen", script = "game_throwf" , category = ACMD_GAME , low_priority)]
unsafe fn gaogaen_throw_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    VarModule::off_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK);
    if WorkModule::is_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
        VarModule::on_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK);
    }
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 11.0, 40, 58, 0, 82, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("hip"), 4.0, 30, 63, 0, 100, 8.0, -3.0, 7.0, 0.0, Some(1.0), Some(6.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("hip"), 6.0, 30, 62, 0, 100, 8.0, -4.5, 14.0, 0.0, Some(1.0), Some(6.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("hip"), 8.0, 30, 61, 0, 100, 8.0, -6.0, 22.0, 0.0, Some(1.0), Some(6.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 48.0);
    if is_excute(fighter) {
        ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("hip"), 10.0, 30, 60, 0, 100, 8.0, -6.0, 22.0, 0.0, Some(1.0), Some(6.0), Some(0.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 57.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        CHECK_FINISH_CAMERA(fighter, 16, 15);
    }
    frame(lua_state, 58.0);
    if is_excute(fighter) {
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

#[acmd_script( agent = "gaogaen", script = "effect_throwf" , category = ACMD_EFFECT , low_priority)]
unsafe fn gaogaen_throw_f_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0.0, 13.0, 20.0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(fighter, 0.8);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
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
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, -3, 0, 180, -90, 0, 1.6, true, *EF_FLIP_YZ, 1.0);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, 1, 0, 180, -90, 0, 1.8, true, *EF_FLIP_YZ, 1.0);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, -2, 0, 180, 0, 0, 1.7, true, *EF_FLIP_YZ, 1.0);
    }
    frame(lua_state, 41.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, 0, 0, 180, 0, 0, 2, true, *EF_FLIP_YZ, 1.0);
    }
    frame(lua_state, 43.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 48.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, 1.5, 0, 180, 0, 0, 2.3, true, *EF_FLIP_YZ, 1.0);
    }
    frame(lua_state, 58.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 59.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 20, 13, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 360, true, 0.7);
        LAST_EFFECT_SET_RATE(fighter, 0.6);
    }
}

#[acmd_script( agent = "gaogaen", script = "game_throwb" , category = ACMD_GAME , low_priority)]
unsafe fn gaogaen_throw_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VarModule::off_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK);
        if WorkModule::is_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
            VarModule::on_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK);
        }
        // Kill throw
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
            ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 40, 490, 0, 20, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        // Techchase throw
        else{
            ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 275, 100, 30, 0, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(fighter, 15.0/(16.0-11.0));
        }
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(fighter, 1.0);
        }
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        // Kill throw
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
            ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 32, 80, 0, 60, 7.0, 0.0, 2.7, 2.6, Some(0.0), Some(2.7), Some(-3.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
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
        if !VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK) {
            ModelModule::set_joint_translate(boma, Hash40::new("throw"), &Vector3f{x: 2.439, y: -6.660, z: -5.0}, false, false);
            let opponent_boma = fighter.get_grabbed_opponent_boma();
            VarModule::on_flag(opponent_boma.object(), vars::common::instance::IS_KNOCKDOWN_THROW);
        }
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK) {1.0} else {26.0/(52.0 - 15.0)});
        REVERSE_LR(fighter);
    }

}

#[acmd_script( agent = "gaogaen", script = "effect_throwb" , category = ACMD_EFFECT , low_priority)]
unsafe fn gaogaen_throw_b_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0.0, 13.0, 20.0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(fighter, 0.8);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
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
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
            EFFECT(fighter, Hash40::new("sys_bomb_b"), Hash40::new("top"), 10, 0, -5.0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

#[acmd_script( agent = "gaogaen", script = "game_throwhi" , category = ACMD_GAME , low_priority)]
unsafe fn gaogaen_throw_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    VarModule::off_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK);
    if WorkModule::is_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
        VarModule::on_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK);
    }
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
            // Incin Buster
            ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 13.0, 45, 20, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        else {
            // Normal throw
            ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 11.0, 88, 82, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(fighter, 33.0/(22.0-10.0));
        }
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(fighter, 1.0);
        }
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        //FT_CATCH_STOP(fighter, 14, 1);
        CHECK_FINISH_CAMERA(fighter, 1, 20);
        let hitlag = if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK) {4.5} else {3.0};
        let sound = if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK) {*COLLISION_SOUND_ATTR_HEAVY} else {*COLLISION_SOUND_ATTR_PUNCH};
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 88, 70, 0, 65, 7.0, 0.0, 10.0, 0.0, Some(0.0), Some(10.0), Some(0.0), hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(boma, true, false);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}


#[acmd_script( agent = "gaogaen", script = "effect_throwhi" , category = ACMD_EFFECT , low_priority)]
unsafe fn gaogaen_throw_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK) {
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
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK) {
            EFFECT(fighter, Hash40::new("sys_hit_fire"), Hash40::new("waist"), 0.0, 0.0, 10.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(fighter, 0.75);
        }
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        EFFECT_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 32, 0, 90, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.5);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("gaogaen_throw_hi"), Hash40::new("top"), 0, 16, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK) {
            EFFECT(fighter, Hash40::new("gaogaen_revenge_start"), Hash40::new("top"), -2, 10, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
            EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

#[acmd_script( agent = "gaogaen", script = "sound_throwhi" , category = ACMD_SOUND , low_priority)]
unsafe fn gaogaen_throw_hi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK) {
            PLAY_SE(fighter, Hash40::new("vc_gaogaen_appeal_l01"));
        }
        else {
            PLAY_SEQUENCE(fighter, Hash40::new("seq_gaogaen_rnd_attack"));
        }
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_kick_hit_m"));
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK) {
            PLAY_SEQUENCE(fighter, Hash40::new("seq_gaogaen_rnd_attackappeal01"));
        }
    }
}

#[acmd_script( agent = "gaogaen", script = "game_throwlw", category = ACMD_GAME, low_priority )]
unsafe fn gaogaen_throw_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    VarModule::off_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK);
    if WorkModule::is_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
        VarModule::on_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK);
    }
    if is_excute(fighter) {
        let bkb = if WorkModule::is_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) { 50 } else { 70 };
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 68, 47, 0, bkb, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        FT_CATCH_STOP(fighter, 12, 1);
        CHECK_FINISH_CAMERA(fighter, 8, 0);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

#[acmd_script( agent = "gaogaen", script = "effect_throwlw" , category = ACMD_EFFECT , low_priority)]
unsafe fn gaogaen_throw_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK) {
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
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        EFFECT_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -10, 24, 0, 90, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.5);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        EFFECT_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -10, 21, 0, 90, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.5);
    }
}


pub fn install() {
    install_acmd_scripts!(
        gaogaen_throw_f_game,
        gaogaen_throw_f_effect,
        gaogaen_throw_b_game,
        gaogaen_throw_b_effect,
        gaogaen_throw_hi_game,
        gaogaen_throw_hi_effect,
        gaogaen_throw_hi_sound,
        gaogaen_throw_lw_game,
        gaogaen_throw_lw_effect,
    );
}

