use super::*;


#[acmd_script( agent = "eflame", script = "game_specialairhijump", category = ACMD_GAME )]
unsafe fn game_specialairhijump(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 85, 100, 64, 16, 2.5, 0.0, 18.0, 5.0, Some(0.0), Some(2.5), Some(5.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 95, 100, 64, 16, 4.0, 0.0, 15.0, 11.0, Some(0.0), Some(4.0), Some(11.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 100, 100, 64, 16, 4.0, 0.0, 14.0, 15.0, Some(0.0), Some(4.0), Some(15.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 10.0, false);
        AttackModule::set_add_reaction_frame(boma, 2, 10.0, false);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 95, 100, 53, 20, 2.5, 0.0, 21.5, 5.0, Some(0.0), Some(15.0), Some(5.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 100, 100, 53, 20, 4.0, 0.0, 20.0, 11.0, Some(0.0), Some(14.0), Some(11.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 100, 100, 53, 20, 4.0, 0.0, 20.0, 15.0, Some(0.0), Some(14.0), Some(15.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 10.0, false);
        AttackModule::set_add_reaction_frame(boma, 2, 10.0, false);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_EFLAME_STATUS_SPECIAL_HI_FLAG_START_CONTROL);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 95, 100, 50, 20, 2.5, 0.0, 23.0, 5.0, Some(0.0), Some(19.0), Some(5.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 100, 100, 50, 20, 4.0, 0.0, 24.0, 11.0, Some(0.0), Some(20.0), Some(11.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 100, 100, 50, 20, 4.0, 0.0, 24.0, 15.0, Some(0.0), Some(20.0), Some(15.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 10.0, false);
        AttackModule::set_add_reaction_frame(boma, 2, 10.0, false);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 6.0);
    if IS_EXIST_ARTICLE(fighter, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
        if is_excute(fighter) {
            ArticleModule::add_motion_partial(boma, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(boma) {
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
    }
    frame(lua_state, 22.0);
    FT_MOTION_RATE(fighter, 0.7);
    frame(lua_state, 27.0);
    if IS_EXIST_ARTICLE(fighter, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
        if is_excute(fighter) {
            ArticleModule::add_motion_partial(boma, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(boma) {
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    if is_excute(fighter) {
        WorkModule::set_int64(boma, hash40("special_air_hi_jump") as i64, *FIGHTER_EFLAME_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        println!("31");
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
    }
    frame(lua_state, 32.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: -7.0, z: 0.0});
        WorkModule::on_flag(boma, *FIGHTER_EFLAME_STATUS_SPECIAL_HI_FLAG_END_CONTROL);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 280, 130, 39, 0, 6.0, 0.0, 3.0, 9.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_EFLAME_STATUS_SPECIAL_HI_FLAG_ENABLE_GROUND);
    }
}

#[acmd_script( agent = "eflame", script = "game_specialairhifall", category = ACMD_GAME )]
unsafe fn game_specialairhifall(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::set_int64(boma, hash40("special_air_hi_fall") as i64, *FIGHTER_EFLAME_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
    }
}

#[acmd_script(agent = "eflame", script = "game_speciallw", category = ACMD_GAME)]
unsafe fn game_speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 5.0/(12.0-1.0));

    frame(lua_state, 12.0);
    FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script(agent = "eflame", script = "game_specialairlw", category = ACMD_GAME)]
unsafe fn game_specialairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 5.0/(12.0-1.0));

    frame(lua_state, 12.0);
    FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "eflame_firepillar", script = "game_specialhi" , category = ACMD_GAME , low_priority)]
unsafe fn eflame_firepillar_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 60, 100, 0, 90, 8.0, 0.0, 8.0, 1.0, Some(0.0), Some(8.0), Some(-2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 60, 100, 0, 90, 7.0, 0.0, 15.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 60, 100, 0, 90, 5.0, 0.0, 23.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 60, 100, 0, 90, 8.0, 0.0, 8.0, 1.0, Some(0.0), Some(8.0), Some(-2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 60, 100, 0, 90, 7.0, 0.0, 15.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 60, 100, 0, 80, 5.0, 0.0, 27.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x199c462b5d));
    }
}

#[acmd_script( agent = "eflame_firepillar", script = "effect_specialhi" , category = ACMD_EFFECT , low_priority)]
unsafe fn eflame_firepillar_special_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("eflame_promrevolt_firepillar_ground"), Hash40::new("top"), 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("eflame_promrevolt_firepillar"), Hash40::new("top"), 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.06, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
        EFFECT(fighter, Hash40::new("eflame_promrevolt_firepillar_impact"), Hash40::new("top"), 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.06, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
    }
}

pub fn install() {
    install_acmd_scripts!(
        game_specialairhijump,
        game_specialairhifall,
        game_speciallw,
        game_specialairlw,

        eflame_firepillar_special_hi_game,
        eflame_firepillar_special_hi_effect,
    );
}

