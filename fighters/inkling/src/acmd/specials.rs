
use super::*;

#[acmd_script( agent = "inkling", script = "game_specialnend" , category = ACMD_GAME , low_priority)]
unsafe fn inkling_special_n_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(fighter, 3, 0, Hash40::new("haver"), 5.0, 65, 55, 0, 75, 4.5, 0.0, 1.0, 2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 4, 0, Hash40::new("haver"), 5.0, 65, 55, 0, 75, 6.0, 0.0, 1.5, 10.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        AttackModule::set_ink_value(boma, 0, 15.0);
        AttackModule::set_ink_value(boma, 1, 15.0);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "inkling", script = "game_specialairnend" , category = ACMD_GAME , low_priority)]
unsafe fn inkling_special_air_n_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(fighter, 3, 0, Hash40::new("haver"), 5.0, 65, 55, 0, 75, 4.5, 0.0, 1.0, 2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 4, 0, Hash40::new("haver"), 5.0, 65, 55, 0, 75, 6.0, 0.0, 1.5, 10.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        AttackModule::set_ink_value(boma, 0, 15.0);
        AttackModule::set_ink_value(boma, 1, 15.0);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "inkling", script = "effect_specialsend" , category = ACMD_EFFECT , low_priority)]
unsafe fn inkling_special_s_end_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0.0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -2, 0, 15.0, 0, 180, 0, 1.15, true);
        EFFECT_FOLLOW(fighter, Hash40::new("inkling_squid_change"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 2.0, true);
        LAST_PARTICLE_SET_COLOR(fighter, WorkModule::get_float(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_R), WorkModule::get_float(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_G), WorkModule::get_float(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_B));
        EffectModule::enable_sync_init_pos_last(boma);
    }
}

pub fn install() {
    install_acmd_scripts!(
        inkling_special_n_end_game,
        inkling_special_air_n_end_game,
        inkling_special_s_end_effect,
    );
}

