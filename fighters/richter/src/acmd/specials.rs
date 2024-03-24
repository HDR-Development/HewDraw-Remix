
use super::*;

unsafe extern "C" fn richter_special_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
            VarModule::on_flag(fighter.battle_object, vars::richter::instance::SPECIAL_N_LAND_CANCEL);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, false, -1);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        ArticleModule::shoot(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
    }
}

unsafe extern "C" fn richter_special_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_bottle_appear"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.4, true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.0, 0.5);
        LAST_EFFECT_SET_RATE(fighter, 0.6);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_sp_flash"), false, true);
    }
}

unsafe extern "C" fn richter_special_n_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_special_l01"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_richter_rnd_special_s"));
    }
}

unsafe extern "C" fn richter_special_n_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_lightthrow4item"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn richter_special_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
            VarModule::on_flag(fighter.battle_object, vars::richter::instance::SPECIAL_N_LAND_CANCEL);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, false, -1);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        ArticleModule::shoot(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
    }
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        VarModule::off_flag(boma.object(), vars::richter::instance::SPECIAL_N_LAND_CANCEL);
    }
}

unsafe extern "C" fn richter_special_air_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_bottle_appear"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.4, true);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.0, 0.5);
        LAST_EFFECT_SET_RATE(fighter, 0.6);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_sp_flash"), false, true);
    }
}

unsafe extern "C" fn richter_special_air_n_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_special_l01"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_richter_rnd_special_s"));
    }
}

unsafe extern "C" fn richter_special_air_n_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_lightthrow4item"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn richter_axe_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let lr = PostureModule::lr(fighter.module_accessor);
    let offset = Vector2f {x: (0.0 * lr), y: -4.2};
    let kirbyoffset = Vector2f {x: (0.0 * lr), y: -12.2};
    let axe_owner = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if axe_owner.kind () == *FIGHTER_KIND_KIRBY {
    if (lr == 1.0){
        PostureModule::set_rot(fighter.module_accessor, &Vector3f{ x: 0.0, y: 0.0, z: -90.0 }, 0);
        PostureModule::set_scale(fighter.module_accessor, 1.12, false);
        PostureModule::add_pos_2d(boma, &kirbyoffset);
    }
    else {
        PostureModule::set_rot(fighter.module_accessor, &Vector3f{ x: 0.0, y: 0.0, z: 90.0 }, 0);
        PostureModule::set_scale(fighter.module_accessor, 1.12, false);
        PostureModule::add_pos_2d(boma, &kirbyoffset);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("axe"), 3.5, 96, 99, 0, 32, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(boma);
    }
} else {
    if (lr == 1.0){
        PostureModule::set_rot(fighter.module_accessor, &Vector3f{ x: 0.0, y: 0.0, z: -90.0 }, 0);
        PostureModule::set_scale(fighter.module_accessor, 1.12, false);
        PostureModule::add_pos_2d(boma, &offset);
    }
    else {
        PostureModule::set_rot(fighter.module_accessor, &Vector3f{ x: 0.0, y: 0.0, z: 90.0 }, 0);
        PostureModule::set_scale(fighter.module_accessor, 1.12, false);
        PostureModule::add_pos_2d(boma, &offset);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("axe"), 3.5, 96, 99, 0, 32, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(boma);
    }
}
}

unsafe extern "C" fn richter_axe_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_greenshell_trace"), Hash40::new("rot"), 0, 1.5, 0, 0, 0, 0, 0.70, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.19, 0.0);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_greenshell_trace"), Hash40::new("rot"), 0, 1.5, 0, 0, 0, 0, 0.70, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.19, 0.0);
        BURN_COLOR(fighter, 1.0, 1.0, 1.0, 1.0);
        FLASH(fighter, 0.89, 0.706, 0.106, 0.5);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        BURN_COLOR_NORMAL(fighter);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("axe"), 0, -2.3, 0.4, 0, 0, 0, 0.1, false);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        FLASH(fighter, 1.0, 1.0, 1.0, 1.0);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        FLASH(fighter, 1.0, 1.0, 1.0, 1.0);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        FLASH(fighter, 1.0, 1.0, 1.0, 1.0);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        FLASH(fighter, 1.0, 1.0, 1.0, 0.5);
    }
}

unsafe extern "C" fn richter_axe_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_sword_swing_s"));
    }
}

unsafe extern "C" fn richter_special_s1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, vars::common::status::ATTACK_DASH_ENABLE_AIR_FALL);
        VarModule::on_flag(fighter.battle_object, vars::common::status::ATTACK_DASH_ENABLE_AIR_DRIFT);
        VarModule::on_flag(fighter.battle_object, vars::common::status::ATTACK_DASH_ENABLE_AIR_CONTINUE);
        boma.select_cliff_hangdata_from_name("special_s1");
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.object(), vars::common::instance::SIDE_SPECIAL_CANCEL_NO_HIT);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 60, 65, 0, 60, 4.0, 0.0, 8.5, -6.9, Some(0.0), Some(8.5), Some(8.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 60, 65, 0, 60, 4.0, 0.0, 12.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0, 60, 65, 0, 60, 4.0, 0.0, 5.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 60, 60, 0, 55, 4.0, 0.0, 8.5, -6.9, Some(0.0), Some(8.5), Some(8.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 60, 60, 0, 55, 4.0, 0.0, 12.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 60, 60, 0, 55, 4.0, 0.0, 5.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        VarModule::on_flag(fighter.object(), vars::common::instance::SIDE_SPECIAL_CANCEL_NO_HIT);
    }
}

unsafe extern "C" fn richter_special_s1_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT_FLW_POS(fighter, Hash40::new("richter_whip_dash"), Hash40::new("top"), -2.4, 9, 1, 0, 0, 0, 0.86, true);
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("richter_whip_dash"), true, true);
    }
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn richter_special_s1_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_whip_holding"));
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_richter_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_richter_attackdash"));
    }
}

unsafe extern "C" fn richter_special_s1_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn richter_whip_special_s1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
}

unsafe extern "C" fn richter_special_air_s1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, vars::common::status::ATTACK_DASH_ENABLE_AIR_FALL);
        VarModule::on_flag(fighter.battle_object, vars::common::status::ATTACK_DASH_ENABLE_AIR_DRIFT);
        VarModule::on_flag(fighter.battle_object, vars::common::status::ATTACK_DASH_ENABLE_AIR_CONTINUE);
        boma.select_cliff_hangdata_from_name("special_s1");
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        VarModule::on_flag(fighter.object(), vars::common::instance::SIDE_SPECIAL_CANCEL_NO_HIT);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 60, 65, 0, 60, 4.0, 0.0, 8.5, -6.9, Some(0.0), Some(8.5), Some(8.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 60, 65, 0, 60, 4.0, 0.0, 12.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0, 60, 65, 0, 60, 4.0, 0.0, 5.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 60, 60, 0, 55, 4.0, 0.0, 8.5, -6.9, Some(0.0), Some(8.5), Some(8.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 60, 60, 0, 55, 4.0, 0.0, 12.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 60, 60, 0, 55, 4.0, 0.0, 5.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        VarModule::on_flag(fighter.object(), vars::common::instance::SIDE_SPECIAL_CANCEL_NO_HIT);
    }
}

unsafe extern "C" fn richter_special_air_s1_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT_FLW_POS(fighter, Hash40::new("richter_whip_dash"), Hash40::new("top"), -2.4, 9, 1, 0, 0, 0, 0.86, true);
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("richter_whip_dash"), true, true);
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn richter_special_air_s1_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_whip_holding"));
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_richter_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_richter_attackdash"));
    }
}

unsafe extern "C" fn richter_special_air_s1_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}


unsafe extern "C" fn richter_whip_special_air_s1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
}

unsafe extern "C" fn richter_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        boma.select_cliff_hangdata_from_name("special_hi");
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 3.0);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 71, 90, 0, 65, 8.5, 0.0, 9.5, 10.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 2.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 71, 90, 0, 65, 6.0, 0.0, 23.0, 9.5, Some(0.0), Some(6.0), Some(7.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        WorkModule::on_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 71, 90, 0, 65, 6.0, 0.0, 23.0, 9.5, Some(0.0), Some(6.0), Some(7.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_CHANGE_KINE);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
        VarModule::on_flag(fighter.object(), vars::common::instance::UP_SPECIAL_CANCEL);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
}

unsafe extern "C" fn richter_special_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
    EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        FLASH(fighter, 1.0, 1.0, 1.0, 0.1);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FLASH(fighter, 1.0, 1.0, 1.0, 0.1);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
        EFFECT_FLW_POS(fighter, Hash40::new("richter_upper"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        EffectModule::kill_kind(boma, Hash40::new("richter_upper"), true, true);
    }
}

unsafe extern "C" fn richter_special_hi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        let play_vc = app::sv_math::rand(hash40("fighter"), 3);
        if play_vc == 0 {
            PLAY_SE(fighter, Hash40::new("vc_richter_special_h01"));
        }
        if play_vc == 1 {
            PLAY_SE(fighter, Hash40::new("vc_richter_special_n01"));
        }
        if play_vc == 2 {
            PLAY_SE(fighter, Hash40::new("vc_richter_attack07"));
        }
    }
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_special_h01"));
    }
}

unsafe extern "C" fn richter_special_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        boma.select_cliff_hangdata_from_name("special_hi");
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 3.0);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 71, 90, 0, 65, 8.5, 0.0, 9.5, 10.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 2.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 71, 90, 0, 65, 6.0, 0.0, 23.0, 9.5, Some(0.0), Some(6.0), Some(7.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        WorkModule::on_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 71, 90, 0, 65, 6.0, 0.0, 23.0, 9.5, Some(0.0), Some(6.0), Some(7.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_CHANGE_KINE);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        VarModule::on_flag(fighter.object(), vars::common::instance::UP_SPECIAL_CANCEL);
    }
}

unsafe extern "C" fn richter_special_air_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
    EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        FLASH(fighter, 1.0, 1.0, 1.0, 0.1);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FLASH(fighter, 1.0, 1.0, 1.0, 0.1);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
        EFFECT_FLW_POS(fighter, Hash40::new("richter_upper"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        EffectModule::kill_kind(boma, Hash40::new("richter_upper"), true, true);
    }
}

unsafe extern "C" fn richter_special_air_hi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        let play_vc = app::sv_math::rand(hash40("fighter"), 3);
        if play_vc == 0 {
            PLAY_SE(fighter, Hash40::new("vc_richter_special_h01"));
        }
        if play_vc == 1 {
            PLAY_SE(fighter, Hash40::new("vc_richter_special_n01"));
        }
        if play_vc == 2 {
            PLAY_SE(fighter, Hash40::new("vc_richter_attack07"));
        }
    }
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_special_h01"));
    }
}

unsafe extern "C" fn richter_special_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SIMON_STATUS_SPECIAL_LW_FLAG_GENERATE_HOLYWATER);
        FT_MOTION_RATE_RANGE(fighter, 1.0, 14.0, 10.0);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 1.0);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SIMON_STATUS_SPECIAL_LW_FLAG_SHOOT_HOLYWATER);
    }
}

unsafe extern "C" fn richter_special_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_bottle_appear"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.4, true);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("richter_bottle_release"), Hash40::new("haver"), 0, 6, -2, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn richter_special_lw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_richter_special_l01"));
        PLAY_SE(fighter, Hash40::new("se_richter_special_l01"));
    }
}

unsafe extern "C" fn richter_special_lw_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_lightthrowitem"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn richter_special_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SIMON_STATUS_SPECIAL_LW_FLAG_GENERATE_HOLYWATER);
        FT_MOTION_RATE_RANGE(fighter, 1.0, 14.0, 10.0);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 1.0);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SIMON_STATUS_SPECIAL_LW_FLAG_SHOOT_HOLYWATER);
    }
}

unsafe extern "C" fn richter_special_air_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_bottle_appear"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.4, true);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("richter_bottle_release"), Hash40::new("haver"), 0, 6, -2, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn richter_special_air_lw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_richter_special_l01"));
        PLAY_SE(fighter, Hash40::new("se_richter_special_l01"));
    }
}

unsafe extern "C" fn richter_special_air_lw_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_lightthrowitem"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn richter_special_lw_blank_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("richter_bottle_blank"), Hash40::new("haver"), 0, 6, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn richter_special_lw_blank_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_richter_special_l01"));
        PLAY_SE(fighter, Hash40::new("se_richter_special_l01"));
    }
}

unsafe extern "C" fn richter_special_lw_blank_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_lightthrowitem"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn richter_special_air_lw_blank_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("richter_bottle_blank"), Hash40::new("haver"), 0, 6, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn richter_special_air_lw_blank_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_richter_special_l01"));
        PLAY_SE(fighter, Hash40::new("se_richter_special_l01"));
    }
}

unsafe extern "C" fn richter_special_air_lw_blank_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_lightthrowitem"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    smashline::Agent::new("richter")
        .acmd("game_specialn", richter_special_n_game)
        .acmd("effect_specialn", richter_special_n_effect)
        .acmd("sound_specialn", richter_special_n_sound)
        .acmd("expression_specialn", richter_special_n_expression)
        .acmd("game_specialairn", richter_special_air_n_game)
        .acmd("effect_specialairn", richter_special_air_n_effect)
        .acmd("sound_specialairn", richter_special_air_n_sound)
        .acmd("expression_specialairn", richter_special_air_n_expression)
        .acmd("game_specials1", richter_special_s1_game)
        .acmd("effect_specials1", richter_special_s1_effect)
        .acmd("sound_specials1", richter_special_s1_sound)
        .acmd("expression_specials1", richter_special_s1_expression)
        .acmd("game_specialairs1", richter_special_air_s1_game)
        .acmd("effect_specialairs1", richter_special_air_s1_effect)
        .acmd("sound_specialairs1", richter_special_air_s1_sound)
        .acmd("expression_specialairs1", richter_special_air_s1_expression)
        .acmd("game_specialhi", richter_special_hi_game)
        .acmd("effect_specialhi", richter_special_hi_effect)
        .acmd("sound_specialhi", richter_special_hi_sound)
        .acmd("game_specialairhi", richter_special_air_hi_game)
        .acmd("effect_specialairhi", richter_special_air_hi_effect)
        .acmd("sound_specialairhi", richter_special_air_hi_sound)
        .acmd("game_speciallw", richter_special_lw_game)
        .acmd("effect_speciallw", richter_special_lw_effect)
        .acmd("sound_speciallw", richter_special_lw_sound)
        .acmd("expression_speciallw", richter_special_lw_expression)
        .acmd("game_specialairlw", richter_special_air_lw_game)
        .acmd("effect_specialairlw", richter_special_air_lw_effect)
        .acmd("sound_specialairlw", richter_special_air_lw_sound)
        .acmd("expression_specialairlw", richter_special_air_lw_expression)
        .acmd("effect_speciallwblank", richter_special_lw_blank_effect)
        .acmd("sound_speciallwblank", richter_special_lw_blank_sound)
        .acmd("expression_speciallwblank", richter_special_lw_blank_expression)
        .acmd("effect_specialairlwblank", richter_special_air_lw_blank_effect)
        .acmd("sound_specialairlwblank", richter_special_air_lw_blank_sound)
        .acmd("expression_specialairlwblank", richter_special_air_lw_blank_expression)
        .install();
    smashline::Agent::new("richter_whip")
        .acmd("game_specials1", richter_whip_special_s1_game)
        .acmd("game_specialairs1", richter_whip_special_air_s1_game)
        .install();
    smashline::Agent::new("richter_axe")
        .acmd("game_fly", richter_axe_game)
        .acmd("effect_fly", richter_axe_effect)
        .acmd("sound_fly", richter_axe_sound)
        .install();
}
