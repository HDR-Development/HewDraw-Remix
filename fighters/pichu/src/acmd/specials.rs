
use super::*;


#[acmd_script( agent = "pichu", scripts = ["game_specialn", "game_specialairn"] , category = ACMD_GAME , low_priority)]
unsafe fn pichu_special_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let charged = if fighter.kind() == *FIGHTER_KIND_KIRBY {false} else {VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1};
    let charge_state_time = if fighter.kind() == *FIGHTER_KIND_KIRBY {1} else {ParamModule::get_int(boma.object(), ParamType::Agent, "charge_state_time")};
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK);
        if !charged {
            FT_MOTION_RATE(fighter, (14.0/18.0));
        }
        else if charged {
            VarModule::on_flag(fighter.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK);
            VarModule::sub_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 180);
            MeterModule::drain_direct(boma.object(), (50.0/(charge_state_time as f32)) * 180.0);
        }
    }
    frame(lua_state, 18.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_PICHU_GENERATE_ARTICLE_DENGEKIDAMA, false, -1);
        if !VarModule::is_flag(fighter.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK) {
            MeterModule::add(fighter.battle_object, 2.0);
            FT_ADD_DAMAGE(fighter, 1.0);
        }
        else {
            FT_ADD_DAMAGE(fighter, 3.0);
        }
    }
}
#[acmd_script( agent = "pichu", script = "game_specials" , category = ACMD_GAME , low_priority)]
unsafe fn pichu_special_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let charged = VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1;
    let charge_state_time = ParamModule::get_int(boma.object(), ParamType::Agent, "charge_state_time");
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK);
        if charged {
            VarModule::on_flag(fighter.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK);
            VarModule::sub_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 120);
            MeterModule::drain_direct(boma.object(), (50.0/(charge_state_time as f32)) * 120.0);
        }
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_ATTACK_TRIGGER);
        if !VarModule::is_flag(fighter.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK) {
            FT_ADD_DAMAGE(fighter, 1.0);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 68, 55, 0, 70, 3.2, 0.0, 3.3, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
            WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_CALC_ATTACK_POWER);
        }
        else {
            FT_ADD_DAMAGE(fighter, 3.0);
            FT_DESIRED_RATE(fighter, 18.0, 12.0);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 25.0, 40, 65, 0, 60, 3.2, 0.0, 3.3, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
            wait(lua_state, 9.0);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 40, 65, 0, 60, 3.2, 0.0, 3.3, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
        }
        AttackModule::set_attack_keep_rumble(boma, 0, true);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_BRAKE_TRIGGER);
    }
}
#[acmd_script( agent = "pichu", script = "effect_specials" , category = ACMD_EFFECT , low_priority)]
unsafe fn pichu_special_s_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let charged = VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1;
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("pichu_rocket_bomb"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    frame(lua_state, 5.0);        
    if !VarModule::is_flag(fighter.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK) {
        if is_excute(fighter) {
            EFFECT_FLW_POS(fighter, Hash40::new("pichu_rocket_aura"), Hash40::new("head"), 6.5, 0, 0, 90, 0, 0, 0.9, true);
            EffectModule::enable_sync_init_pos_last(boma);
            LAST_EFFECT_SET_COLOR(fighter, 0.85, 0.9, 1);
            EFFECT_FLW_POS_NO_STOP(fighter, Hash40::new("pichu_rocket_jet"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 0.8 as u64, true);
        }
    }
    else {
        if is_excute(fighter) {
            EFFECT_FLW_POS(fighter, Hash40::new("pichu_rocket_aura_max"), Hash40::new("head"), 6.5, 0, 0, 90, 0, 0, 1.3, true);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FLW_POS_NO_STOP(fighter, Hash40::new("pichu_rocket_jet"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1.25 as u64, true);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_final_sphere_start"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 0.3, true);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.08, 0.661, 1, 0.471);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        FLASH(fighter, 1, 1, 1, 0.392);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    wait(lua_state, 1.0);
}
#[acmd_script( agent = "pichu", scripts = ["game_specialsend", "game_specialairsend"] , category = ACMD_GAME , low_priority)]
unsafe fn pichu_special_s_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK) {
            FT_DESIRED_RATE(fighter, 26.0, 33.0);
        }
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}
#[acmd_script( agent = "pichu", script = "game_specialairsmissend" , category = ACMD_GAME , low_priority)]
unsafe fn pichu_special_air_s_miss_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK) {
            FT_DESIRED_RATE(fighter, 26.0, 33.0);
        }
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_MISS_END_RUMBLE_2);
    }
}
#[acmd_script( agent = "pichu", scripts = ["game_specialhi1", "game_specialairhi1"] , category = ACMD_GAME , low_priority)]
unsafe fn pichu_special_hi_1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let charged = VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1;
    let charge_state_time = ParamModule::get_int(boma.object(), ParamType::Agent, "charge_state_time");
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK);
        if charged {
            VarModule::on_flag(fighter.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK);
            VarModule::sub_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 120);
            MeterModule::drain_direct(boma.object(), (50.0/(charge_state_time as f32)) * 120.0);
        }
        if VarModule::is_flag(fighter.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK) {
            ATTACK(fighter, 0, 0, Hash40::new("neck"), 2.0, 70, 50, 0, 20, 1.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            FT_ADD_DAMAGE(fighter, 1.0);
        }
        else {
            FT_ADD_DAMAGE(fighter, 0.5);
        }
        JostleModule::set_status(boma, false);
    }
}
#[acmd_script( agent = "pichu", scripts = ["game_specialhi2", "game_specialairhi2"] , category = ACMD_GAME , low_priority)]
unsafe fn pichu_special_hi_2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let charged = VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1;
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK) {
            ATTACK(fighter, 0, 0, Hash40::new("neck"), 3.0, 70, 150, 0, 20, 1.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            FT_ADD_DAMAGE(fighter, 2.0);
        }
        else {
            FT_ADD_DAMAGE(fighter, 1.0);
        }
        JostleModule::set_status(boma, false);
    }
}
#[acmd_script( agent = "pichu", scripts = ["game_speciallw", "game_specialairlw"] , category = ACMD_GAME , low_priority)]
unsafe fn pichu_special_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let charged = VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1;
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK);
        if !charged {
            VarModule::off_flag(fighter.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK);
            WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_KAMINARI_GENERATE);
        }
        else {
            let charge_state_time = ParamModule::get_int(boma.object(), ParamType::Agent, "charge_state_time") as f32;
            let charge_state_remaining = VarModule::get_int(boma.object(), vars::common::instance::GIMMICK_TIMER) as f32;
            // 5 seconds to use full strength Discharge before it starts decreasing in power
            let discharge_decrease_power_frame = charge_state_time - 300.0;
            // 50% damage at minimum
            let discharge_min_power_mul = 0.5;
            let discharge_power_mul = 1.0 - ((1.0 - (charge_state_remaining.min(discharge_decrease_power_frame)/discharge_decrease_power_frame)) * (1.0 - discharge_min_power_mul));
            VarModule::set_float(boma.object(), vars::pichu::instance::DISCHARGE_POWER_MUL, discharge_power_mul);
            // 75% size at minimum
            let discharge_min_size_mul = 0.75;
            let discharge_size_mul = 1.0 - ((1.0 - (charge_state_remaining.min(discharge_decrease_power_frame)/discharge_decrease_power_frame)) * (1.0 - discharge_min_size_mul));
            VarModule::set_float(boma.object(), vars::pichu::instance::DISCHARGE_SIZE_MUL, discharge_size_mul);
            VarModule::on_flag(fighter.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK);
            VarModule::set_int(boma.object(), vars::common::instance::GIMMICK_TIMER, 0);
            MeterModule::drain_direct(boma.object(), 999.0);
        }
    }
    if VarModule::is_flag(fighter.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK) {
        frame(lua_state, 7.0);
        if is_excute(fighter) {
            WHOLE_HIT(fighter, *HIT_STATUS_XLU);
        }
        frame(lua_state, 15.0);
        FT_DESIRED_RATE(fighter, 6.0, 3.0);
        if is_excute(fighter) {
            HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        }
        frame(lua_state, 21.0);
        if is_excute(fighter) {
            boma.change_status_req(*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_LW_HIT, true);
        }
    }
}
#[acmd_script( agent = "pichu", scripts = ["game_speciallwhit", "game_specialairlwhit"] , category = ACMD_GAME , low_priority)]
unsafe fn pichu_special_lw_hit_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let discharge_power_mul = VarModule::get_float(boma.object(), vars::pichu::instance::DISCHARGE_POWER_MUL);
    let discharge_size_mul = VarModule::get_float(boma.object(), vars::pichu::instance::DISCHARGE_SIZE_MUL);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK) {
            MeterModule::watch_damage(fighter.battle_object, true);
            MeterModule::add(fighter.battle_object, 2.0);
            FT_ADD_DAMAGE(fighter, 3.5);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 361, 71, 0, 90, 11.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        }
        else {
            FT_ADD_DAMAGE(fighter, 8.0 * discharge_power_mul);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0 * discharge_power_mul, 361, 80, 0, 70, 18.0 * discharge_size_mul, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
            VarModule::set_int(boma.object(), vars::common::instance::GIMMICK_TIMER, 0);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(fighter.battle_object, false);
        if VarModule::is_flag(fighter.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK) {
            FT_MOTION_RATE(fighter, 1.5 * discharge_power_mul);
        }
    }
}
#[acmd_script( agent = "pichu", script = "effect_speciallwhit" , category = ACMD_EFFECT , low_priority)]
unsafe fn pichu_special_lw_hit_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 1.0);
    WorkModule::is_flag(boma, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_KAMINARI_ATTACK_HIT);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK) {
            let discharge_effect_mul = VarModule::get_float(boma.object(), vars::pichu::instance::DISCHARGE_SIZE_MUL);
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_kaminari_hit2"), Hash40::new("hip"), 0, 0, 0, 0, 90, 0, 1.15 * discharge_effect_mul, true);
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_final_explosion"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 0.3 * discharge_effect_mul, false);
        }
        else {
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_kaminari_hit2"), Hash40::new("top"), 0, -2, 0, 0, 90, 0, 1.15, true);
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_kaminari_hit"), Hash40::new("top"), 0, -2, 0, 0, 90, 0, 0.85, true);
        }
    }
    if is_excute(fighter) {
        FLASH(fighter, 0, 0, 0, 0);
        BURN_COLOR(fighter, 2, 2, 0.5, 0.9);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        FLASH_FRM(fighter, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0.7);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        FLASH_FRM(fighter, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        BURN_COLOR_NORMAL(fighter);
        COL_NORMAL(fighter);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pichu_kaminari_hit2"), false, true);
    }
    if is_excute(fighter) {
        FLASH(fighter, 0, 0, 0, 0);
        BURN_COLOR(fighter, 2, 2, 0.5, 0.9);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        FLASH_FRM(fighter, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0.7);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        FLASH_FRM(fighter, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        BURN_COLOR_NORMAL(fighter);
        COL_NORMAL(fighter);
    }
    wait(lua_state, 1.0);
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pichu_kaminari_hit"), false, true);
    }
    for _ in 0..3{
        if is_excute(fighter) {
            FLASH(fighter, 0, 0, 0, 0);
            BURN_COLOR(fighter, 2, 2, 0.5, 0.9);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            FLASH_FRM(fighter, 2, 0, 0, 0, 0);
            BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0.7);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            FLASH_FRM(fighter, 2, 0, 0, 0, 0);
            BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            BURN_COLOR_NORMAL(fighter);
            COL_NORMAL(fighter);
        }
        wait(lua_state, 1.0);
    }
    if is_excute(fighter) {
        FLASH(fighter, 0, 0, 0, 0);
        BURN_COLOR(fighter, 2, 2, 0.5, 0.9);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        FLASH_FRM(fighter, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0.7);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        FLASH_FRM(fighter, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        BURN_COLOR_NORMAL(fighter);
        COL_NORMAL(fighter);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pichu_cheek"), false, true);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "pichu", script = "effect_specialairlwhit" , category = ACMD_EFFECT , low_priority)]
unsafe fn pichu_special_air_lw_hit_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
    }
    frame(lua_state, 1.0);
    WorkModule::is_flag(boma, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_KAMINARI_ATTACK_HIT);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK) {
            let discharge_effect_mul = VarModule::get_float(boma.object(), vars::pichu::instance::DISCHARGE_SIZE_MUL);
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_kaminari_hit2"), Hash40::new("hip"), 0, 0, 0, 0, 90, 0, 1.15 * discharge_effect_mul, true);
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_final_explosion"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 0.3 * discharge_effect_mul, false);
        }
        else {
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_kaminari_hit2"), Hash40::new("top"), 0, -2, 0, 0, 90, 0, 1.15, true);
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_kaminari_hit"), Hash40::new("top"), 0, -2, 0, 0, 90, 0, 0.85, true);
        }
    }
    if is_excute(fighter) {
        FLASH(fighter, 0, 0, 0, 0);
        BURN_COLOR(fighter, 2, 2, 0.5, 0.9);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        FLASH_FRM(fighter, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0.7);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        FLASH_FRM(fighter, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        BURN_COLOR_NORMAL(fighter);
        COL_NORMAL(fighter);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pichu_kaminari_hit2"), false, true);
    }
    if is_excute(fighter) {
        FLASH(fighter, 0, 0, 0, 0);
        BURN_COLOR(fighter, 2, 2, 0.5, 0.9);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        FLASH_FRM(fighter, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0.7);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        FLASH_FRM(fighter, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        BURN_COLOR_NORMAL(fighter);
        COL_NORMAL(fighter);
    }
    wait(lua_state, 1.0);
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pichu_kaminari_hit"), false, true);
    }
    for _ in 0..3{
        if is_excute(fighter) {
            FLASH(fighter, 0, 0, 0, 0);
            BURN_COLOR(fighter, 2, 2, 0.5, 0.9);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            FLASH_FRM(fighter, 2, 0, 0, 0, 0);
            BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0.7);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            FLASH_FRM(fighter, 2, 0, 0, 0, 0);
            BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            BURN_COLOR_NORMAL(fighter);
            COL_NORMAL(fighter);
        }
        wait(lua_state, 1.0);
    }
    if is_excute(fighter) {
        FLASH(fighter, 0, 0, 0, 0);
        BURN_COLOR(fighter, 2, 2, 0.5, 0.9);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        FLASH_FRM(fighter, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0.7);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        FLASH_FRM(fighter, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        BURN_COLOR_NORMAL(fighter);
        COL_NORMAL(fighter);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pichu_cheek"), false, true);
    }
}

pub fn install() {
    install_acmd_scripts!(
        pichu_special_n_game,
        pichu_special_s_game,
        pichu_special_s_effect,
        pichu_special_s_end_game,
        pichu_special_air_s_miss_end_game,
        pichu_special_hi_1_game,
        pichu_special_hi_2_game,
        pichu_special_lw_game,
        pichu_special_lw_hit_game,
        pichu_special_lw_hit_effect,
        pichu_special_air_lw_hit_effect
    );
}

