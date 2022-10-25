
use super::*;

#[acmd_script( agent = "luigi", script = "game_specialn" , category = ACMD_GAME , low_priority)]
unsafe fn game_specialn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, vars::luigi::status::IS_SPECIAL_N_FIREBRAND);
        VarModule::off_flag(fighter.battle_object, vars::luigi::status::IS_SPECIAL_N_DOUBLE_FIREBALL);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) {
            VarModule::on_flag(fighter.battle_object, vars::luigi::status::IS_SPECIAL_N_FIREBRAND);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::luigi::status::IS_SPECIAL_N_FIREBRAND){
            ATTACK(fighter, 0, 0, Hash40::new("arml"), 9.0, 69, 55, 0, 65, 4.5, 6.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("arml"), 9.0, 69, 55, 0, 65, 4.5, 2.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("shoulderl"), 9.0, 69, 55, 0, 65, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
            //HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
            //HIT_NODE(fighter, Hash40::new("handl"), *HIT_STATUS_XLU);
            //HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
            FT_MOTION_RATE(fighter, 2.0);
        }
        else{
            ArticleModule::generate_article(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_FIREBALL, false, 0);
        }
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        if VarModule::is_flag(fighter.battle_object, vars::luigi::status::IS_SPECIAL_N_FIREBRAND) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            FT_MOTION_RATE(fighter, 0.5);
        }
    }
    
}

#[acmd_script( agent = "luigi", script = "effect_specialn" , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_specialn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::luigi::status::IS_SPECIAL_N_FIREBRAND){
            EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("havel"), 3.0, 0.0, 0.0, 0, 90, 90, 0.5, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_thunder"), Hash40::new("havel"), 0.0, 0.0, 0.0, 0, 90, 90, 0.8, true);
            LAST_EFFECT_SET_RATE(fighter, 1.25);
            FLASH(fighter, 0, 0.25, 1.0, 0.7);
        }
        else{
            EFFECT_FOLLOW(fighter, Hash40::new("luigi_fb_shoot"), Hash40::new("havel"), 0, 0, 0, -30, 0, 0, 1, true);
            FLASH(fighter, 0, 1, 0, 0.353);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_thunder"), false, false);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_hit_elec_s"), true, true);
    }
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("luigi_fb_shoot"), false, false);
    }
}

#[acmd_script( agent = "luigi", script = "sound_specialn" , category = ACMD_SOUND , low_priority)]
unsafe fn sound_specialn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::luigi::status::IS_SPECIAL_N_FIREBRAND){
            PLAY_SE(fighter, Hash40::new("se_common_elec_s_damage"));
        }
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::luigi::status::IS_SPECIAL_N_FIREBRAND){
            PLAY_SE(fighter, Hash40::new("vc_luigi_attack07"));
            PLAY_SE(fighter, Hash40::new("se_common_elec_m_damage"));
        }
        else{
            PLAY_SE(fighter, Hash40::new("se_luigi_special_n01"));
        }
    }
}

#[acmd_script( agent = "luigi", script = "game_specialairn" , category = ACMD_GAME , low_priority)]
unsafe fn game_specialairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, vars::luigi::status::IS_SPECIAL_N_FIREBRAND);
        VarModule::off_flag(fighter.battle_object, vars::luigi::status::IS_SPECIAL_N_DOUBLE_FIREBALL);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) {
            VarModule::on_flag(fighter.battle_object, vars::luigi::status::IS_SPECIAL_N_FIREBRAND);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::luigi::status::IS_SPECIAL_N_FIREBRAND){
            ATTACK(fighter, 0, 0, Hash40::new("arml"), 9.0, 69, 55, 0, 65, 4.5, 6.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("arml"), 9.0, 69, 55, 0, 65, 4.5, 2.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("shoulderl"), 9.0, 69, 55, 0, 65, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
            //HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
            //HIT_NODE(fighter, Hash40::new("handl"), *HIT_STATUS_XLU);
            //HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
            FT_MOTION_RATE(fighter, 2.0);
        }
        else{
            ArticleModule::generate_article(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_FIREBALL, false, 0);
        }
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        if VarModule::is_flag(fighter.battle_object, vars::luigi::status::IS_SPECIAL_N_FIREBRAND) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            FT_MOTION_RATE(fighter, 0.5);
        }
    }
    
}

#[acmd_script( agent = "luigi", script = "effect_specialairn" , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_specialairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::luigi::status::IS_SPECIAL_N_FIREBRAND){
            EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("havel"), 3.0, 0.0, 0.0, 0, 90, 90, 0.5, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_thunder"), Hash40::new("havel"), 0.0, 0.0, 0.0, 0, 90, 90, 0.8, true);
            LAST_EFFECT_SET_RATE(fighter, 1.25);
            FLASH(fighter, 0, 0.25, 1.0, 0.7);
        }
        else{
            EFFECT_FOLLOW(fighter, Hash40::new("luigi_fb_shoot"), Hash40::new("havel"), 0, 0, 0, -30, 0, 0, 1, true);
            FLASH(fighter, 0, 1, 0, 0.353);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_thunder"), false, false);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_hit_elec_s"), true, true);
    }
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("luigi_fb_shoot"), false, false);
    }
}

#[acmd_script( agent = "luigi", script = "sound_specialairn" , category = ACMD_SOUND , low_priority)]
unsafe fn sound_specialairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::luigi::status::IS_SPECIAL_N_FIREBRAND){
            PLAY_SE(fighter, Hash40::new("se_common_elec_s_damage"));
        }
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::luigi::status::IS_SPECIAL_N_FIREBRAND){
            PLAY_SE(fighter, Hash40::new("vc_luigi_attack07"));
            PLAY_SE(fighter, Hash40::new("se_common_elec_m_damage"));
        }
        else{
            PLAY_SE(fighter, Hash40::new("se_luigi_special_n01"));
        }
    }
}

#[acmd_script(agent = "luigi", script = "game_specialairsstart", category = ACMD_GAME)]
unsafe fn game_specialairsstart(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        KineticModule::mul_speed(boma, &Vector3f{x: 1.0, y: 0.0, z: 1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_FLAG_REVERSE_LR);
    }
}

#[acmd_script(agent = "luigi", script = "effect_specialshold", category = ACMD_EFFECT)]
unsafe fn effect_specialshold(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("luigi_rocket_hold"), Hash40::new("top"), 0, 6, 0,  0, 1, 0, 1, true);
        let handle = EffectModule::get_last_handle(fighter.module_accessor) as u32;
        VarModule::set_int(fighter.battle_object, vars::luigi::instance::CHARGE_PULSE_EFFECT_HANDLE, handle as i32);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE) {
            LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.8, 0.0);
        }
    }
    for _ in 0..100 {
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 10, 00, 4, 0, 0, 0, false);
            let handle = EffectModule::get_last_handle(fighter.module_accessor) as u32;
            VarModule::set_int(fighter.battle_object, vars::luigi::instance::CHARGE_SMOKE_EFFECT_HANDLE, handle as i32);
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE) {
                LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.7, 0.3);
            }
        }
        wait(lua_state, 10.0);
    }
}

#[acmd_script(agent = "luigi", script = "effect_specialairshold", category = ACMD_EFFECT)]
unsafe fn effect_specialairshold(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("luigi_rocket_hold"), Hash40::new("top"), 0, 6, 0,  0, 1, 0, 1, true);
        let handle = EffectModule::get_last_handle(fighter.module_accessor) as u32;
        VarModule::set_int(fighter.battle_object, vars::luigi::instance::CHARGE_PULSE_EFFECT_HANDLE, handle as i32);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE) {
            LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.8, 0.0);
        }
    }
}

#[acmd_script(agent = "luigi", script = "game_specials", category = ACMD_GAME)]
unsafe fn game_specials(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        SA_SET(fighter, *SITUATION_KIND_AIR);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_LUIGI_SPECIAL_S_RAM);
        CORRECT(fighter, *GROUND_CORRECT_KIND_AIR);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("neck"), 10.0, 361, 80, 0, 30, 4.8, 1.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
        AttackModule::set_attack_keep_rumble(boma, 0, true);
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_GROUND_CHECK);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("neck"), 10.0, 361, 80, 0, 30, 4.8, 1.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_GROUND);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_BRAKE);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        //WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_BRAKE);
    }
}

#[acmd_script(agent = "luigi", script = "game_specialsdischarge", category = ACMD_GAME)]
unsafe fn game_specialsdischarge(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let misfire_multiplier = VarModule::get_float(fighter.battle_object, vars::luigi::instance::MISFIRE_DAMAGE_MULTIPLIER);
    VarModule::set_float(fighter.battle_object, vars::luigi::instance::MISFIRE_DAMAGE_MULTIPLIER, 1.0);
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        SA_SET(fighter, *SITUATION_KIND_AIR);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_LUIGI_SPECIAL_S_RAM);
        CORRECT(fighter, *GROUND_CORRECT_KIND_AIR);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("neck"), 25.0 * misfire_multiplier, 361, 100, 0, 20, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
        AttackModule::set_attack_keep_rumble(fighter.module_accessor, 0, true);
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_GROUND_CHECK);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("neck"), 25.0 * misfire_multiplier, 361, 100, 0, 20, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_GROUND);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_BRAKE);
    }
}

#[acmd_script(agent = "luigi", script = "effect_specialsdischarge", category = ACMD_EFFECT)]
unsafe fn effect_specialsdischarge(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("luigi_rocket_bomb"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.8, 0.0);
    }

    frame(lua_state, 3.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("luigi_rocket_jet2"), Hash40::new("top"), 0, 4, 4, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.7, 0.3);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
}

#[acmd_script(agent = "luigi", script = "game_specialairsend", category = ACMD_GAME)]
unsafe fn game_specialairsend(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.8);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "luigi", script = "game_speciallw" , category = ACMD_GAME , low_priority)]
unsafe fn game_speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 90, 80, 0, 50, 5.5, 0.0, 10.0, -5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 90, 80, 0, 50, 5.5, 0.0, 10.0, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 90, 80, 0, 50, 6.0, 0.0, 5.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.6);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 44.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 90, 80, 0, 50, 7.5, 0.0, 11.0, -9.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 90, 80, 0, 50, 7.5, 0.0, 11.0, 9.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 12.0, 90, 80, 0, 50, 6.5, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        //KineticModule::mul_speed(boma, &Vector3f{x: 0.0, y: 1.0, z: 1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        //KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
        WorkModule::off_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "luigi", script = "game_specialairlw" , category = ACMD_GAME , low_priority)]
unsafe fn game_specialairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) < 0.0 {
            KineticModule::mul_speed(boma, &Vector3f{x: 1.0, y: 0.1, z: 1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 367, 30, 0, 80, 6.0, 0.0, 9.5, 5.5, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 367, 30, 0, 80, 6.0, 0.0, 9.5, -5.5, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 367, 30, 0, 80, 6.0, 0.0, 2.5, 0.0, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        //ATTACK(fighter, 4, 1, Hash40::new("top"), 0.0, 180, 100, 45, 0, 15.5, 0.0, 8.5, 0.0, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 10, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        let hitVec = Vector2f{ x: 0.0, y: 8.0 };
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hitVec, 8, false);
        AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &hitVec, 8, false);
        AttackModule::set_vec_target_pos(boma, 2, Hash40::new("top"), &hitVec, 8, false);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.6);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 44.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 70, 120, 0, 85, 7.5, 0.0, 11.0, -9.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 70, 120, 0, 85, 7.5, 0.0, 11.0, 9.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 90, 120, 0, 85, 6.5, 0.0, 2.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
        WorkModule::off_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BUOYANCY);
        AttackModule::clear_all(boma);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        game_specialn,
        game_specialairn,
        game_specialairsstart,
        game_specials,
        game_specialsdischarge,
        game_specialairsend,
        game_speciallw,
        game_specialairlw,

        effect_specialn,
        effect_specialairn,
        effect_specialshold,
        effect_specialairshold,
        effect_specialsdischarge,

        sound_specialn,
        sound_specialairn,
    );
}

