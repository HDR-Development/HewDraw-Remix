
use super::*;


#[acmd_script( agent = "tantan", script = "game_specialairn", category = ACMD_GAME, low_priority)]
unsafe fn game_specialairn(fighter: &mut L2CAgentBase) {
    let armType =  WorkModule::get_int(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R);
    frame(fighter.lua_state_agent, 1.0);
    if(armType == 1){
        macros::FT_MOTION_RATE(fighter, 1.35);
    }
    else if (armType==2)
    {
        macros::FT_MOTION_RATE(fighter, 0.75);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("handr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("armr5"), *HIT_STATUS_XLU);
        if(armType == 1){
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr5"), 12.0, 60, 95, 0, 50, 2.5, 1.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("handr"), 12.0, 60, 95, 0, 50, 5.0, 3.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        }
        else if(armType == 2){
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr5"), 6.0, 361, 85, 0, 45, 2.5, 1.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("handr"), 6.0, 361, 85, 0, 45, 3.5, 3.5, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr5"), 8.5, 50, 95, 0, 40, 2.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("handr"), 8.5, 50, 95, 0, 40, 4.5, 3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}
#[acmd_script( agent = "tantan", script = "effect_specialairn", category = ACMD_EFFECT, low_priority)]
unsafe fn effect_specialairn(fighter: &mut L2CAgentBase) {
    let armType =  WorkModule::get_int(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("tantan_jump_line_s"), false, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("tantan_jump_line_l"), false, true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        if(armType==1){
            macros::EFFECT_FOLLOW(fighter, Hash40::new("tantan_atk_air_n2"), Hash40::new("top"), 0, 10, 4, 0, 0, 0, 1.2, true);
        }
        else if (armType==2){
            macros::EFFECT_FOLLOW(fighter, Hash40::new("tantan_atk_air_n3"), Hash40::new("top"), 0, 10, 4, 0, 0, 0, 1.2, true);
        }
        else{
            macros::EFFECT_FOLLOW(fighter, Hash40::new("tantan_atk_air_n1"), Hash40::new("top"), 0, 10, 4, 0, 0, 0, 1.2, true);
        }
    }
}
#[acmd_script( agent = "tantan", script = "sound_specialairn", category = ACMD_SOUND, low_priority)]
unsafe fn sound_specialairn(fighter: &mut L2CAgentBase) {
    let armType =  WorkModule::get_int(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R);
    if(armType==1){
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::PLAY_STATUS(fighter, Hash40::new("se_tantan_attackair_n02"));
            macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_tantan_rnd_attack03"));
        }
    }
    else if (armType==2){
        frame(fighter.lua_state_agent, 7.0);
        if macros::is_excute(fighter) {
            macros::PLAY_STATUS(fighter, Hash40::new("se_tantan_attackair_n03"));
        }
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_tantan_rnd_attack01"));
        }
    }
    else{
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            macros::PLAY_STATUS(fighter, Hash40::new("se_tantan_attackair_n01"));
        }
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_tantan_rnd_attack01"));
        }
    }
}

#[acmd_script( agent = "tantan", script = "expression_attackairf", category = ACMD_EXPRESSION, low_priority)]
unsafe fn expression_specialairn(fighter: &mut L2CAgentBase) {
    let armType =  WorkModule::get_int(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R);
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        if(armType==1){
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_76_megabolthit"), 10);
        }
        else if (armType==2){
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_slashl"), 12);
        }
        else{
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 9);
        }
    }
}


#[acmd_script( agent = "tantan", script = "game_specialairnend", category = ACMD_GAME, low_priority)]
unsafe fn game_specialairnend(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    //Landing lag cancel
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}
#[acmd_script( agent = "tantan", script = "effect_specialairnend", category = ACMD_EFFECT, low_priority)]
unsafe fn effect_specialairnend(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
}
#[acmd_script( agent = "tantan", script = "sound_specialairnend", category = ACMD_SOUND, low_priority)]
unsafe fn sound_specialairnend(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_LANDING_SE(fighter, Hash40::new("se_tantan_landing02"));
    }
}
#[acmd_script( agent = "tantan", script = "expression_specialairnend", category = ACMD_EXPRESSION, low_priority)]
unsafe fn expression_specialairnend(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "tantan_punch1", script = "game_specialairhiattack", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairhiattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("have"), 9.0, 70, 83, 0, 56, 4.5, 3.0, 0.0, 0.3, Some(-6.0), Some(0.0), Some(0.3), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("have"), 9.0, 70, 83, 0, 56, 4.5, 3.0, 0.0, 0.3, Some(0.0), Some(0.0), Some(0.3), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}
#[acmd_script( agent = "tantan_punch1", script = "game_specialairhiattackdragon", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairhiattackdragon(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("have"), 10.35, 70, 83, 0, 66, 5.5, 4.0, 0.0, 0.4, Some(-6.0), Some(0.0), Some(0.4), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("have"), 10.35, 70, 83, 0, 66, 5.5, 4.0, 0.0, 0.4, Some(0.0), Some(0.0), Some(0.4), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

pub fn install() {
    install_acmd_scripts!(
        game_specialairn,
        effect_specialairn,
        sound_specialairn,
        expression_specialairn,
        
        game_specialairnend,
        effect_specialairnend,
        sound_specialairnend,
        expression_specialairnend,

        game_specialairhiattack,
        game_specialairhiattackdragon
    );
}

