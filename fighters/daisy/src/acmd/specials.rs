
use super::*;


#[acmd_script( agent = "daisy", script = "game_specialsstart" , category = ACMD_GAME , low_priority)]
unsafe fn daisy_special_s_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 10.0/(11.0-5.0));
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
}

#[acmd_script( agent = "daisy", script = "game_specialairsstart" , category = ACMD_GAME , low_priority)]
unsafe fn daisy_special_air_s_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 7.0/(11.0-5.0));
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
}

#[acmd_script( agent = "daisy", script = "game_specialsjump" , category = ACMD_GAME , low_priority)]
unsafe fn daisy_special_s_jump_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 15.0/(25.0-1.0));
        JostleModule::set_status(boma, false);
        SEARCH(fighter, 0, 0, Hash40::new("hip"), 3.5, 0.0, 0.0, 0.0, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_PEACH_SPECIAL_S_BRAKE);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_PEACH_STATUS_SPECIAL_S_JUMP_ID_TIME_OUT);
    }
}

#[acmd_script( agent = "daisy", script = "game_specialshitend" , category = ACMD_GAME , low_priority)]
unsafe fn daisy_special_s_hit_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 75, 60, 0, 90, 7.7, 0.0, 5.0, 4.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 6, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HIP);
        AttackModule::set_add_reaction_frame(boma, 0, 2.0, false);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        FT_MOTION_RATE(fighter, 1.4);
    }

}

#[acmd_script( agent = "daisy", script = "game_specialhistart" , category = ACMD_GAME , low_priority)]
unsafe fn daisy_special_hi_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, Hash40::new("special_hi_start"), false, 1.0);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PEACH_STATUS_SPECIAL_HI_FLAG_MOVE_TRANS);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("haver"), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 118, 0, 40, 6.0, 0.0, 5.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PARASOL);
        ATTACK(fighter, 1, 0, Hash40::new("head"), 10.0, 361, 118, 0, 40, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PARASOL);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 1, false);
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("haver"), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        ATTACK(fighter, 0, 0, Hash40::new("havel"), 1.0, 78, 100, 90, 0, 5.0, 0.0, 4.0, 0.0, Some(0.0), Some(-1.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PARASOL);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "daisy", script = "expression_specialhistart", category = ACMD_EXPRESSION, low_priority )]
unsafe fn daisy_special_hi_start_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 2);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "daisy", script = "game_specialairhistart" , category = ACMD_GAME , low_priority)]
unsafe fn daisy_special_air_hi_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, Hash40::new("special_hi_start"), false, 1.0);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PEACH_STATUS_SPECIAL_HI_FLAG_MOVE_TRANS);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("haver"), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 118, 0, 40, 6.0, 0.0, 5.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PARASOL);
        ATTACK(fighter, 1, 0, Hash40::new("head"), 10.0, 361, 118, 0, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PARASOL);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 1, false);
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("haver"), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        ATTACK(fighter, 0, 0, Hash40::new("havel"), 1.0, 78, 100, 90, 0, 5.0, 0.0, 4.0, 0.0, Some(0.0), Some(-1.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PARASOL);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "daisy", script = "expression_specialairhistart", category = ACMD_EXPRESSION, low_priority )]
unsafe fn daisy_special_air_hi_start_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "daisy", script = "game_specialhiopen" , category = ACMD_GAME , low_priority)]
unsafe fn daisy_special_hi_open_game(fighter: &mut L2CAgentBase) {
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::change_motion(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, Hash40::new("special_hi_open"), false, 1.0);
    }

}

#[acmd_script( agent = "daisy", script = "game_speciallw", category = ACMD_GAME, low_priority )]
unsafe fn daisy_special_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        let item_kind = fighter.get_int(*FIGHTER_PEACH_STATUS_SPECIAL_LW_WORK_INT_UNIQ_ITEM_KIND); 
        if item_kind == *ITEM_KIND_NONE {
            ArticleModule::generate_article(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_DAIKON, false, -1);
        } else if item_kind == *ITEM_KIND_BOMBHEI {
            ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_BOMBHEI), 0, 0, false, false);
        } else if item_kind == *ITEM_KIND_DOSEISAN {
            ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_DOSEISAN), 0, 0, false, false);
        } else if item_kind == *ITEM_KIND_BEAMSWORD {
            ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_BEAMSWORD), 0, 0, false, false);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if !fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
        }
    }
}

#[acmd_script( agent = "daisy", script = "effect_speciallw", category = ACMD_EFFECT, low_priority )]
unsafe fn daisy_special_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("daisy_hikkonuki"), Hash40::new("top"), 0, 0, 1, 0, 0, 0, 1.0, true);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("daisy_hikkonuki"), -1);
        LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "daisy", script = "sound_speciallw", category = ACMD_SOUND, low_priority )]
unsafe fn daisy_special_lw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_daisy_special_l02"));
    }
    wait(lua_state, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_daisy_special_l01"));
        if fighter.get_int(*FIGHTER_PEACH_STATUS_SPECIAL_LW_WORK_INT_UNIQ_ITEM_KIND) != *ITEM_KIND_NONE 
        && ItemModule::is_have_item(fighter.module_accessor, 0) {
            PLAY_SE(fighter, Hash40::new("vc_daisy_appeal_s01"));
        } 
    }
}

pub fn install() {
    install_acmd_scripts!(
        daisy_special_s_start_game,
        daisy_special_air_s_start_game,
        daisy_special_s_jump_game,
        daisy_special_s_hit_end_game,
        daisy_special_hi_start_game,
        daisy_special_hi_start_expression,
        daisy_special_air_hi_start_game,
        daisy_special_air_hi_start_expression,
        daisy_special_hi_open_game,
        daisy_special_lw_game,
        daisy_special_lw_effect,
        daisy_special_lw_sound
    );
}

