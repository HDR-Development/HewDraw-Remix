
use super::*;


#[acmd_script( agent = "pichu", script = "game_attacks4" , category = ACMD_GAME , low_priority)]
unsafe fn pichu_attack_s4_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter){
        if VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) < 2 {
            MeterModule::watch_damage(fighter.battle_object, true);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        FT_ADD_DAMAGE(fighter, 2.5*(VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_RECOIL_MUL)));
    }
    frame(lua_state, 16.0);
    for _ in 0..5 {
        if is_excute(fighter) {
            ATTACK(fighter, 1, 0, Hash40::new("top"), (2.0*(VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_DAMAGE_MUL))), 366, 50, 0, 10, 5.3, 0.0, 4.7, 11.4, None, None, None, 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
            ATTACK(fighter, 2, 0, Hash40::new("top"), (2.0*(VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_DAMAGE_MUL))), 20, 50, 0, 10, 4.3, 0.0, 4.7, 8.0, None, None, None, 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 1.0);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), (8.0*(VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_DAMAGE_MUL))), 45, 130, 0, 65, 5.3, 0.0, 4.7, 11.4, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        ATTACK(fighter, 2, 0, Hash40::new("top"), (8.0*(VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_DAMAGE_MUL))), 45, 130, 0, 65, 4.3, 0.0, 4.7, 8.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(fighter.battle_object, false);
    }
    
}

#[acmd_script( agent = "pichu", script = "game_attackhi4" , category = ACMD_GAME , low_priority)]
unsafe fn pichu_attack_hi4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("neck"), 14.0, 95, 98, 0, 40, 6.0, 4.7, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 1, 0, Hash40::new("hip"), 14.0, 95, 98, 0, 15, 3.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        HIT_NODE(fighter, Hash40::new("mimir1"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("mimil1"), *HIT_STATUS_XLU);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    
}
#[acmd_script( agent = "pichu", script = "game_attacklw4" , category = ACMD_GAME , low_priority)]
unsafe fn pichu_attack_lw4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter){
        if VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) < 2 {
            MeterModule::watch_damage(fighter.battle_object, true);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.0, 2.0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), (12.0*(VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_DAMAGE_MUL))), 55, 120, 0, 45, 6.0, 0.0, 4.5, -5.5, Some(0.0), Some(4.5), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        FT_ADD_DAMAGE(fighter, 1.5*(VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_RECOIL_MUL)));
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), (8.0*(VarModule::get_float(boma.object(), vars::pichu::instance::CHARGE_DAMAGE_MUL))), 55, 90, 0, 45, 4.5, 0.0, 4.0, 9.0, Some(0.0), Some(4.0), Some(-5.5), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(fighter.battle_object, false);
    }

}

pub fn install() {
    install_acmd_scripts!(
        pichu_attack_s4_s_game,
        pichu_attack_hi4_game,
        pichu_attack_lw4_game,
    );
}

