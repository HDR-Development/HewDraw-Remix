use super::*;

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 0.8);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, false, 0);
    }
    frame(lua_state, 30.0);
    if is_excute(agent){
        ArticleModule::shoot(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE,  app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
    }
}

unsafe extern "C" fn game_specials1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.2);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_CROSS, false, -1);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_SIMON_STATUS_SPECIAL_S_FLAG_FALL);
    }
    frame(lua_state, 16.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ArticleModule::shoot(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_CROSS, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_AIR) {
           VarModule::on_flag(agent.battle_object, vars::simon::status::CROSS_LAND);
        }
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_SIMON_STATUS_SPECIAL_S_FLAG_FALL);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        boma.select_cliff_hangdata_from_name("special_hi");
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 95, 100, 155, 0, 8.5, 0.0, 9.5, 10.5, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 367, 100, 80, 0, 6.0, 0.0, 23.0, 9.5, Some(0.0), Some(6.0), Some(7.5), 0.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 0, false);
        ATTACK(agent, 1, 1, Hash40::new("top"), 6.0, 61, 95, 0, 85, 6.0, 0.0, 23.0, 9.5, Some(0.0), Some(6.0), Some(7.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_CHANGE_KINE);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn", game_specialn, Priority::Low);
    agent.acmd("game_specialairn", game_specialn, Priority::Low);

    agent.acmd("game_specials1", game_specials1, Priority::Low);
    agent.acmd("game_specialairs1", game_specials1, Priority::Low);

    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("game_specialairhi", game_specialhi, Priority::Low);
}