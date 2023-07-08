
use super::*;


#[acmd_script( agent = "palutena", scripts = ["game_specialn", "game_specialairn"] , category = ACMD_GAME , low_priority)]
unsafe fn palutena_special_n_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 72, 40, 0, 70, 8.0, 0.0, 12.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "palutena", scripts = ["effect_specialn", "effect_specialairn"], category = ACMD_EFFECT, low_priority )]
unsafe fn palutena_special_n_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), -0.2, 22, -1, 10, 90, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.1, 0.1, 0.15);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light_trace"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.1, 0.1, 0.15);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.1, 0.1, 0.15);
        EFFECT(agent, Hash40::new("palutena_wand_finish"), Hash40::new("top"), 0.0, 12.0, 10.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.1, 0.1, 0.15);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light_trace"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light2"), false, false);
    }
}

#[acmd_script( agent = "palutena", scripts = ["sound_specialn", "sound_specialairn"], category = ACMD_SOUND, low_priority )]
unsafe fn palutena_special_n_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_palutena_special_n01"));
    }
    wait(lua_state, 22.0);
    if is_excute(agent) {
        sound!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
    }
}

#[acmd_script( agent = "palutena", scripts = ["game_specialnr", "game_specialairnr"], category = ACMD_GAME, low_priority )]
unsafe fn palutena_special_n_r_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let power = if VarModule::get_int(boma.object(), vars::palutena::instance::POWER_BOARD_SLOT_2) == 1 {8.0} else {0.0};
    let sound_lvl = if VarModule::get_int(boma.object(), vars::palutena::instance::POWER_BOARD_SLOT_2) == 1 {*ATTACK_SOUND_LEVEL_L} else {*ATTACK_SOUND_LEVEL_M};
    let size = if VarModule::get_int(boma.object(), vars::palutena::instance::POWER_BOARD_SLOT_2) == 1 {2.0} else {0.0};
    FT_DESIRED_RATE(agent, 18.0, 12.0);
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !MeterModule::drain(boma.object(), 2) {
            MeterModule::drain(boma.object(), 1);
        }
        else {
            MeterModule::drain(boma.object(), 2);
        }
        VarModule::on_flag(boma.object(), vars::palutena::instance::FLUSH);
    }
    frame(lua_state, 18.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0 + power, 361, 100, 0, 40, 6.1 + size, 0.0, 11.0, 6.0, Some(0.0), Some(9.9), Some(11.9), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), sound_lvl, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0 + power, 361, 102, 0, 40, 6.1 + size, 0.0, 11.0, 6.0, Some(0.0), Some(8.5), Some(18.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), sound_lvl, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_MAGIC);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "palutena", scripts = ["effect_specialnr","effect_specialairnr"], category = ACMD_EFFECT, low_priority )]
unsafe fn palutena_special_n_r_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let power = VarModule::get_int(boma.object(), vars::palutena::instance::POWER_BOARD_SLOT_2) == 1;
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light_trace"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 2.0, 0.03, 0.01);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 2.0, 0.03, 0.01);
    }
    if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 1, 21, 2.5, 0, -50, 0, 1, true);
            LAST_EFFECT_SET_COLOR(agent, 2.0, 0.03, 0.01);
        }
    }
    else{
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 1, 21, 2.5, 0, -55, 0, 1, true);
            LAST_EFFECT_SET_COLOR(agent, 2.0, 0.03, 0.01);
        }
    }
    frame(lua_state, 18.0);
    if power {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_bomb_d"), Hash40::new("top"), 0.0, 12.0, 10.0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0.0, 12.0, 10.0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light_trace"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light2"), false, false);
    }
    
}

#[acmd_script( agent = "palutena", scripts = ["sound_specialnr","sound_specialairnr"], category = ACMD_SOUND, low_priority )]
unsafe fn palutena_special_n_r_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_palutena_rnd_smash_s"));
        PLAY_SE(agent, Hash40::new("se_palutena_smash_s01"));
    }
}

#[acmd_script( agent = "palutena", scripts = ["game_specialnb","game_specialairnb"] , category = ACMD_GAME , low_priority)]
unsafe fn palutena_special_n_b_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let power = if VarModule::get_int(boma.object(), vars::palutena::instance::POWER_BOARD_SLOT_2) == 2 {6.0} else {0.0};
    let powered = VarModule::get_int(boma.object(), vars::palutena::instance::POWER_BOARD_SLOT_2) == 2;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !MeterModule::drain(boma.object(), 2) {
            MeterModule::drain(boma.object(), 1);
        }
        else {
            MeterModule::drain(boma.object(), 2);
        }
        VarModule::on_flag(boma.object(), vars::palutena::instance::FLUSH);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
    }
    FT_DESIRED_RATE(agent, 14.0, 8.0);
    frame(lua_state, 18.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0 + power, 88, 91, 0, 53, 4.5, 0.0, 15.0, 9.7, Some(0.0), Some(4.0), Some(9.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 1, 0, Hash40::new("top"), 6.0 + power, 88, 91, 0, 58, 2.8, 0.0, 30.0, 9.7, Some(0.0), Some(4.0), Some(9.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        if powered {
            ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 88, 91, 0, 62, 2.8, 0.0, 45.0, 9.7, Some(0.0), Some(4.0), Some(9.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        }
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "palutena", scripts = ["effect_specialnb","effect_specialairnb"] , category = ACMD_EFFECT, low_priority )]
unsafe fn palutena_special_n_b_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let length = if VarModule::get_int(boma.object(), vars::palutena::instance::POWER_BOARD_SLOT_2) == 2 {0.69} else {0.5};
    let powered = VarModule::get_int(boma.object(), vars::palutena::instance::POWER_BOARD_SLOT_2) == 2;
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.05, 0.05, 0.90);
    }
    frame(lua_state, 16.0);
    if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("palutena_pressure"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, length, true);
            LAST_EFFECT_SET_ALPHA(agent, 0.3);
            LAST_EFFECT_SET_RATE(agent, (18.0/8.0));
            EFFECT_FOLLOW(agent, Hash40::new("sys_hit_ice"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_RATE(agent, 0.8);
            EFFECT_FOLLOW(agent, Hash40::new("sys_freezer"), Hash40::new("top"), 0, 24, 10, 0, 0, 0, 0.8, true);
            LAST_EFFECT_SET_RATE(agent, 0.8);
            if powered {
                EFFECT_FOLLOW(agent, Hash40::new("sys_freezer"), Hash40::new("top"), 0, 38, 10, 0, 0, 0, 0.6, true);
                LAST_EFFECT_SET_RATE(agent, 0.8);
            }
            EFFECT_FOLLOW_ALPHA(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 4, 21.5, 2, 0, -60, 0, 1, true, 0.7);
            LAST_EFFECT_SET_RATE(agent, 1.1);
            LAST_EFFECT_SET_COLOR(agent, 0.35, 0.35, 0.90);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 4, 21.5, 2.5, 0, -60, 0, 1, true);
            LAST_EFFECT_SET_RATE(agent, 1.1);
            LAST_EFFECT_SET_COLOR(agent, 0.35, 0.35, 0.90);
            EFFECT_FOLLOW(agent, Hash40::new("palutena_pressure"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, length, true);
            LAST_EFFECT_SET_ALPHA(agent, 0.3);
            LAST_EFFECT_SET_RATE(agent, (18.0/8.0));
            EFFECT_FOLLOW(agent, Hash40::new("sys_hit_ice"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_RATE(agent, 0.8);
            EFFECT_FOLLOW(agent, Hash40::new("sys_freezer"), Hash40::new("top"), 0, 24, 10, 0, 0, 0, 0.8, true);
            LAST_EFFECT_SET_RATE(agent, 0.8);
            if powered {
                EFFECT_FOLLOW(agent, Hash40::new("sys_freezer"), Hash40::new("top"), 0, 38, 10, 0, 0, 0, 0.6, true);
                LAST_EFFECT_SET_RATE(agent, 0.8);
            }
        }
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light2"), false, false);
    }
}

#[acmd_script( agent = "palutena", scripts = ["sound_specialnb","sound_specialairnb"], category = ACMD_SOUND, low_priority )]
unsafe fn palutena_special_n_b_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 17.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_palutena_attack07"));
        PLAY_SE(agent, Hash40::new("se_palutena_smash_h01"));
    }
}

#[acmd_script( agent = "palutena", scripts = ["game_specialny", "game_specialairny"] , category = ACMD_GAME , low_priority)]
unsafe fn palutena_special_n_y_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let hitlag = if VarModule::get_int(boma.object(), vars::palutena::instance::POWER_BOARD_SLOT_2) == 3 {0.5} else {0.75};
    let paralyze = if VarModule::get_int(boma.object(), vars::palutena::instance::POWER_BOARD_SLOT_2) == 3 {0.6} else {0.3};
    let power = if VarModule::get_int(boma.object(), vars::palutena::instance::POWER_BOARD_SLOT_2) == 3 {2} else {4};
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !MeterModule::drain(boma.object(), 2) {
            MeterModule::drain(boma.object(), 1);
        }
        else {
            MeterModule::drain(boma.object(), 2);
        }
        VarModule::on_flag(boma.object(), vars::palutena::instance::FLUSH);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 365, 40, 75, 0, 7.0, 0.0, 12.0, 10.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, power, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_MAGIC);
    }
    wait(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 65, 40, 0, 75, 9.0, 0.0, 12.0, 10.0, None, None, None, paralyze, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_MAGIC);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "palutena", scripts = ["effect_specialny", "effect_specialairny"], category = ACMD_EFFECT, low_priority )]
unsafe fn palutena_special_n_y_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), -0.2, 22, -1, 10, 90, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.85, 0.30, 0.01);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light_trace"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.85, 0.30, 0.01);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.85, 0.30, 0.01);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_paralysis"), Hash40::new("top"), 0.0, 12.0, 10.0, 0, 0, 0, 0.7, true);
        LAST_EFFECT_SET_RATE(agent, (45.0/13.0));
        LAST_EFFECT_SET_COLOR(agent, 0.85, 0.65, 0.3);
    }
    wait(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_hit_dead"), Hash40::new("top"), 0.0, 12.0, 10.0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.65, 0.85, 0.3);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light_trace"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light2"), false, false);
    }
}

#[acmd_script( agent = "palutena", scripts = ["sound_specialny", "sound_specialairny"], category = ACMD_SOUND, low_priority )]
unsafe fn palutena_special_n_y_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 11.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_palutena_special_n01"));
    }
    wait(lua_state, 40.0);
    if is_excute(agent) {
        sound!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
    }
}

#[acmd_script( agent = "palutena", scripts =[ "game_specialnp", "game_specialairnp"], category = ACMD_GAME, low_priority )]
unsafe fn palutena_special_n_p_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, false, -1);
        ArticleModule::change_motion(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("attack_lw4_charge"), false, -1.0);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        MeterModule::drain(boma.object(), 2);
        VarModule::on_flag(boma.object(), vars::palutena::instance::FLUSH);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ArticleModule::change_motion(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("attack_lw4"), false, -1.0);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 180, 90, 35, 0, 6.0, 0.0, 4.0, 14.0, Some(0.0), Some(4.0), Some(-14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 258, 70, 0, 30, 6.0, 0.0, 4.0, 14.0, Some(0.0), Some(4.0), Some(-14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_MAGIC);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        AttackModule::set_add_reaction_frame(boma, 0, 19.0, false);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

#[acmd_script( agent = "palutena", scripts =[ "effect_specialnp", "effect_specialairnp"], category = ACMD_EFFECT, low_priority )]
unsafe fn palutena_special_n_p_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light_trace"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.75, 0.01, 0.35);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.75, 0.01, 0.35);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_smash_lw_trace"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 3, 13.5, 0, 0, -90, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.75, 0.01, 0.35);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("palutena_feather"), Hash40::new("top"), 0, 3, 8, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.75, 0.01, 0.35);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("palutena_feather"), Hash40::new("top"), 0, 3, -8, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.75, 0.01, 0.35);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light_trace"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light2"), false, false);
    }
}

#[acmd_script( agent = "palutena", scripts =[ "sound_specialnp", "sound_specialairnp"], category = ACMD_SOUND, low_priority )]
unsafe fn palutena_special_n_p_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 16.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start_03"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_palutena_rnd_smash_l"));
        PLAY_SE(agent, Hash40::new("se_palutena_smash_l01"));
    }
}

#[acmd_script( agent = "palutena", scripts =[ "game_specialno", "game_specialairno"], category = ACMD_GAME, low_priority )]
unsafe fn palutena_special_n_o_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        MeterModule::drain(boma.object(), 2);
        VarModule::on_flag(boma.object(), vars::palutena::instance::FLUSH);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 15.0);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 361, 50, 0, 70, 7.0, 0.0, 10.5, 13.0, Some(0.0), Some(10.5), Some(14.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 361, 30, 0, 65, 9.0, 0.0, 10.5, 9.0, Some(0.0), Some(10.5), Some(20.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_MAGIC);
        AttackModule::set_force_reaction(boma, 0, true, false);
        AttackModule::set_force_reaction(boma, 1, true, false);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }

}
#[acmd_script( agent = "palutena", scripts =[ "effect_specialno", "effect_specialairno"], category = ACMD_EFFECT, low_priority )]
unsafe fn palutena_special_n_o_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("palutena_counter_flash"), Hash40::new("shield"), -1, 0, -3, 0, 0, 0, 1, true);
        EFFECT(agent, Hash40::new("palutena_counter_success"), Hash40::new("top"), 0, 14.8, -1, 0, 90, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("sys_counter_flash"), Hash40::new("top"), 0, 14.8, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 19.0);
    if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            EFFECT_FLW_POS(agent, Hash40::new("palutena_backlight"), Hash40::new("waist"), 10, -3, -1, 0, 90, 0, 1, true);
            LAST_EFFECT_SET_COLOR(agent, 0.8, 0.2, 0.01);
            EffectModule::set_disable_render_offset_last(boma);
            LAST_EFFECT_SET_RATE(agent, 2.75);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT_FLW_POS(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 0, 21, 5, 0, -90, 0, 1, true);
            LAST_EFFECT_SET_COLOR(agent, 0.8, 0.2, 0.01);
            EffectModule::set_disable_render_offset_last(boma);
            LAST_EFFECT_SET_RATE(agent, 2.75);
        }
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        FLASH(agent, 1, 1, 1, 0);
        EFFECT_FLW_POS(agent, Hash40::new("palutena_counter_attack"), Hash40::new("stick"), 0, 8.5, 0, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(agent, 0.8, 0.2, 0.01);
        EffectModule::set_disable_render_offset_last(boma);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("palutena_backlight"), -1);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

#[acmd_script( agent = "palutena", scripts =[ "sound_specialno", "sound_specialairno"], category = ACMD_SOUND, low_priority )]
unsafe fn palutena_special_n_o_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_palutena_special_l01"));
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_palutena_special_l02"));
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_palutena_special_l03"));
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_palutena_rnd_special_l01"));
    }
}

#[acmd_script( agent = "palutena", scripts = ["game_specialng", "game_specialairng"] , category = ACMD_GAME , low_priority)]
unsafe fn palutena_special_n_g_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        MeterModule::drain(boma.object(), 2);
        VarModule::on_flag(boma.object(), vars::palutena::instance::FLUSH);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 122, 40, 0, 75, 8.0, 0.0, 12.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 0, 40, 0, 75, 16.0, 0.0, 12.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_turn"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "palutena", scripts = ["effect_specialng", "effect_specialairng"], category = ACMD_EFFECT, low_priority )]
unsafe fn palutena_special_n_g_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), -0.2, 22, -1, 10, 90, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.05, 0.50, 0.01);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light_trace"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.05, 0.50, 0.01);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.05, 0.50, 0.01);
        EFFECT_FOLLOW(agent, Hash40::new("sys_hit_magic_s"), Hash40::new("top"), 0.0, 12.0, 10.0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.05, 0.50, 0.01);
        LAST_EFFECT_SET_ALPHA(agent, 0.3);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_hit_magic_s"), Hash40::new("top"), 0.0, 12.0, 10.0, 0, 0, 0, 1.4, true);
        LAST_EFFECT_SET_COLOR(agent, 0.05, 0.50, 0.01);
        LAST_EFFECT_SET_ALPHA(agent, 0.3);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light_trace"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light2"), false, false);
    }
}

#[acmd_script( agent = "palutena", scripts = ["sound_specialng", "sound_specialairng"], category = ACMD_SOUND, low_priority )]
unsafe fn palutena_special_n_g_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_palutena_special_n01"));
    }
    wait(lua_state, 20.0);
    if is_excute(agent) {
        sound!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
    }
}

#[acmd_script( agent = "palutena", script = "game_speciallw" , category = ACMD_GAME , low_priority)]
unsafe fn palutena_special_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 6.8, 6.8);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW))
           && !ArticleModule::is_exist(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_REFLECTIONBOARD)
           && !(VarModule::get_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER) > 0) {
            VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 1); // Start counting the cooldown timer
            VarModule::set_float(fighter.battle_object, vars::palutena::status::SPECIAL_LW_LR, PostureModule::lr(fighter.module_accessor));
            VarModule::on_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_REFLECT, true);
        }
        else{
            WorkModule::on_flag(boma, *FIGHTER_PALUTENA_STATUS_SPECIAL_LW_FLAG_SHIELD);
            shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PALUTENA_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
            SEARCH(fighter, 0, 0, Hash40::new("top"), 12.3, 0.0, 10.0, 1.5, None, None, None, *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
        }
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.9);
        WorkModule::off_flag(boma, *FIGHTER_PALUTENA_STATUS_SPECIAL_LW_FLAG_SHIELD);
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PALUTENA_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 3.0, 3.2);
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR, 0);
    }

}

#[acmd_script( agent = "palutena", script = "game_specialairlw" , category = ACMD_GAME , low_priority)]
unsafe fn palutena_special_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 6.8, 6.8);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW))
           && !ArticleModule::is_exist(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_REFLECTIONBOARD)
           && !(VarModule::get_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER) > 0) {
            VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 1); // Start counting the cooldown timer
            VarModule::set_float(fighter.battle_object, vars::palutena::status::SPECIAL_LW_LR, PostureModule::lr(fighter.module_accessor));
            VarModule::on_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_REFLECT, true);
        }
        else{
            WorkModule::on_flag(boma, *FIGHTER_PALUTENA_STATUS_SPECIAL_LW_FLAG_SHIELD);
            shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PALUTENA_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
            SEARCH(fighter, 0, 0, Hash40::new("top"), 12.3, 0.0, 10.0, 1.5, None, None, None, *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
        }
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.9);
        WorkModule::off_flag(boma, *FIGHTER_PALUTENA_STATUS_SPECIAL_LW_FLAG_SHIELD);
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PALUTENA_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 3.0, 3.2);
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR, 0);
    }
    
}

#[acmd_script( agent = "palutena", script = "game_speciallwreflect" , category = ACMD_GAME , low_priority)]
unsafe fn palutena_special_lw_reflect_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR) {
            FT_MOTION_RATE(fighter, 1.5);
            PostureModule::set_lr(boma, VarModule::get_float(fighter.battle_object, vars::palutena::status::SPECIAL_LW_LR));
            PostureModule::update_rot_y_lr(boma);
        }
        else {
            shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PALUTENA_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR) {
            shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PALUTENA_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR) {
            ArticleModule::generate_article(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_REFLECTIONBOARD, false, 0);
            FT_MOTION_RATE(fighter, 0.75);
        }
        else{
            FT_MOTION_RATE(fighter, 0.6);
        }
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }

}

#[acmd_script( agent = "palutena", script = "effect_speciallwreflect" , category = ACMD_EFFECT , low_priority)]
unsafe fn palutena_special_lw_reflect_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR) {
            LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
            EFFECT_FOLLOW(fighter, Hash40::new("palutena_wand_light_trace"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FOLLOW(fighter, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
            EFFECT(fighter, Hash40::new("palutena_throw_twinkle"), Hash40::new("top"), 0.0, 16.0, -8.0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR) {
            EFFECT(fighter, Hash40::new("palutena_mirror_break"), Hash40::new("top"), 0.0, 16.0, -8.0, 0, 0, 0, 0.225, 0, 0, 0, 0, 0, 0, true);
        }
        else{
            EFFECT(fighter, Hash40::new("palutena_mirror"), Hash40::new("top"), 12.0, 12.0, 0, 0, 22.5, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
            EFFECT_DETACH_KIND(fighter, Hash40::new("palutena_backlight"), -1);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR) {
            EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("palutena_backlight"), Hash40::new("top"), -1, 21, 1, 0, 90, 0, 1, true, 0.7);
        }
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR) {
            EFFECT_OFF_KIND(fighter, Hash40::new("palutena_wand_light_trace"), false, false);
            EFFECT_OFF_KIND(fighter, Hash40::new("palutena_wand_light2"), false, false);
        }
    }

}

#[acmd_script( agent = "palutena", script = "game_specialairlwreflect" , category = ACMD_GAME , low_priority)]
unsafe fn palutena_special_air_lw_reflect_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR) {
            FT_MOTION_RATE(fighter, 1.5);
            PostureModule::set_lr(boma, VarModule::get_float(fighter.battle_object, vars::palutena::status::SPECIAL_LW_LR));
            PostureModule::update_rot_y_lr(boma);
        }
        else {
            shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PALUTENA_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR) {
            shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PALUTENA_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR) {
            ArticleModule::generate_article(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_REFLECTIONBOARD, false, 0);
            FT_MOTION_RATE(fighter, 0.75);
        }
        else{
            FT_MOTION_RATE(fighter, 0.6);
        }
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }

}

#[acmd_script( agent = "palutena", script = "effect_specialairlwreflect" , category = ACMD_EFFECT , low_priority)]
unsafe fn palutena_special_air_lw_reflect_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR) {
            EFFECT_FOLLOW(fighter, Hash40::new("palutena_wand_light_trace"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FOLLOW(fighter, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
            EFFECT(fighter, Hash40::new("palutena_throw_twinkle"), Hash40::new("top"), 0.0, 16.0, -8.0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR) {
            EFFECT(fighter, Hash40::new("palutena_mirror_break"), Hash40::new("top"), 0.0, 16.0, -8.0, 0, 0, 0, 0.225, 0, 0, 0, 0, 0, 0, true);
        }
        else{
            EFFECT(fighter, Hash40::new("palutena_mirror"), Hash40::new("top"), 12.0, 12.0, 0, 0, 22.5, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
            EFFECT_DETACH_KIND(fighter, Hash40::new("palutena_backlight"), -1);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR) {
            EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("palutena_backlight"), Hash40::new("top"), -1, 21, 1, 0, 90, 0, 1, true, 0.7);
        }
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::palutena::status::SPECIAL_LW_AEGIS_REFLECTOR) {
            EFFECT_OFF_KIND(fighter, Hash40::new("palutena_wand_light_trace"), false, false);
            EFFECT_OFF_KIND(fighter, Hash40::new("palutena_wand_light2"), false, false);
        }
    }

}


pub fn install() {
    install_acmd_scripts!(
        palutena_special_lw_game,
        palutena_special_air_lw_game,
        palutena_special_lw_reflect_game,
        palutena_special_lw_reflect_effect,
        palutena_special_air_lw_reflect_game,
        palutena_special_air_lw_reflect_effect,
        palutena_special_n_game,
        palutena_special_n_effect,
        palutena_special_n_sound,
        palutena_special_n_r_game,
        palutena_special_n_r_effect,
        palutena_special_n_r_sound,
        palutena_special_n_b_game,
        palutena_special_n_b_effect,
        palutena_special_n_b_sound,
        palutena_special_n_y_game,
        palutena_special_n_y_effect,
        palutena_special_n_y_sound,
        palutena_special_n_p_game,
        palutena_special_n_p_effect,
        palutena_special_n_p_sound,
        palutena_special_n_o_game,
        palutena_special_n_o_effect,
        palutena_special_n_o_sound,
        palutena_special_n_g_game,
        palutena_special_n_g_effect,
        palutena_special_n_g_sound,
    );
}

