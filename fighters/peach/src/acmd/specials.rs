use super::*;

unsafe extern "C" fn peach_special_s_hit_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 55, 80, 0, 60, 7.7, 0.0, 5.0, 4.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 6, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HIP);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
            KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

unsafe extern "C" fn peach_special_hi_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KASSAR, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KASSAR, Hash40::new("special_hi_start"), false, 1.0);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PEACH_STATUS_SPECIAL_HI_FLAG_MOVE_TRANS);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("haver"), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 88, 100, 160, 0, 5.0, 0.0, 5.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PARASOL);
        ATTACK(fighter, 1, 0, Hash40::new("head"), 3.0, 100, 100, 130, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PARASOL);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("haver"), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 93, 100, 160, 0, 4.0, 0.0, 10.0, 8.0, Some(0.0), Some(8.0), Some(7.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PARASOL);
        ATTACK(fighter, 1, 0, Hash40::new("havel"), 1.0, 87, 100, 80, 0, 1.5, 2.0, 5.0, 3.5, Some(2.0), Some(2.5), Some(3.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PARASOL);
        ATTACK(fighter, 2, 0, Hash40::new("havel"), 1.0, 367, 100, 70, 0, 5.0, 0.0, 6.5, 0.0, Some(0.0), Some(-1.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PARASOL);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 0, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("havel"), 1.0, 368, 100, 80, 0, 1.5, 2.0, 5.0, 3.5, Some(2.0), Some(2.5), Some(3.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PARASOL);
        ATTACK(fighter, 2, 0, Hash40::new("havel"), 1.0, 368, 100, 70, 0, 5.0, 0.0, 6.5, 0.0, Some(0.0), Some(-1.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PARASOL);
        let hit1 = Vector2f { x: 6.0, y: 25.0 };
        AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &hit1, 12, false);
        AttackModule::set_vec_target_pos(boma, 2, Hash40::new("top"), &hit1, 12, false);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("haver"), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        ATTACK(fighter, 0, 0, Hash40::new("havel"), 4.0, 81, 105, 0, 90, 5.2, 0.0, 4.5, -2.0, Some(0.0), Some(4.5), Some(2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PARASOL);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 4.0, 81, 105, 0, 90, 3.0, 0.0, -1.0, 0.0, Some(0.0), Some(1.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PARASOL);
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

unsafe extern "C" fn peach_special_air_hi_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KASSAR, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KASSAR, Hash40::new("special_hi_start"), false, 1.0);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PEACH_STATUS_SPECIAL_HI_FLAG_MOVE_TRANS);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("haver"), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 88, 100, 160, 0, 5.0, 0.0, 5.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PARASOL);
        ATTACK(fighter, 1, 0, Hash40::new("head"), 3.0, 100, 100, 130, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PARASOL);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("haver"), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 93, 100, 160, 0, 4.0, 0.0, 10.0, 8.0, Some(0.0), Some(8.0), Some(7.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PARASOL);
        ATTACK(fighter, 1, 0, Hash40::new("havel"), 1.0, 87, 100, 80, 0, 1.5, 2.0, 5.0, 3.5, Some(2.0), Some(2.5), Some(3.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PARASOL);
        ATTACK(fighter, 2, 0, Hash40::new("havel"), 1.0, 367, 100, 70, 0, 5.0, 0.0, 6.5, 0.0, Some(0.0), Some(-1.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PARASOL);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 0, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("havel"), 1.0, 368, 100, 80, 0, 1.5, 2.0, 5.0, 3.5, Some(2.0), Some(2.5), Some(3.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PARASOL);
        ATTACK(fighter, 2, 0, Hash40::new("havel"), 1.0, 368, 100, 70, 0, 5.0, 0.0, 6.5, 0.0, Some(0.0), Some(-1.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PARASOL);
        let hit1 = Vector2f { x: 6.0, y: 25.0 };
        AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &hit1, 12, false);
        AttackModule::set_vec_target_pos(boma, 2, Hash40::new("top"), &hit1, 12, false);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("haver"), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        ATTACK(fighter, 0, 0, Hash40::new("havel"), 3.0, 78, 95, 0, 72, 5.2, 0.0, 4.5, -2.0, Some(0.0), Some(4.5), Some(2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PARASOL);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 3.0, 72, 95, 0, 72, 3.0, 0.0, -1.0, 0.0, Some(0.0), Some(1.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PARASOL);
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

unsafe extern "C" fn peach_special_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        let item_kind = fighter.get_int(*FIGHTER_PEACH_STATUS_SPECIAL_LW_WORK_INT_UNIQ_ITEM_KIND); 
        if item_kind == *ITEM_KIND_NONE {
            ArticleModule::generate_article(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_DAIKON, false, -1);
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

unsafe extern "C" fn peach_special_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("peach_hikkonuki"), Hash40::new("top"), 0, 0, 1, 0, 0, 0, 1.0, true);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("peach_hikkonuki"), -1);
        LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn peach_special_lw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_peach_special_l02"));
    }
    wait(lua_state, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_peach_special_l01"));
        if fighter.get_int(*FIGHTER_PEACH_STATUS_SPECIAL_LW_WORK_INT_UNIQ_ITEM_KIND) != *ITEM_KIND_NONE 
        && ItemModule::is_have_item(fighter.module_accessor, 0) {
            PLAY_SE(fighter, Hash40::new("vc_peach_appeal01"));
        } 
    }
}

pub fn install() {
    smashline::Agent::new("peach")
        .acmd("game_specialshitend", peach_special_s_hit_end_game)
        .acmd("game_specialhistart", peach_special_hi_start_game)
        .acmd("game_specialairhistart", peach_special_air_hi_start_game)
        .acmd("game_speciallw", peach_special_lw_game)
        .acmd("effect_speciallw", peach_special_lw_effect)
        .acmd("sound_speciallw", peach_special_lw_sound)
        .install();
}
