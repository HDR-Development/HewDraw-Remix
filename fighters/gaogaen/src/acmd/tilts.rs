
use super::*;


#[acmd_script( agent = "gaogaen", script = "game_attacks3hi" , category = ACMD_GAME , low_priority)]
unsafe fn gaogaen_attack_s3_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 12.0, 31, 88, 0, 54, 4.5, -1.0, -1.8, 1.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 12.0, 31, 88, 0, 54, 3.4, 0.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("handl"), 13.0, 31, 88, 0, 54, 4.5, 3.4, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "gaogaen", script = "effect_attacks3hi" , category = ACMD_EFFECT , low_priority)]
unsafe fn gaogaen_attack_s3_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::status::IS_HEAVY_ATTACK){
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_purple"), Hash40::new("shoulderl"), 0, 0, 0, 0, 0, 0, 0.45, true);
            LAST_EFFECT_SET_RATE(fighter, 0.5);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_purple"), Hash40::new("arml"), 1.0, 0, 0, 0, 0, 0, 0.6, true);
            LAST_EFFECT_SET_RATE(fighter, 0.5);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_purple"), Hash40::new("arml"), 7.0, 0, 0, 0, 0, 0, 0.75, true);
            LAST_EFFECT_SET_RATE(fighter, 0.5);
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 15.0, -5.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);

            EFFECT_FOLLOW(fighter, Hash40::new("gaogaen_belt_fire_appeal"), Hash40::new("feeler"), 0, 3, 0, 0, 0, 0, 0.65, true);
            LAST_EFFECT_SET_COLOR(fighter, 0.5, 0.1, 0.675);
            LAST_EFFECT_SET_RATE(fighter, 1.25);

            COL_PRI(fighter, 200);
            FLASH(fighter, 0.284, 0.1156, 0.284, 0.8);
        }
        else{
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::status::IS_HEAVY_ATTACK){
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("gaogaen_thrust"), Hash40::new("gaogaen_thrust"), Hash40::new("top"), 0, 10.0, 8.0, -38, 0, 0, 1.5, true, *EF_FLIP_YZ);
            
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 5, 0, -3.0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_COLOR(fighter, 0.175, 0.0, 0.375);
            LAST_EFFECT_SET_RATE(fighter, 0.8);
        }
        else{
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("gaogaen_thrust"), Hash40::new("gaogaen_thrust"), Hash40::new("top"), 0, 10.0, 8.0, -38, 0, 0, 1.0, true, *EF_FLIP_YZ);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::status::IS_HEAVY_ATTACK){
            EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_purple"), Hash40::new("handl"), 6.5, 0, 0, 0, 0, 0, 0.25, true);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::status::IS_HEAVY_ATTACK){
            COL_NORMAL(fighter);
        }
    }
}

#[acmd_script( agent = "gaogaen", script = "game_attacks3" , category = ACMD_GAME , low_priority)]
unsafe fn gaogaen_attack_s3_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 12.0, 31, 88, 0, 54, 4.5, -1.0, -1.8, 1.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 12.0, 31, 88, 0, 54, 3.4, 0.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("handl"), 13.0, 31, 88, 0, 54, 4.5, 3.4, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "gaogaen", script = "effect_attacks3" , category = ACMD_EFFECT , low_priority)]
unsafe fn gaogaen_attack_s3_s_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::status::IS_HEAVY_ATTACK){
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_purple"), Hash40::new("shoulderl"), 0, 0, 0, 0, 0, 0, 0.45, true);
            LAST_EFFECT_SET_RATE(fighter, 0.5);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_purple"), Hash40::new("arml"), 1.0, 0, 0, 0, 0, 0, 0.6, true);
            LAST_EFFECT_SET_RATE(fighter, 0.5);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_purple"), Hash40::new("arml"), 7.0, 0, 0, 0, 0, 0, 0.75, true);
            LAST_EFFECT_SET_RATE(fighter, 0.5);
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 15.0, -5.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);

            EFFECT_FOLLOW(fighter, Hash40::new("gaogaen_belt_fire_appeal"), Hash40::new("feeler"), 0, 3, 0, 0, 0, 0, 0.65, true);
            LAST_EFFECT_SET_COLOR(fighter, 0.5, 0.1, 0.675);
            LAST_EFFECT_SET_RATE(fighter, 1.25);

            COL_PRI(fighter, 200);
            FLASH(fighter, 0.284, 0.1156, 0.284, 0.8);
        }
        else{
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::status::IS_HEAVY_ATTACK){
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("gaogaen_thrust"), Hash40::new("gaogaen_thrust"), Hash40::new("top"), 0, 7.5, 8.0, -3.0, 0, 0, 1.5, true, *EF_FLIP_YZ);
            
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 5, 0, -3.0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_COLOR(fighter, 0.175, 0.0, 0.375);
            LAST_EFFECT_SET_RATE(fighter, 0.8);
        }
        else{
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("gaogaen_thrust"), Hash40::new("gaogaen_thrust"), Hash40::new("top"), 0, 7.5, 8.0, -4.0, 0, 0, 1.0, true, *EF_FLIP_YZ);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::status::IS_HEAVY_ATTACK){
            EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_purple"), Hash40::new("handl"), 6.5, 0, 0, 0, 0, 0, 0.25, true);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::status::IS_HEAVY_ATTACK){
            COL_NORMAL(fighter);
        }
    }
}

#[acmd_script( agent = "gaogaen", script = "game_attacks3lw" , category = ACMD_GAME , low_priority)]
unsafe fn gaogaen_attack_s3_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 12.0, 31, 88, 0, 54, 4.5, -1.0, -1.8, 1.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 12.0, 31, 88, 0, 54, 3.4, 0.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("handl"), 13.0, 31, 88, 0, 54, 4.5, 3.4, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "gaogaen", script = "effect_attacks3lw" , category = ACMD_EFFECT , low_priority)]
unsafe fn gaogaen_attack_s3_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::status::IS_HEAVY_ATTACK){
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_purple"), Hash40::new("shoulderl"), 0, 0, 0, 0, 0, 0, 0.45, true);
            LAST_EFFECT_SET_RATE(fighter, 0.5);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_purple"), Hash40::new("arml"), 1.0, 0, 0, 0, 0, 0, 0.6, true);
            LAST_EFFECT_SET_RATE(fighter, 0.5);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_purple"), Hash40::new("arml"), 7.0, 0, 0, 0, 0, 0, 0.75, true);
            LAST_EFFECT_SET_RATE(fighter, 0.5);
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 15.0, -5.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);

            EFFECT_FOLLOW(fighter, Hash40::new("gaogaen_belt_fire_appeal"), Hash40::new("feeler"), 0, 3, 0, 0, 0, 0, 0.65, true);
            LAST_EFFECT_SET_COLOR(fighter, 0.5, 0.1, 0.675);
            LAST_EFFECT_SET_RATE(fighter, 1.25);

            COL_PRI(fighter, 200);
            FLASH(fighter, 0.284, 0.1156, 0.284, 0.8);
        }
        else{
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::status::IS_HEAVY_ATTACK){
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("gaogaen_thrust"), Hash40::new("gaogaen_thrust"), Hash40::new("top"), 0, 4.5, 8.0, 7, 0, 0, 1.5, true, *EF_FLIP_YZ);
            
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 5, 0, -3.0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_COLOR(fighter, 0.175, 0.0, 0.375);
            LAST_EFFECT_SET_RATE(fighter, 0.8);
        }
        else{
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("gaogaen_thrust"), Hash40::new("gaogaen_thrust"), Hash40::new("top"), 0, 4.5, 8.0, 7, 0, 0, 1.0, true, *EF_FLIP_YZ);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::status::IS_HEAVY_ATTACK){
            EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_purple"), Hash40::new("handl"), 6.5, 0, 0, 0, 0, 0, 0.25, true);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::common::status::IS_HEAVY_ATTACK){
            COL_NORMAL(fighter);
        }
    }
}

#[acmd_script( agent = "gaogaen", script = "game_attacklw3" , category = ACMD_GAME , low_priority)]
unsafe fn gaogaen_attack_lw3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
        ATTACK(fighter, 0, 0, Hash40::new("legl"), 9.0, 89, 47, 0, 70, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneel"), 9.0, 93, 47, 0, 70, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("kneel"), 9.0, 97, 47, 0, 70, 4.5, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {

    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        JostleModule::set_status(boma, true);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        gaogaen_attack_s3_hi_game,
        gaogaen_attack_s3_hi_effect,
        gaogaen_attack_s3_s_game,
        gaogaen_attack_s3_s_effect,
        gaogaen_attack_s3_lw_game,
        gaogaen_attack_s3_lw_effect,
        gaogaen_attack_lw3_game,
    );
}

