
use super::*;

unsafe extern "C" fn mewtwo_attack_11_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.600);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 83, 100, 35, 0, 3.5, 0.0, 11.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 83, 100, 35, 0, 4.5, 0.0, 11.8, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 83, 100, 35, 0, 4.0, 0.0, 9.0, 13.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        //Jab-Lock Hitbox
        ATTACK(fighter, 3, 0, Hash40::new("top"), 3.0, 361, 20, 0, 25, 2.5, 1.0, 3.0, 2.0, Some(1.0), Some(3.0), Some(14.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        AttackModule::set_down_only(boma, 3, true);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        //WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
    }
    
}

unsafe extern "C" fn mewtwo_attack_100_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 361, 69, 0, 89, 5.5, 0.0, 10.5, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 361, 69, 0, 89, 5.5, 0.0, 10.5, 15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 361, 69, 0, 89, 5.5, 0.0, 10.5, 18.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

unsafe extern "C" fn mewtwo_attack_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.85);
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 8.5, 3.0);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 54, 67, 0, 80, 5.5, 0.0, 8.6, 13.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 60, 57, 0, 80, 4.3, 0.0, 8.2, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 13.0);
    FT_MOTION_RATE_RANGE(fighter, 13.0, 21.0, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 60, 60, 0, 77, 4.5, 0.0, 8.6, 14.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 70, 60, 0, 77, 3.7, 0.0, 8.2, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 3.5, 3.0);
    }
    frame(lua_state, 21.0); //43
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn mewtwo_attack_dash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_FLW_POS(fighter, Hash40::new("mewtwo_pk_attack_e"), Hash40::new("haver"), 1.9 * PostureModule::lr(boma), 0, 0, 0, 0, 0, 0.9, true);
        EffectModule::enable_sync_init_pos_last(boma);
        //LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        EffectModule::kill_kind(boma, Hash40::new("mewtwo_pk_attack_e"), true, true);
    }
}

pub fn install() {
    smashline::Agent::new("mewtwo")
        .acmd("game_attack11", mewtwo_attack_11_game)
        .acmd("game_attack100end", mewtwo_attack_100_end_game)
        .acmd("game_attackdash", mewtwo_attack_dash_game)
		.acmd("effect_attackdash", mewtwo_attack_dash_effect)
        .install();
}
