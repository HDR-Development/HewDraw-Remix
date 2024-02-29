
use super::*;

unsafe extern "C" fn jack_attack_11_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 30, 0, 32, 3.0, 0.0, 5.5, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 361, 30, 0, 32, 3.0, 0.0, 5.5, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 361, 30, 0, 32, 4.0, 0.0, 7.5, 12.5, Some(0.0), Some(7.5), Some(15.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        if WorkModule::is_flag(boma,  *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
            ATTACK(fighter, 4, 1, Hash40::new("top"), 1.0, 361, 30, 0, 33, 3.0, 0.0, 7.5, 7.5, Some(0.0), Some(7.5), Some(10.8), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 5, 1, Hash40::new("top"), 1.0, 361, 30, 0, 33, 3.0, 0.0, 7.5, 10.8, Some(0.0), Some(7.5), Some(14.7), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    
}

unsafe extern "C" fn jack_attack_12_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 75, 30, 0, 25, 3.0, 0.0, 6.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 75, 30, 0, 25, 3.0, 0.0, 6.0, 13.0, Some(0.0), Some(8.0), Some(14.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 75, 30, 0, 25, 3.0, 0.0, 8.0, 18.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        if WorkModule::is_flag(boma,  *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
            ATTACK(fighter, 4, 1, Hash40::new("top"), 1.0, 75, 30, 0, 25, 2.5, 0.0, 8.0, 10.5, Some(0.0), Some(8.0), Some(15.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 5, 1, Hash40::new("top"), 1.0, 75, 30, 0, 25, 2.5, 0.0, 8.0, 15.0, Some(0.0), Some(8.0), Some(18.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    
}

unsafe extern "C" fn jack_attack_13_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 80, 0, 60, 3.0, 0.0, 8.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 361, 80, 0, 60, 3.0, 0.0, 8.0, 13.5, Some(0.0), Some(8.0), Some(14.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("knife"), 5.0, 361, 80, 0, 60, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        if WorkModule::is_flag(boma,  *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
            ATTACK(fighter, 3, 1, Hash40::new("top"), 1.6, 361, 135, 0, 65, 3.5, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(14.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        }
    }
    frame(lua_state, 6.0);
    if WorkModule::is_flag(boma,  *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        FT_MOTION_RATE(fighter, 27.0/(28.0-6.0));
    }
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

unsafe extern "C" fn jack_attack_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.8);
    frame(lua_state, 1.0);
    if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE){
        FT_MOTION_RATE(fighter, 7.0/(7.0-1.0));
    }
    else{
        FT_MOTION_RATE(fighter, 5.0/(7.0-1.0));
    }
    frame(lua_state, 7.0);
    JostleModule::set_status(boma, false);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE){
            ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 85, 50, 0, 80, 3.5, 0.0, 3.2, 0.0, Some(0.0), Some(3.2), Some(8.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("legr"), 7.0, 75, 66, 0, 75, 3.5, 0.0, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 1, 0, Hash40::new("kneer"), 7.0, 75, 66, 0, 75, 3.5, 0.0, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 2, 0, Hash40::new("kneer"), 7.0, 75, 66, 0, 75, 3.5, 4.0, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE){
            ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 85, 50, 0, 80, 3.5, 0.0, 3.2, 0.0, Some(0.0), Some(3.2), Some(8.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("legr"), 6.0, 82, 66, 0, 75, 3.0, 0.0, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 1, 0, Hash40::new("kneer"), 6.0, 82, 66, 0, 75, 3.0, 0.0, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 2, 0, Hash40::new("kneer"), 6.0, 82, 66, 0, 75, 3.0, 5.0, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    }
    frame(lua_state, 17.0);
    if WorkModule::is_flag(boma,  *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        FT_MOTION_RATE(fighter, 23.0/(38.0-17.0));
    } 
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        JostleModule::set_status(boma, true);
    }

}

unsafe extern "C" fn jack_attack_dash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        if !WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE){
            LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -1.5, 0, 0, 0, 0.8, 0, 0, 1.5, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(fighter, 1);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE){
            EFFECT_FOLLOW(fighter, Hash40::new("jack_doyle_attack_line"), Hash40::new("top"), 0, 4, -2, 0, 0, 0, 0.8, true);
            LAST_EFFECT_SET_RATE(fighter, 0.8);
            if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
                EFFECT_FOLLOW(fighter, Hash40::new("jack_attack_line2"), Hash40::new("top"), 7, 2, -2, 0, 0, 0, 0.8, true);
                LAST_EFFECT_SET_RATE(fighter, 0.8);
            }
            else{
                EFFECT_FOLLOW(fighter, Hash40::new("jack_attack_line2"), Hash40::new("top"), -7, 2, -2, 0, 0, 0, 0.8, true);
                LAST_EFFECT_SET_RATE(fighter, 0.8);
            }
        }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if !WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE){
            LANDING_EFFECT(fighter, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 6, 0, -2.5, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(fighter, 1.2);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if !WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE){
            LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 8, 0, -2.5, 0, 0, 0, 0.45, 0, 0, 1.5, 0, 0, 0, false);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if !WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE){
            LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 7, 0, -2.5, 0, 0, 0, 0.35, 0, 0, 1.5, 0, 0, 0, false);
        }
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        if !WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE){
            LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 5, 0, -2.5, 0, 0, 0, 0.35, 0, 0, 1.5, 0, 0, 0, false);
        }
    }

}

unsafe extern "C" fn jack_attack_dash_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 5);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2, true);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
    }
}

pub fn install() {
    smashline::Agent::new("jack")
        .acmd("game_attack11", jack_attack_11_game)
        .acmd("game_attack12", jack_attack_12_game)
        .acmd("game_attack13", jack_attack_13_game)
        .acmd("game_attackdash", jack_attack_dash_game)
        .acmd("effect_attackdash", jack_attack_dash_effect)
        .acmd("expression_attackdash", jack_attack_dash_expression)
        .install();
}
