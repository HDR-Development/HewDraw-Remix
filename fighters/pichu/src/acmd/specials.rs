
use super::*;


#[acmd_script( agent = "pichu", script = "game_specialn" , category = ACMD_GAME , low_priority)]
unsafe fn pichu_special_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1 {
        if is_excute(fighter) {
            VarModule::sub_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 60);
        }
        frame(lua_state, 1.0);
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, 1.1);
        }
        frame(lua_state, 18.0);
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, 1.0);
            ArticleModule::generate_article(boma, *FIGHTER_PICHU_GENERATE_ARTICLE_DENGEKIDAMA, false, -1);
        }
        if is_excute(fighter) {
            FT_ADD_DAMAGE(fighter, 3.0);
        }
    }
    else if VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 0 {
        frame(lua_state, 1.0);
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, 0.7);
        }
        frame(lua_state, 18.0);
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, 1.0);
            ArticleModule::generate_article(boma, *FIGHTER_PICHU_GENERATE_ARTICLE_DENGEKIDAMA, false, -1);
            MeterModule::add(fighter.battle_object, 3.0);
        }
        if is_excute(fighter) {
            FT_ADD_DAMAGE(fighter, 1.0);
        }
    }
}

#[acmd_script( agent = "pichu", script = "game_specialairn" , category = ACMD_GAME , low_priority)]
unsafe fn pichu_special_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1 {
        if is_excute(fighter) {
            VarModule::sub_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 60);
        }
        frame(lua_state, 1.0);
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, 1.1);
        }
        frame(lua_state, 18.0);
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, 1.0);
            ArticleModule::generate_article(boma, *FIGHTER_PICHU_GENERATE_ARTICLE_DENGEKIDAMA, false, -1);
        }
        if is_excute(fighter) {
            FT_ADD_DAMAGE(fighter, 3.0);
        }
    }
    else if VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 0 {
        frame(lua_state, 1.0);
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, 0.7);
        }
        frame(lua_state, 18.0);
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, 1.0);
            ArticleModule::generate_article(boma, *FIGHTER_PICHU_GENERATE_ARTICLE_DENGEKIDAMA, false, -1);
            MeterModule::add(fighter.battle_object, 3.0);
        }
        if is_excute(fighter) {
            FT_ADD_DAMAGE(fighter, 1.0);
        }
    }
}

#[acmd_script( agent = "pichu", script = "game_specials" , category = ACMD_GAME , low_priority)]
unsafe fn pichu_special_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1 {
            VarModule::sub_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 60);
        }
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_ATTACK_TRIGGER);
        if VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 0 {
            FT_ADD_DAMAGE(fighter, 1.5);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 68, 55, 0, 70, 3.2, 0.0, 3.3, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
            WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_CALC_ATTACK_POWER);
        }
        else if VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1 {
            FT_ADD_DAMAGE(fighter, 4.5);
            FT_MOTION_RATE(fighter, 0.5);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 30.0, 40, 55, 0, 40, 3.2, 0.0, 3.3, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
        }
        AttackModule::set_attack_keep_rumble(boma, 0, true);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, false);
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_BRAKE_TRIGGER);
    }
}
#[acmd_script( agent = "pichu", script = "game_speciallw" , category = ACMD_GAME , low_priority)]
unsafe fn pichu_special_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 0 {
        if is_excute(fighter) {
            WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_KAMINARI_GENERATE);
        }
    }
    else if VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1 {
        frame(lua_state, 20.0);
        if is_excute(fighter) {
            boma.change_status_req(*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_LW_HIT, true);
        }
    }
}
#[acmd_script( agent = "pichu", script = "game_specialairlw" , category = ACMD_GAME , low_priority)]
unsafe fn pichu_special_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 0 {
        if is_excute(fighter) {
            WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_KAMINARI_GENERATE);
        }
    }
    else if VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1 {
        frame(lua_state, 20.0);
        if is_excute(fighter) {
            boma.change_status_req(*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_LW_HIT, true);
        }
    }
}
#[acmd_script( agent = "pichu", script = "game_speciallwhit" , category = ACMD_GAME , low_priority)]
unsafe fn pichu_special_lw_hit_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 0 {
        if is_excute(fighter) {
            MeterModule::watch_damage(fighter.battle_object, true);
            FT_ADD_DAMAGE(fighter, 3.5);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 361, 71, 0, 90, 11.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        }
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            MeterModule::watch_damage(fighter.battle_object, false);
        }
    }
    else if VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1 {
        if is_excute(fighter){
            FT_ADD_DAMAGE(fighter, 20.0);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 24.0, 361, 71, 0, 40, 15.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        }
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            FT_MOTION_RATE(fighter, 2.0);
            VarModule::set_int(boma.object(), vars::common::instance::GIMMICK_TIMER, 0);
        }
    }
}

#[acmd_script( agent = "pichu", script = "game_specialairlwhit" , category = ACMD_GAME , low_priority)]
unsafe fn pichu_special_air_lw_hit_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 0 {
        if is_excute(fighter) {
            MeterModule::watch_damage(fighter.battle_object, true);
            FT_ADD_DAMAGE(fighter, 3.5);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 361, 71, 0, 90, 11.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        }
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            MeterModule::watch_damage(fighter.battle_object, false);
        }
    }
    else if VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1 {
        if is_excute(fighter){
            FT_ADD_DAMAGE(fighter, 20.0);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 24.0, 361, 71, 0, 40, 15.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        }
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            FT_MOTION_RATE(fighter, 2.0);
            VarModule::set_int(boma.object(), vars::common::instance::GIMMICK_TIMER, 0);
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        pichu_special_n_game,
        pichu_special_air_n_game,
        pichu_special_s_game,
        pichu_special_lw_game,
        pichu_special_air_lw_game,
        pichu_special_lw_hit_game,
        pichu_special_air_lw_hit_game,
    );
}

