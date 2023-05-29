use super::*;

#[acmd_script( agent = "kirby", script = "game_attack11" , category = ACMD_GAME , low_priority)]
unsafe fn kirby_attack_11_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.5);
    frame(lua_state, 3.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 25, 0, 20, 3.0, 0.0, 5.5, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 361, 25, 0, 20, 3.0, 0.0, 5.5, 10.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 361, 20, 0, 20, 3.0, 0.0, 5.5, 13.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        //WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
    }
    
}

#[acmd_script( agent = "kirby", script = "game_attack12" , category = ACMD_GAME , low_priority)]
unsafe fn kirby_attack_12_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 65, 15, 0, 25, 3.0, 0.0, 5.2, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 65, 15, 0, 25, 3.5, 0.0, 5.2, 13.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    
}

#[acmd_script( agent = "kirby", script = "game_attackdash" , category = ACMD_GAME , low_priority)]
unsafe fn kirby_attack_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.8);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, vars::common::status::ATTACK_DASH_ENABLE_AIR_FALL);
        VarModule::on_flag(fighter.battle_object, vars::common::status::ATTACK_DASH_ENABLE_AIR_CONTINUE);
    }
    frame(lua_state, 9.0);
    FT_MOTION_RATE_RANGE(fighter, 9.0, 14.0, 4.0);
    if is_excute(fighter) {
        JostleModule::set_status(boma, false);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 361, 66, 0, 77, 5.0, 0.0, 5.0, 3.0, Some(0.0), Some(5.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        KineticModule::add_speed(boma, &Vector3f::new(0.8, 0.0, 0.0));
    }
    frame(lua_state, 14.0);
    FT_MOTION_RATE_RANGE(fighter, 14.0, 31.0, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 70, 50, 0, 80, 4.0, 0.0, 5.0, 4.0, Some(0.0), Some(5.0), Some(-0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            KineticModule::add_speed(boma, &Vector3f::new(-0.6, 0.0, 0.0));
        }
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            KineticModule::add_speed(boma, &Vector3f::new(-0.4, 0.0, 0.0));
        }
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            KineticModule::add_speed(boma, &Vector3f::new(-0.1, 0.0, 0.0));
        }
    }
    frame(lua_state, 31.0);
    FT_MOTION_RATE_RANGE(fighter, 31.0, 65.0, 22.0);
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, vars::common::status::ATTACK_DASH_ENABLE_AIR_FALL);
        VarModule::off_flag(fighter.battle_object, vars::common::status::ATTACK_DASH_ENABLE_AIR_CONTINUE);
        VarModule::on_flag(fighter.battle_object, vars::common::status::ATTACK_DASH_ENABLE_AIR_DRIFT);
        AttackModule::clear_all(boma);
        JostleModule::set_status(boma, true);
    }
    
}

#[acmd_script( agent = "kirby", script = "effect_attackdash", category = ACMD_EFFECT, low_priority )]
unsafe fn kirby_attack_dash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("kirby_dash"), Hash40::new("top"), 0, 6, 5, -90, 0, 160, 0.7, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        BURN_COLOR(fighter, 2, 0.059, 0.008, 0);
        BURN_COLOR_FRAME(fighter, 4, 2, 0.059, 0.008, 0.9);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        BURN_COLOR(fighter, 2, 0.059, 0.008, 0.9);
        BURN_COLOR_FRAME(fighter, 12, 2, 0.059, 0.008, 0);
        EFFECT_OFF_KIND(fighter, Hash40::new("kirby_dash"), false, true);
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        BURN_COLOR_NORMAL(fighter);
    }
    frame(lua_state, 44.0);
    if is_excute(fighter) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            FOOT_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        kirby_attack_11_game,
        kirby_attack_12_game,
        kirby_attack_dash_game,
        kirby_attack_dash_effect,
    );
}

