
use super::*;

unsafe fn manage_sword_motion(fighter: &mut L2CAgentBase, motion: Hash40) {
    let exists = {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD);
        app::sv_animcmd::IS_EXIST_ARTICLE(fighter.lua_state_agent)
    };

    if !exists {
        return;
    }

    if is_excute(fighter) {
        ArticleModule::add_motion_partial(
            fighter.module_accessor,
            *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD,
            *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE,
            motion,
            5.0,
            5.0,
            false,
            false,
            0.0,
            false,
            true,
            false
        );
    }

    if MotionModule::is_changing(fighter.module_accessor) && is_excute(fighter) {
        fighter.on_flag(*FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
    }
}

#[acmd_script(agent = "elight", script = "effect_specialhistart", category = ACMD_EFFECT)]
unsafe fn effect_specialhistart(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("elight_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, -0.3);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("elight_sword_open"), true, true);
        EFFECT_FOLLOW(fighter, Hash40::new("elight_sword_beam_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, -0.3);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_elight_sword1"), Hash40::new("tex_elight_sword2"), 5, Hash40::new("sword1"), 0.0, 0.0, -0.08, Hash40::new("sword1"), 15.5, 0.0, -0.08, false, Hash40::new("null"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(agent = "elight", script = "game_specialairhi1", category = ACMD_GAME)]
unsafe fn game_specialairhi1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    manage_sword_motion(fighter, Hash40::new("to_open"));

    frame(lua_state, 13.0);
    FT_MOTION_RATE(fighter, 0.75);

    frame(lua_state, 21.0);
    FT_MOTION_RATE(fighter, 1.0);

    frame(lua_state, 24.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_EXPROSIVESHOT, false, -1);
    }

    frame(lua_state, 25.0);
    manage_sword_motion(fighter, Hash40::new("to_close"));
}

#[acmd_script(agent = "elight", script = "game_specialairhi2", category = ACMD_GAME)]
unsafe fn game_specialairhi2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 1.0);

    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, 0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_NONE);
    }

    manage_sword_motion(fighter, Hash40::new("to_open"));

    frame(lua_state, 23.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_SPREADBULLET, false, -1);
    }

    frame(lua_state, 24.0);

    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_SPREADBULLET, false, -1);
    }

    frame(lua_state, 25.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_SPREADBULLET, false, -1);
    }

    frame(lua_state, 26.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_SPREADBULLET, false, -1);
    }

    frame(lua_state, 27.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_SPREADBULLET, false, -1);
    }

    frame(lua_state, 29.0);
    manage_sword_motion(fighter, Hash40::new("to_close"));
}

#[acmd_script(agent = "elight", script = "effect_specialairhi1", category = ACMD_EFFECT)]
unsafe fn effect_specialairhi1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), 14, 0, 3.3, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("elight_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, -0.3);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("elight_sword_open"), true, true);
        EFFECT_FOLLOW(fighter, Hash40::new("elight_sword_beam_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, -0.3);
        EFFECT(fighter, Hash40::new("elight_lay_shot"), Hash40::new("sword1"), 6, 0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("elight_sword_beam_m"), true, true);
        EFFECT_FOLLOW(fighter, Hash40::new("elight_sword_close_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, -0.3);
    }
}

#[acmd_script(agent = "elight", script = "effect_specialairhi2", category = ACMD_EFFECT)]
unsafe fn effect_specialairhi2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), 12, 0, -1.7, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("elight_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, -0.3);
    }
    
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("elight_sword_open"), true, true);
        EFFECT_FOLLOW(fighter, Hash40::new("elight_sword_beam_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, -0.3);
        EFFECT(fighter, Hash40::new("elight_lay_dust_shot"), Hash40::new("sword1"), 6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("elight_sword_beam_m"), true, true);
        EFFECT_FOLLOW(fighter, Hash40::new("elight_sword_close_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, -0.3);
    }
}

#[acmd_script(agent = "elight", script = "sound_specialairhi1", category = ACMD_SOUND)]
unsafe fn sound_specialairhi1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new_raw(0x1a1b87a0dc));
    }
    
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_elight_special_h02_01_shot"));
    }
}

#[acmd_script(agent = "elight", script = "sound_specialairhi2", category = ACMD_SOUND)]
unsafe fn sound_specialairhi2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new_raw(0x1a9d13d272));
    }
    
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_elight_special_h03_shot"));
    }
}

#[acmd_script(agent = "elight", script = "expression_specialairhi1", category = ACMD_EFFECT)]
unsafe fn expression_specialairhi1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_beams"), 0, false, 0x50000000);
    }
}


#[acmd_script(agent = "elight", script = "expression_specialairhi2", category = ACMD_EFFECT)]
unsafe fn expression_specialairhi2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_79_beams"), 0, false, 0x50000000);
    }
}

#[acmd_script(agent = "elight", script = "game_specialairhijump", category = ACMD_GAME)]
unsafe fn game_specialairhijump(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, 0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_NONE);
        let mut angle = app::lua_bind::FighterKineticEnergyMotion::get_angle(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION) as _);
        if PostureModule::lr(fighter.module_accessor) > 0.0 {
            angle *= -1.0;
        }
        angle = 80.0 - angle.to_degrees() / 2.0;
        if angle < 0.0 {
            angle += 360.0;
        }
        ATTACK(fighter, 0, 0, Hash40::new("sword2"), 7.0, angle as u64, 100, 110, 0, 4.0, 2.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("sword2"), 7.0, angle as u64, 100, 110, 0, 4.0, 10.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0, angle as u64, 100, 110, 0, 4.0, 0.0, 17.0, 6.0, Some(0.0), Some(4.0), Some(6.0), 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 7.0, angle as u64, 100, 110, 0, 4.0, 0.0, 17.0, 13.0, Some(0.0), Some(4.0), Some(13.0), 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        //AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
        //AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 5.0, false);
        //AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 5.0, false);
        //AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 5.0, false);
    }

    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 57, 10, 0, 60, 4.0, 0.0, 22.0, 5.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 80, 10, 0, 50, 4.0, 0.0, 22.0, 13.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0, 68, 10, 0, 75, 4.0, 0.0, 10.0, 7.0,Some( 0.0), Some(15.0), Some(7.0), 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 7.0, 75, 10, 0, 70, 4.0, 0.0, 10.0, 15.0,Some( 0.0), Some(15.0), Some(15.0), 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        //AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
        //AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 5.0, false);
        //AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 5.0, false);
        //AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 5.0, false);
    }

    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("sword2"), 7.0, 72, 10, 0, 55, 4.0, 2.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("sword2"), 7.0, 68, 10, 0, 55, 4.0, 9.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::clear(fighter.module_accessor, 2, false);
        AttackModule::clear(fighter.module_accessor, 3, false);
        //AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
        //AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 5.0, false);
    }

    frame(lua_state, 6.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        notify_event_msc_cmd!(fighter, 0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }

    frame(lua_state, 10.0);

    manage_sword_motion(fighter, Hash40::new("to_close"));

    frame(lua_state, 6.0);
    FT_MOTION_RATE(fighter, 0.5);

    frame(lua_state, 12.0);
    FT_MOTION_RATE(fighter, 1.0);

}

#[acmd_script(agent = "elight", script = "effect_specialairhijump", category = ACMD_EFFECT)]
unsafe fn effect_specialairhijump(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("elight_lay_lump"), Hash40::new("top"), 0, 0, -4, 0, 0, 0, 0.8, true);
        EFFECT_FOLLOW(fighter, Hash40::new("elight_sword_beam_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, -0.3);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("elight_sword_light"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -2, 11, 15, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 3);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("elight_sword_beam_m"), true, true);
        EFFECT_FOLLOW(fighter, Hash40::new("elight_sword_close_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, -0.3);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("elight_sword_light"), false, false);
    }
}

#[acmd_script(agent = "elight", script = "game_speciallw", category = ACMD_GAME)]
unsafe fn game_speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 5.0/(12.0-1.0));

    frame(lua_state, 12.0);
    FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script(agent = "elight", script = "game_specialairlw", category = ACMD_GAME)]
unsafe fn game_specialairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 5.0/(12.0-1.0));

    frame(lua_state, 12.0);
    FT_MOTION_RATE(fighter, 1.0);
}

pub fn install() {
    install_acmd_scripts!(
        game_specialairhi1,
        game_specialairhi2,
        game_specialairhijump,

        effect_specialhistart,
        effect_specialairhi1,
        effect_specialairhi2,
        effect_specialairhijump,

        sound_specialairhi1,
        sound_specialairhi2,

        expression_specialairhi1,
        expression_specialairhi2,

        game_speciallw,
        game_specialairlw,
    );
}

