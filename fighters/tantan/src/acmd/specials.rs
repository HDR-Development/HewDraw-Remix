
use super::*;


#[acmd_script( agent = "tantan", script = "game_specialairn", category = ACMD_GAME, low_priority)]
unsafe fn tantan_special_n_air_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let armType =  WorkModule::get_int(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R);
    frame(lua_state, 1.0);
    if(armType == 1){
        FT_MOTION_RATE(fighter, 1.35);
    }
    else if (armType==2)
    {
        FT_MOTION_RATE(fighter, 0.75);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("handr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr5"), *HIT_STATUS_XLU);
        if(armType == 1){
            ATTACK(fighter, 0, 0, Hash40::new("armr5"), 12.0, 60, 95, 0, 50, 2.5, 1.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("handr"), 12.0, 60, 95, 0, 50, 5.0, 3.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        }
        else if(armType == 2){
            ATTACK(fighter, 0, 0, Hash40::new("armr5"), 6.0, 361, 85, 0, 45, 2.5, 1.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("handr"), 6.0, 361, 85, 0, 45, 3.5, 3.5, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        }
        else {
            ATTACK(fighter, 0, 0, Hash40::new("armr5"), 9.5, 45, 95, 0, 45, 2.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("handr"), 9.5, 45, 95, 0, 45, 4.5, 3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}
#[acmd_script( agent = "tantan", script = "effect_specialairn", category = ACMD_EFFECT, low_priority)]
unsafe fn tantan_special_n_air_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let armType =  WorkModule::get_int(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("tantan_jump_line_s"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("tantan_jump_line_l"), false, true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if(armType==1){
            EFFECT_FOLLOW(fighter, Hash40::new("tantan_atk_air_n2"), Hash40::new("top"), 0, 10, 4, 0, 0, 0, 1.2, true);
        }
        else if (armType==2){
            EFFECT_FOLLOW(fighter, Hash40::new("tantan_atk_air_n3"), Hash40::new("top"), 0, 10, 4, 0, 0, 0, 1.2, true);
        }
        else{
            EFFECT_FOLLOW(fighter, Hash40::new("tantan_atk_air_n1"), Hash40::new("top"), 0, 10, 4, 0, 0, 0, 1.2, true);
        }
    }
}
#[acmd_script( agent = "tantan", script = "sound_specialairn", category = ACMD_SOUND, low_priority)]
unsafe fn tantan_special_n_air_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let armType =  WorkModule::get_int(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R);
    if(armType==1){
        frame(lua_state, 8.0);
        if is_excute(fighter) {
            PLAY_STATUS(fighter, Hash40::new("se_tantan_attackair_n02"));
            PLAY_SEQUENCE(fighter, Hash40::new("seq_tantan_rnd_attack03"));
        }
    }
    else if (armType==2){
        frame(lua_state, 7.0);
        if is_excute(fighter) {
            PLAY_STATUS(fighter, Hash40::new("se_tantan_attackair_n03"));
        }
        frame(lua_state, 8.0);
        if is_excute(fighter) {
            PLAY_SEQUENCE(fighter, Hash40::new("seq_tantan_rnd_attack01"));
        }
    }
    else{
        frame(lua_state, 6.0);
        if is_excute(fighter) {
            PLAY_STATUS(fighter, Hash40::new("se_tantan_attackair_n01"));
        }
        frame(lua_state, 8.0);
        if is_excute(fighter) {
            PLAY_SEQUENCE(fighter, Hash40::new("seq_tantan_rnd_attack01"));
        }
    }
}

#[acmd_script( agent = "tantan", script = "expression_attackairf", category = ACMD_EXPRESSION, low_priority)]
unsafe fn tantan_special_n_air_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let armType =  WorkModule::get_int(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R);
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if(armType==1){
            RUMBLE_HIT(fighter, Hash40::new("rbkind_76_megabolthit"), 10);
        }
        else if (armType==2){
            RUMBLE_HIT(fighter, Hash40::new("rbkind_slashl"), 12);
        }
        else{
            RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 9);
        }
    }
}


#[acmd_script( agent = "tantan", script = "game_specialairnend", category = ACMD_GAME, low_priority)]
unsafe fn tantan_special_n_air_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let armType =  WorkModule::get_int(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R);

    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.5);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 5.0);
    if (armType!=2) {
        FT_MOTION_RATE(fighter, 1.0);
        if (armType==0){
            frame(lua_state, 7.0);
            FT_MOTION_RATE(fighter, 0.5);
        }
        else{
            frame(lua_state, 11.0);
            FT_MOTION_RATE(fighter, 0.5);
        }
    }
    //Landing lag
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        CancelModule::enable_cancel(boma);
    }
}
#[acmd_script( agent = "tantan", script = "effect_specialairnend", category = ACMD_EFFECT, low_priority)]
unsafe fn tantan_special_n_air_end_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
}
#[acmd_script( agent = "tantan", script = "sound_specialairnend", category = ACMD_SOUND, low_priority)]
unsafe fn tantan_special_n_air_end_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 2.0);
    if is_excute(fighter) {
        PLAY_LANDING_SE(fighter, Hash40::new("se_tantan_landing02"));
    }
}
#[acmd_script( agent = "tantan", script = "expression_specialairnend", category = ACMD_EXPRESSION, low_priority)]
unsafe fn tantan_special_n_air_end_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "tantan_punch1", script = "game_specialairhiattack", category = ACMD_GAME, low_priority )]
unsafe fn tantan_special_hi_attack_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("have"), 9.0, 70, 88, 0, 56, 4.5, 3.0, 0.0, 0.3, Some(-6.0), Some(0.0), Some(0.3), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("have"), 9.0, 70, 88, 0, 56, 4.5, 3.0, 0.0, 0.3, Some(0.0), Some(0.0), Some(0.3), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}
#[acmd_script( agent = "tantan_punch1", script = "game_specialairhiattackdragon", category = ACMD_GAME, low_priority )]
unsafe fn tantan_special_hi_attack_dragon_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("have"), 10.35, 70, 92, 0, 66, 5.5, 4.0, 0.0, 0.4, Some(-6.0), Some(0.0), Some(0.4), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("have"), 10.35, 70, 92, 0, 66, 5.5, 4.0, 0.0, 0.4, Some(0.0), Some(0.0), Some(0.4), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

pub fn install() {
    install_acmd_scripts!(
        tantan_special_n_air_game,
        tantan_special_n_air_effect,
        tantan_special_n_air_sound,
        tantan_special_n_air_expression,
        
        tantan_special_n_air_end_game,
        tantan_special_n_air_end_effect,
        tantan_special_n_air_end_sound,
        tantan_special_n_air_end_expression,

        tantan_special_hi_attack_game,
        tantan_special_hi_attack_dragon_game
    );
}

