
use super::*;


#[acmd_script( agent = "packun", script = "game_attacks4" , category = ACMD_GAME , low_priority)]
unsafe fn packun_attack_s4_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let stance = StanceInfo::from(VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE));
    if stance.label == 2 {
        FT_MOTION_RATE(fighter, (24.0/16.0));
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 16.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        let kbg = if stance.label == 0 { 110 } else if stance.label == 1 { 140 } else { 100 };
        let hitlag = if stance.label != 2 { 1.2 } else { 1.5 };
        let sound = if stance.label != 2 { *COLLISION_SOUND_ATTR_PUNCH } else { *COLLISION_SOUND_ATTR_HEAVY };
        ATTACK(fighter, 0, 0, Hash40::new("virtualhit2"), 14.0 * stance.damage_head, 45, 100, 0, 25, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("virtualhit3"), 14.0 * stance.damage_head, 45, 100, 0, 25, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("mouth"), 15.0 * stance.damage_head, 45, kbg, 0, 25, 7.0, 2.5, 0.0, 0.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_HEAD);
        HIT_NODE(fighter, Hash40::new("mouth"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("lipu3"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("lipd3"), *HIT_STATUS_XLU);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HIT_NODE(fighter, Hash40::new("mouth"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("lipu3"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("lipd3"), *HIT_STATUS_NORMAL);
    }
    
}

#[acmd_script( agent = "packun", script = "game_attackhi4" , category = ACMD_GAME , low_priority)]
unsafe fn packun_attack_hi4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let stance = StanceInfo::from(VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE));
    if stance.label == 2 {
        FT_MOTION_RATE(fighter, (15.0/12.0));
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 12.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0 * stance.damage_bite, 75, 0, 0, 75, 5.5, 0.0, 12.0, -4.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0 * stance.damage_bite, 105, 0, 0, 75, 5.5, 0.0, 12.0, 4.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        HIT_NODE(fighter, Hash40::new("mouth"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("lipu3"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("lipd3"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        if stance.label == 1 {
            ATTACK(fighter, 0, 0, Hash40::new("mouth"), 12.5 * stance.damage_bite, 90, 86, 0, 90, 9.0, 2.0, 0.0, 0.0, Some(3.5), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            AttackModule::set_poison_param(boma, 0, 136, 45, 4.0, false);
        }
        else {
            ATTACK(fighter, 0, 0, Hash40::new("mouth"), 12.5 * stance.damage_bite, 90, 86, 0, 90, 9.0, 2.0, 0.0, 0.0, Some(3.5), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        }
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HIT_NODE(fighter, Hash40::new("mouth"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("lipu3"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("lipd3"), *HIT_STATUS_NORMAL);
    }
    
}

#[acmd_script( agent = "packun", script = "game_attacklw4" , category = ACMD_GAME , low_priority)]
unsafe fn packun_attack_lw4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let stance = StanceInfo::from(VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE));
    let angle1 = if stance.label == 0 { 175 } else { 32 };
    let angle2 = if stance.label == 0 { 94 } else { 30 };
    let dmg1 = if stance.label == 0 { 7.0 } else { 14.0 };
    let dmg2 = if stance.label == 0 { 6.0 } else { 12.0 };
    let fkb1 = if stance.label == 0 { 150 } else { 0 };
    let bkb1 = if stance.label == 0 { 0 } else { 25 }; 
    let kbg2 = if stance.label == 0 { 70 } else { 99 };
    let bkb2 = if stance.label == 0 { 75 } else { 25 };
    let element = if stance.label == 0 { Hash40::new("collision_attr_fire") } else { Hash40::new("collision_attr_normal") };
    let lvl1 = if stance.label == 0 { *ATTACK_SOUND_LEVEL_M } else { *ATTACK_SOUND_LEVEL_L };
    if stance.label == 2 {
        FT_MOTION_RATE(fighter, (17.0/14.0));
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 7.0);
    if stance.label == 0 {
        FT_DESIRED_RATE(fighter, 7.0, 3.0);
    }
    frame(lua_state, 14.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), dmg1 * stance.damage_other, angle1, 99, fkb1, bkb1, 5.0, 0.0, 4.0, 12.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, element, lvl1, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), dmg1 * stance.damage_other, angle1, 99, fkb1, bkb1, 4.0, 0.0, 3.5, 7.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, element, lvl1, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        if stance.label == 0 {
            VarModule::on_flag(boma.object(), vars::packun::status::FLAME_ACTIVE);
        }
        else {
            AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), dmg2 * stance.damage_other, angle2, kbg2, 0, bkb2, 5.0, 0.0, 4.0, -13.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, element, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), dmg2 * stance.damage_other, angle2, kbg2, 0, bkb2, 4.0, 0.0, 3.6, -8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, element, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        if stance.label != 0 {
            AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        VarModule::off_flag(boma.object(), vars::packun::status::FLAME_ACTIVE);
    }
    
}

#[acmd_script( agent = "packun", script = "effect_attacklw4", category = ACMD_EFFECT, low_priority )]
unsafe fn packun_attack_lw4_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE));
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT_FLIP(agent, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), 5, 4, 9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 1, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 11.0);
    for _ in 0..3 {
        if stance.label == 0{
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, Hash40::new("packun_atk_air_b_fire"), Hash40::new("potc"), 0, 0, 0, 0, 0, 0, 1.5, true);
            }
        }
        wait(lua_state, 2.0);
    }
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("packun_smash_lw_arc"), Hash40::new("packun_smash_lw_arc"), Hash40::new("top"), 0, 5, 3, -180, 170, 7, 1.4, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("packun_smash_lw_arc"), true, true);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("packun_smash_lw_arc"), Hash40::new("packun_smash_lw_arc"), Hash40::new("top"), 0, 5, -4, -180, 20, 5, 1.4, true, *EF_FLIP_YZ);
    }
}

pub fn install() {
    install_acmd_scripts!(
        packun_attack_s4_s_game,
        packun_attack_hi4_game,
        packun_attack_lw4_game,
        packun_attack_lw4_effect,
    );
}

