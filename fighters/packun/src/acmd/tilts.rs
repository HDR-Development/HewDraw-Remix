
use super::*;


#[acmd_script( agent = "packun", script = "game_attacks3" , category = ACMD_GAME , low_priority)]
unsafe fn packun_attack_s3_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let stance = StanceInfo::from(VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE));
    let shield_damage = if stance.label != 2 { 0 } else { 2 };
    if stance.label == 2 {
        FT_MOTION_RATE(fighter, (9.0/7.0));
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5 * stance.damage_bite, 366, 20, 0, 20, 6.3, 0.0, 7.5, 12.0, Some(0.0), Some(10.5), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, shield_damage, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        ATTACK(fighter, 1, 0, Hash40::new("virtualhit3"), 5.5 * stance.damage_bite, 361, 20, 0, 20, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, shield_damage, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        ATTACK(fighter, 2, 0, Hash40::new("virtualhit2"), 5.5 * stance.damage_bite, 361, 20, 0, 20, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, shield_damage, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        AttackModule::set_add_reaction_frame(boma, 0, 11.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 11.0, false);
        AttackModule::set_add_reaction_frame(boma, 2, 11.0, false);
        HIT_NODE(fighter, Hash40::new("lipu3"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("lipd3"), *HIT_STATUS_XLU);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HIT_NODE(fighter, Hash40::new("lipu3"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("lipd3"), *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    
}

#[acmd_script( agent = "packun", script = "game_attacks32" , category = ACMD_GAME , low_priority)]
unsafe fn packun_attack_s3_s2_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE));
    let shield_damage = if stance.label != 2 { 0 } else { 2 };
    if stance.label == 2 {
        FT_MOTION_RATE(agent, (7.0/5.0));
    }
    frame(lua_state, 5.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        if stance.label == 1 {
            ATTACK(agent, 0, 0, Hash40::new("top"), 6.0 * stance.damage_bite, 361, 125, 0, 40, 6.5, 0.0, 7.5, 15.0, Some(0.0), Some(10.5), Some(15.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, shield_damage, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            ATTACK(agent, 1, 0, Hash40::new("virtualhit3"), 6.0 * stance.damage_bite, 361, 125, 0, 40, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, shield_damage, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            ATTACK(agent, 2, 0, Hash40::new("virtualhit2"), 6.0 * stance.damage_bite, 361, 125, 0, 40, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, shield_damage, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            AttackModule::set_poison_param(boma, 0, 136, 45, 2.0, false);
            AttackModule::set_poison_param(boma, 1, 136, 45, 2.0, false);
            AttackModule::set_poison_param(boma, 2, 136, 45, 2.0, false);
        }
        else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 6.0 * stance.damage_bite, 361, 125, 0, 40, 6.5, 0.0, 7.5, 15.0, Some(0.0), Some(10.5), Some(15.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, shield_damage, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            ATTACK(agent, 1, 0, Hash40::new("virtualhit2"), 6.0 * stance.damage_bite, 361, 125, 0, 40, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, shield_damage, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            ATTACK(agent, 2, 0, Hash40::new("virtualhit3"), 6.0 * stance.damage_bite, 361, 125, 0, 40, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, shield_damage, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            if stance.label == 2 {
                HIT_NODE(agent, Hash40::new("mouth"), *HIT_STATUS_XLU);
                HIT_NODE(agent, Hash40::new("lipu3"), *HIT_STATUS_XLU);
                HIT_NODE(agent, Hash40::new("lipd3"), *HIT_STATUS_XLU);
                HIT_NODE(agent, Hash40::new("neck6"), *HIT_STATUS_XLU);
                HIT_NODE(agent, Hash40::new("neck8"), *HIT_STATUS_XLU);
            }
        }
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HIT_NODE(agent, Hash40::new("mouth"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("lipu3"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("lipd3"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("neck6"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("neck8"), *HIT_STATUS_NORMAL);
    }
    
}

#[acmd_script( agent = "packun", script = "game_attackhi3" , category = ACMD_GAME , low_priority)]
unsafe fn packun_attack_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let stance = StanceInfo::from(VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE));
    let sound_level_neck = if stance.label != 2 { *ATTACK_SOUND_LEVEL_S } else { *ATTACK_SOUND_LEVEL_S };
    let sound_neck = if stance.label != 2 { *COLLISION_SOUND_ATTR_PUNCH } else { *COLLISION_SOUND_ATTR_HEAVY };
    let sound_level_head = if stance.label != 2 { *ATTACK_SOUND_LEVEL_M } else { *ATTACK_SOUND_LEVEL_M };
    let sound_head = if stance.label != 2 { *COLLISION_SOUND_ATTR_PUNCH } else { *COLLISION_SOUND_ATTR_HEAVY };
    if stance.label == 2 {
        FT_MOTION_RATE(fighter, (9.0/6.0));
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("virtualhit1"), 7.0 * stance.damage_head, 80, 100, 0, 55, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), sound_level_neck, sound_neck, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("virtualhit2"), 7.0 * stance.damage_head, 80, 100, 0, 55, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), sound_level_neck, sound_neck, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("virtualhit3"), 7.0 * stance.damage_head, 80, 100, 0, 55, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), sound_level_neck, sound_neck, *ATTACK_REGION_BODY);
        ATTACK(fighter, 3, 0, Hash40::new("mouth"), 9.0 * stance.damage_head, 85, 100, 0, 55, 6.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), sound_level_head, sound_head, *ATTACK_REGION_HEAD);
        HIT_NODE(fighter, Hash40::new("mouth"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("lipu3"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("lipd3"), *HIT_STATUS_XLU);
    }
    wait(lua_state, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HIT_NODE(fighter, Hash40::new("mouth"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("lipu3"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("lipd3"), *HIT_STATUS_NORMAL);
    }
    
}

#[acmd_script( agent = "packun", script = "game_attacklw3" , category = ACMD_GAME , low_priority)]
unsafe fn packun_attack_lw3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let stance = StanceInfo::from(VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE));
    let bkb = if stance.label == 1 {10} else {0};
    if stance.label != 2 {
        FT_DESIRED_RATE(fighter, 6.0, 3.0);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0 * stance.damage_other, 73, 75, 0, 60 + bkb, 5.0, 0.0, 4.0, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0 * stance.damage_other, 73, 75, 0, 60 + bkb, 5.0, 0.0, 3.4, 9.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0 * stance.damage_other, 86, 75, 0, 60 + bkb, 5.0, 0.0, 2.8, 16.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "packun", script = "effect_attacklw3", category = ACMD_EFFECT, low_priority )]
unsafe fn packun_attack_lw3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 8.0);
    if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_lw_arc"), Hash40::new("top"), 0.7, 4.6, 4.6, -170, -20, -12, 1, true);
        }
    }
    else {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_lw_arc"), Hash40::new("top"), -0.7, 4.6, 4.6, -10, 200, 12, 1, true);
        }
    }
}


pub fn install() {
    install_acmd_scripts!(
        packun_attack_s3_s_game,
        packun_attack_s3_s2_game,
        packun_attack_hi3_game,
        packun_attack_lw3_game,
        packun_attack_lw3_effect,
    );
}

