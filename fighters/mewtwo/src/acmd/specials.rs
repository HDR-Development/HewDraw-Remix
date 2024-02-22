
use super::*;

unsafe extern "C" fn mewtwo_special_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 7.4, 0.0, 8.4, 17.0, None, None, None, *FIGHTER_STATUS_KIND_MEWTWO_THROWN, *COLLISION_SITUATION_MASK_GA);
        CATCH(fighter, 1, Hash40::new("top"), 10.2, 0.0, 8.4, 17.0, None, None, None, *FIGHTER_STATUS_KIND_MEWTWO_THROWN, *COLLISION_SITUATION_MASK_G);
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_MEWTWO_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 280, 16, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 0.0, 361, 100, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW_MEWTWO, 0, 1.0, 280, 50, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        AttackModule::set_catch_only(boma, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW_MEWTWO, true, true);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
    frame(lua_state, 20.0);
    for _ in 0..6 {
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            let target = WorkModule::get_int64(boma, *FIGHTER_MEWTWO_STATUS_SPECIAL_S_WORK_INT_TARGET_OBJECT_ID);
            let target_group = WorkModule::get_int64(boma, *FIGHTER_MEWTWO_STATUS_SPECIAL_S_WORK_INT_THROWN_HIT_GROUP);
            let target_no = WorkModule::get_int64(boma, *FIGHTER_MEWTWO_STATUS_SPECIAL_S_WORK_INT_THROWN_HIT_NO);
            ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW_MEWTWO, Hash40::new("throw"), target, target_group, target_no);
        }
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MEWTWO_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        WorkModule::on_flag(boma, *FIGHTER_MEWTWO_STATUS_SPECIAL_S_FLAG_GRAVITY_NORMAL);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MEWTWO_STATUS_SPECIAL_S_FLAG_HIT);
    
    }
}

unsafe extern "C" fn mewtwo_special_air_hi_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        KineticModule::clear_speed_all(boma);
    }
}

unsafe extern "C" fn mewtwo_special_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        GroundModule::select_cliff_hangdata(boma, *FIGHTER_MEWTWO_CLIFF_HANG_DATA_SPECIAL_HI as u32);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
}

unsafe extern "C" fn mewtwo_special_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_MEWTWO_GENERATE_ARTICLE_BINDBALL, false, -1);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 20.0/(53.0-24.0));
    }
}

unsafe extern "C" fn mewtwo_special_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_MEWTWO_GENERATE_ARTICLE_BINDBALL, false, -1);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 20.0/(53.0-24.0));
    }
}

unsafe extern "C" fn bindball_shoot_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 30, 80, 0, 50, 3.0, 0.0, -1.7, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 361, 140, 0, 0, 2.3, 0.0, -1.7, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
}

pub fn install() {
    smashline::Agent::new("mewtwo")
        .acmd("game_specials", mewtwo_special_s_game)
        .acmd("game_specialairhistart", mewtwo_special_air_hi_start_game)
        .acmd("game_specialairhi", mewtwo_special_air_hi_game)
        .acmd("game_speciallw", mewtwo_special_lw_game)
        .acmd("game_specialairlw", mewtwo_special_air_lw_game)
        .install();
    smashline::Agent::new("mewtwo_bindball")
        .acmd("game_shoot", bindball_shoot_game)
        .install();
}
