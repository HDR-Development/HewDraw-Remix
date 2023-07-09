
use super::*;


#[acmd_script( agent = "pichu", script = "game_attackairn" , category = ACMD_GAME , low_priority)]
unsafe fn pichu_attack_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(fighter, 0, 0, Hash40::new("neck"), 10.0, 361, 105, 0, 35, 4.5, 5.0, 0.0, 0.0, Some(-1.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("neck"), 7.0, 361, 95, 0, 30, 4.5, 5.0, 0.0, 0.0, Some(-1.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 18.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "pichu", script = "game_attackairf" , category = ACMD_GAME , low_priority)]
unsafe fn pichu_attack_air_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let recoil_mul = VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_RECOIL_MUL);
    let damage_mul = VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_DAMAGE_MUL);
    let charged = VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1;
    if is_excute(fighter){
        if !charged {
            MeterModule::watch_damage(fighter.battle_object, true);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        FT_ADD_DAMAGE(fighter, 2.0 * recoil_mul);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, (4.0/6.0));
        if !charged {
            MeterModule::add(fighter.battle_object, 2.0);
        }
        ATTACK(fighter, 0, 0, Hash40::new("neck"), 12.5 * damage_mul, 45, 100, 0, 35, 5.6, 2.2, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 1, 0, Hash40::new("hip"), 12.5 * damage_mul, 45, 100, 0, 35, 4.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
    }
    wait(lua_state, 6.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, (5.0/6.0));
        ATTACK(fighter, 0, 0, Hash40::new("neck"), 5.5 * damage_mul, 361, 85, 0, 35, 5.6, 2.2, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 1, 0, Hash40::new("hip"), 5.5 * damage_mul, 361, 85, 0, 35, 4.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(fighter.battle_object, false);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "pichu", script = "expression_attackairf", category = ACMD_EXPRESSION, low_priority )]
unsafe fn pichu_attack_air_b_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
    }
}

#[acmd_script( agent = "pichu", script = "game_attackairb" , category = ACMD_GAME , low_priority)]
unsafe fn pichu_attack_air_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let recoil_mul = VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_RECOIL_MUL);
    let damage_mul = VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_DAMAGE_MUL);
    let charged = VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1;
    if is_excute(fighter){
        if !charged {
            MeterModule::watch_damage(fighter.battle_object, true);
        }
    }
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 2.0);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FT_ADD_DAMAGE(fighter, 2.0 * recoil_mul);
        if !charged {
            MeterModule::add(fighter.battle_object, 2.0);
        }
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0 * damage_mul, 365, 100, 50, 0, 4.0, 0.0, 1.8, -11.5, Some(0.0), Some(1.8), Some(0.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0 * damage_mul, 365, 100, 50, 0, 4.0, 0.0, 1.8, -11.5, Some(0.0), Some(1.8), Some(0.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(fighter.battle_object, false);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        if !charged {
            MeterModule::watch_damage(fighter.battle_object, true);
        }
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0 * damage_mul, 365, 100, 50, 0, 3.5, 0.0, 2.5, -10.5, Some(0.0), Some(2.5), Some(-0.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        AttackModule::set_add_reaction_frame(boma, 1, 2.0, false);
        FT_MOTION_RATE(fighter, 0.6154);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(fighter.battle_object, false);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        if !charged {
            MeterModule::watch_damage(fighter.battle_object, true);
        }
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0 * damage_mul, 365, 100, 50, 0, 3.5, 0.0, 2.5, -10.5, Some(0.0), Some(2.5), Some(-0.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(fighter.battle_object, false);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        if !charged {
            MeterModule::watch_damage(fighter.battle_object, true);
        }
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5 * damage_mul, 42, 257, 0, 35, 5.0, 0.0, 3.0, -10.5, Some(0.0), Some(3.0), Some(-0.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(fighter.battle_object, false);
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "pichu", script = "game_attackairhi" , category = ACMD_GAME , low_priority)]
unsafe fn pichu_attack_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let charged = VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1;
    frame(lua_state, 4.0);
    FT_DESIRED_RATE(fighter, 7.0, 6.5);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(fighter, 0, 0, Hash40::new("tail1"), 7.0, 63, 95, 0, 45, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        ATTACK(fighter, 1, 0, Hash40::new("tail3"), 7.0, 63, 95, 0, 45, 6.0, 1.5, -0.2, -0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
    }
    wait(lua_state, 6.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 11.5, 12.0, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "pichu", script = "game_attackairlw" , category = ACMD_GAME , low_priority)]
unsafe fn pichu_attack_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let recoil_mul = VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_RECOIL_MUL);
    let damage_mul = VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_DAMAGE_MUL);
    let charged = VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1;
    if is_excute(fighter){
        if !charged {
            MeterModule::watch_damage(fighter.battle_object, true);
        }
    }
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        FT_ADD_DAMAGE(fighter, 2.0 * recoil_mul);
        if !charged {
            MeterModule::add(fighter.battle_object, 2.0);
        }
        HIT_NODE(fighter, Hash40::new("mimir1"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("mimil1"), *HIT_STATUS_XLU);
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 13.0 * damage_mul, 361, 110, 0, 40, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 1, 0, Hash40::new("neck"), 13.0 * damage_mul, 361, 110, 0, 40, 5.5, 4.5, -1.3, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 11.0 * damage_mul, 361, 104, 0, 35, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 1, 0, Hash40::new("neck"), 11.0 * damage_mul, 361, 104, 0, 35, 5.0, 4.5, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(fighter.battle_object, false);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        pichu_attack_air_n_game,
        pichu_attack_air_f_game,
        pichu_attack_air_b_expression,
        pichu_attack_air_b_game,
        pichu_attack_air_hi_game,
        pichu_attack_air_lw_game,
    );
}

