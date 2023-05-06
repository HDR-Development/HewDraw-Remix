
use super::*;

#[acmd_script( agent = "miiswordsman", script = "game_specialn1" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_n1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
    }

    frame(lua_state, 17.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_TORNADOSHOT, false, 0);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.85);
    }  
}

#[acmd_script( agent = "miiswordsman", script = "game_specialairn1" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_air_n1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
    }

    frame(lua_state, 17.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_TORNADOSHOT, false, 0);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.8);
    }  
}

//Will need to be looked at in the future when rebalancing tornado
/*
#[acmd_script( agent = "miiswordsman", script = "game_specialn1" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_n1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let end_frame = MotionModule::end_frame(boma);
    if is_excute(fighter) {
        VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 0); // Timer used to track time until Gale Strike is available again
        //VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        ArticleModule::remove(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_TORNADOSHOT, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
     }
     frame(lua_state, 10.0);
     if is_excute(fighter) {
         if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW){
             //VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
             VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
             FT_MOTION_RATE(fighter, 3.0);
         }
         else{
            FT_MOTION_RATE(fighter, 1.25);
         }
     }
     frame(lua_state, 17.0);
     if is_excute(fighter) {
         ArticleModule::generate_article(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_TORNADOSHOT, false, 0);
         //if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 120);
         }
         else{
            VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 52);
         }
     }
     frame(lua_state, 18.0);
     if is_excute(fighter) {
         //if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
             FT_MOTION_RATE(fighter, 1.2);
         }
         else{
             FT_MOTION_RATE(fighter, 0.85);
         }  
     }
}

#[acmd_script( agent = "miiswordsman", script = "effect_specialn1" , category = ACMD_EFFECT , low_priority)]
unsafe fn miiswordsman_special_n1_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let end_frame = MotionModule::end_frame(boma);   
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 7.5, 0.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.84, 0.17);
        }
        else{
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 7.5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.84, 0.17);
            EFFECT(fighter, Hash40::new("miiswordsman_counter_flash"), Hash40::new("top"), 0, 9, 7.5, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.84, 0.17);
        }
        else{
            LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

#[acmd_script( agent = "miiswordsman", script = "game_specialairn1" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_air_n1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let end_frame = MotionModule::end_frame(boma);
    if is_excute(fighter) {
        VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 0); // Timer used to track time until Gale Strike is available again
        //VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        ArticleModule::remove(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_TORNADOSHOT, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
     }
     frame(lua_state, 10.0);
     if is_excute(fighter) {
         if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW){
             //VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
             VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
             FT_MOTION_RATE(fighter, 3.0);
         }
     }
     frame(lua_state, 17.0);
     if is_excute(fighter) {
         ArticleModule::generate_article(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_TORNADOSHOT, false, 0);
         //if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){             
            VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 120);
         }
         else{
            VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 52);
         }
     }
     frame(lua_state, 18.0);
     if is_excute(fighter) {
         //if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
             FT_MOTION_RATE(fighter, 1.2);
         }
         else{
             FT_MOTION_RATE(fighter, 0.8);
         }  
     }
    
}

#[acmd_script( agent = "miiswordsman", script = "effect_specialairn1" , category = ACMD_EFFECT , low_priority)]
unsafe fn miiswordsman_special_air_n1_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let end_frame = MotionModule::end_frame(boma);   
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 7.5, 0.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.84, 0.17);
        }
        else{
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 7.5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.84, 0.17);
            EFFECT(fighter, Hash40::new("miiswordsman_counter_flash"), Hash40::new("top"), 0, 9, 7.5, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.84, 0.17);
        }
        else{
            LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
        }
    }
}
*/

#[acmd_script( agent = "miiswordsman", script = "game_specialn2" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_n2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    if is_excute(fighter) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
        VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
            VarModule::on_flag(fighter.battle_object, vars::miiswordsman::status::WAVE_SPECIAL_N);
            VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
            MotionModule::set_rate(boma, 0.3);
        }
        else {
            MotionModule::set_rate(boma, 1.0/(1.0/(17.0-14.0)));
        }
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 1.0);
        // light
        if !VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            MotionModule::set_rate(boma, 1.0);
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 10.0, 361, 100, 0, 39, 3.5, 0.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 11.0, 361, 100, 0, 39, 3.5, 0.0, 6.75, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("haver"), 12.0, 361, 100, 0, 39, 3.5, 0.0, 11.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        // heavy
        else {
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 12.0, 361, 100, 0, 39, 3.5, 0.0, 2.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 13.0, 361, 100, 0, 39, 3.5, 0.0, 6.75, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("haver"), 14.0, 361, 100, 0, 39, 3.5, 0.0, 11.5, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            WorkModule::set_float(boma, 0.0, *FIGHTER_MIISWORDSMAN_STATUS_FINAL_WORK_ID_FLOAT_WAVE_ANGLE);
            ArticleModule::generate_article(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_WAVE, false, 0);
        }
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            MotionModule::set_rate(boma, 0.333);
        }
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 1.0);
    }
}

#[acmd_script( agent = "miiswordsman", script = "effect_specialn2" , category = ACMD_EFFECT , low_priority)]
unsafe fn miiswordsman_special_n2_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 12.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_miiswordsman_sword03"), Hash40::new("tex_miiswordsman_sword04"), 8, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 10.5, 0.0, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_Y, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 7.5, 0.0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
            //LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.84, 0.17);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        //EFFECT_FOLLOW_WORK(fighter, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(fighter, Hash40::new("miiswordsman_counter_arc"), Hash40::new("top"), -1, 10, 6, -10.6, -159.5, 90.0, 1.3, true);
	    LAST_EFFECT_SET_RATE(fighter, 1.4);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4.0);
    }
}

#[acmd_script( agent = "miiswordsman", script = "sound_specialn2" , category = ACMD_SOUND , low_priority)]
unsafe fn miiswordsman_special_n2_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 16.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_miiswordsman_rnd_special_c2_l01"));
        PLAY_SE(fighter, Hash40::new("se_miiswordsman_special_l03"));
    }
}

#[acmd_script( agent = "miiswordsman", script = "game_specialairn2" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_air_n2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    if is_excute(fighter) {
        let x_vel = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, x_vel * 0.6, 0.0, 0.0, 0.0, 0.0);
        app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable * 0.5, 100.0);
        app::sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
        VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
            VarModule::on_flag(fighter.battle_object, vars::miiswordsman::status::WAVE_SPECIAL_N);
            VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
            MotionModule::set_rate(boma, 0.3);
        }
        else {
            MotionModule::set_rate(boma, 1.0/(1.0/(17.0-14.0)));
        }
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 1.0);
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.8);
            app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        }
        // light
        if !VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            MotionModule::set_rate(boma, 1.0);
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 10.0, 361, 100, 0, 39, 3.5, 0.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 11.0, 361, 100, 0, 39, 3.5, 0.0, 6.75, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("haver"), 12.0, 361, 100, 0, 39, 3.5, 0.0, 11.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        // heavy
        else {
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 12.0, 361, 100, 0, 39, 3.5, 0.0, 2.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 13.0, 361, 100, 0, 39, 3.5, 0.0, 6.75, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("haver"), 14.0, 361, 100, 0, 39, 3.5, 0.0, 11.5, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            WorkModule::set_float(boma, 0.0, *FIGHTER_MIISWORDSMAN_STATUS_FINAL_WORK_ID_FLOAT_WAVE_ANGLE);
            ArticleModule::generate_article(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_WAVE, false, 0);
        }
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            MotionModule::set_rate(boma, 0.333);
        }
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 1.0);
    }
}

#[acmd_script( agent = "miiswordsman", script = "effect_specialairn2" , category = ACMD_EFFECT , low_priority)]
unsafe fn miiswordsman_special_air_n2_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 12.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_miiswordsman_sword03"), Hash40::new("tex_miiswordsman_sword04"), 8, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 10.5, 0.0, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_Y, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 7.5, 0.0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
            //LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.84, 0.17);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        //EFFECT_FOLLOW_WORK(fighter, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("miiswordsman_counter_arc"), Hash40::new("top"), -1, 10, 6, -10.6, -159.5,  90.0, 1.3, true);
	    LAST_EFFECT_SET_RATE(fighter, 1.4);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4.0);
    }
}

#[acmd_script( agent = "miiswordsman", script = "sound_specialairn2" , category = ACMD_SOUND , low_priority)]
unsafe fn miiswordsman_special_air_n2_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 16.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_miiswordsman_rnd_special_c2_l01"));
        PLAY_SE(fighter, Hash40::new("se_miiswordsman_special_l03"));
    }
}

// ================================================================================================
// ======================================== BLURRING BLADE ========================================
// ================================================================================================

#[acmd_script( agent = "miiswordsman", script = "game_specialn3end" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_n3_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let end_frame = MotionModule::end_frame(boma);
    frame(lua_state, 6.0);
    for _ in 0..4 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 0.8, 180, 100, 1, 0, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 0.8, 92, 100, 1, 0, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 2.0);
    }
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 0.8, 91, 100, 21, 0, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 1, 10.0, false);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 4.0);
    }
    frame(lua_state, 44.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 1.0);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        //ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 40, 110, 0, 58, 13.0, 0.0, 10.0, 9.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 8.0, 40, 110, 0, 58, 4.0, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 40, 110, 0, 58, 8.5, 0.0, 8.0, 12.5, Some(0.0), Some(8.5), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "miiswordsman", script = "effect_specialn3end" , category = ACMD_EFFECT , low_priority)]
unsafe fn miiswordsman_special_n3_end_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1.0, true);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_WORK_arg29(fighter, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD, 3, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 0.2, 0.0, true, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_Y, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 2.0, 0.2);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("miiswordsman_rapid_slash_wind_s"), Hash40::new("top"), -0.0, 5.5, 12, 0, 0, 0, 1, true);
	    EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_wind_s"), false, false);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("miiswordsman_rapid_slash_arc"), Hash40::new("top"), 3, 8, 3, 70, -80, 190, 1.2, true);
	    LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(lua_state, 47.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
	    //EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("haver"), -0.0, 12, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        EFFECT(fighter, Hash40::new("sys_bomb_b"), Hash40::new("top"), 0, 0, 13, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 55.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 2);
    }
    frame(lua_state, 74.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
}

#[acmd_script( agent = "miiswordsman", script = "sound_specialn3end" , category = ACMD_SOUND , low_priority)]
unsafe fn miiswordsman_special_n3_end_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let end_frame = MotionModule::end_frame(boma);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_miiswordsman_special_s01"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_miiswordsman_rnd_special_c3_n01"));
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_miiswordsman_special_c3_n01"));
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_miiswordsman_special_c3_n02"));
    }
    frame(lua_state, 47.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_bomb_l"));
        PLAY_SE(fighter, Hash40::new("se_miiswordsman_special_h03_win02"));
    }
}

#[acmd_script( agent = "miiswordsman", script = "game_specialn3endturn" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_n3_end_turn_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let end_frame = MotionModule::end_frame(boma);
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        REVERSE_LR(fighter);
    }
    for _ in 0..4 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 0.8, 180, 100, 1, 0, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 0.8, 92, 100, 1, 0, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 2.0);
    }
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 0.8, 91, 100, 21, 0, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 4.0);
    }
    frame(lua_state, 44.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 1.0);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        //ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 40, 110, 0, 58, 13.0, 0.0, 10.0, 9.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 8.0, 40, 110, 0, 58, 4.0, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 40, 110, 0, 58, 8.5, 0.0, 8.0, -12.5, Some(0.0), Some(8.5), Some(-13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "miiswordsman", script = "effect_specialn3endturn" , category = ACMD_EFFECT , low_priority)]
unsafe fn miiswordsman_special_n3_end_turn_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1.0, true);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new_raw(WorkModule::get_int64(boma, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD)), Hash40::new_raw(WorkModule::get_int64(boma, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD)), 3, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 0.2, 0.0, true, Hash40::new_raw(WorkModule::get_int64(boma, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE)), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_Y, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 2.0, 0.2);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("miiswordsman_rapid_slash_wind_s"), Hash40::new("top"), -0.0, 5.5, -12, 0, 180, 0, 1, true);
	    EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_wind_s"), false, false);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("miiswordsman_rapid_slash_arc"), Hash40::new("top"), -3, 8, -3, 250, 80, 10, 1.2, true);
	    LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(lua_state, 47.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
	    //EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("haver"), -0.0, 12, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        EFFECT(fighter, Hash40::new("sys_bomb_b"), Hash40::new("top"), 0, 0, -13, 0, 180, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 55.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 2);
    }
    frame(lua_state, 74.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
}

#[acmd_script( agent = "miiswordsman", script = "sound_specialn3endturn" , category = ACMD_SOUND , low_priority)]
unsafe fn miiswordsman_special_n3_end_turn_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let end_frame = MotionModule::end_frame(boma);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_miiswordsman_special_s01"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_miiswordsman_rnd_special_c3_n01"));
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_miiswordsman_special_c3_n01"));
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_miiswordsman_special_c3_n02"));
    }
    frame(lua_state, 47.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_bomb_l"));
        PLAY_SE(fighter, Hash40::new("se_miiswordsman_special_h03_win02"));
    }
}

#[acmd_script( agent = "miiswordsman", script = "game_specialn3endmax" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_n3_end_max_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let end_frame = MotionModule::end_frame(boma);
    frame(lua_state, 6.0);
    for _ in 0..4 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 0.8, 180, 100, 1, 0, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 0.8, 92, 100, 1, 0, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 2.0);
    }
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 0.8, 91, 100, 21, 0, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 1, 10.0, false);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 4.0);
    }
    frame(lua_state, 44.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 1.0);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 8.0, 40, 110, 0, 58, 4.0, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 40, 110, 0, 58, 13.0, 0.0, 9.0, 15.0, Some(0.0), Some(8.5), Some(15.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "miiswordsman", script = "effect_specialn3endmax" , category = ACMD_EFFECT , low_priority)]
unsafe fn miiswordsman_special_n3_end_max_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1.0, true);
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_WORK_arg29(fighter, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD, 3, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 0.2, 0.0, true, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_Y, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 2.0, 0.2);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("miiswordsman_rapid_slash_wind_s"), Hash40::new("top"), -0.0, 5.5, 12, 0, 0, 0, 1, true);
	    EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_wind_s"), false, false);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 4.0);
    }
    frame(lua_state, 44.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 1.0);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("miiswordsman_rapid_slash_arc"), Hash40::new("top"), 3, 8, 3, 70, -80, 190, 1.2, true);
	    LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(lua_state, 47.0);
    if is_excute(fighter) {
        //LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
	    //EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("haver"), -0.0, 12, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        EFFECT(fighter, Hash40::new("sys_bomb_c"), Hash40::new("top"), 0, 0, 13, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 55.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 2);
    }
    frame(lua_state, 74.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
}

#[acmd_script( agent = "miiswordsman", script = "sound_specialn3endmax" , category = ACMD_SOUND , low_priority)]
unsafe fn miiswordsman_special_n3_end_max_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let end_frame = MotionModule::end_frame(boma);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_miiswordsman_special_s01"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_miiswordsman_rnd_special_c3_n01"));
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_miiswordsman_special_c3_n01"));
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_miiswordsman_special_c3_n02"));
    }
    frame(lua_state, 47.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_bomb_ll"));
        PLAY_SE(fighter, Hash40::new("se_miiswordsman_special_h03_win02"));
    }
}

#[acmd_script( agent = "miiswordsman", script = "game_specialn3endmaxturn" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_n3_end_max_turn_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let end_frame = MotionModule::end_frame(boma);
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        REVERSE_LR(fighter);
    }
    for _ in 0..4 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 0.8, 180, 100, 1, 0, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 0.8, 92, 100, 1, 0, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 2.0);
    }
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 0.8, 91, 100, 21, 0, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 1, 10.0, false);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 4.0);
    }
    frame(lua_state, 44.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 1.0);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 8.0, 40, 110, 0, 58, 4.0, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 40, 110, 0, 58, 13.0, 0.0, 9.0, -15.0, Some(0.0), Some(8.5), Some(-15.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "miiswordsman", script = "effect_specialn3endmaxturn" , category = ACMD_EFFECT , low_priority)]
unsafe fn miiswordsman_special_n3_end_max_turn_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1.0, true);
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_WORK_arg29(fighter, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD, 3, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 0.2, 0.0, true, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_Y, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 2.0, 0.2);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("miiswordsman_rapid_slash_wind_s"), Hash40::new("top"), -0.0, 5.5, -12, 0, 180, 0, 1, true);
	    EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_wind_s"), false, false);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 4.0);
    }
    frame(lua_state, 44.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 1.0);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("miiswordsman_rapid_slash_arc"), Hash40::new("top"), -3, 8, -3, 250, 80, 10, 1.2, true);
	    LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(lua_state, 47.0);
    if is_excute(fighter) {
        //LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
	    //EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("haver"), -0.0, 12, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        EFFECT(fighter, Hash40::new("sys_bomb_c"), Hash40::new("top"), 0, 0, -13, 0, 180, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 55.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 2);
    }
    frame(lua_state, 74.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
}

#[acmd_script( agent = "miiswordsman", script = "sound_specialn3endmaxturn" , category = ACMD_SOUND , low_priority)]
unsafe fn miiswordsman_special_n3_end_max_turn_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let end_frame = MotionModule::end_frame(boma);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_miiswordsman_special_s01"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_miiswordsman_rnd_special_c3_n01"));
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_miiswordsman_special_c3_n01"));
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_miiswordsman_special_c3_n02"));
    }
    frame(lua_state, 47.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_bomb_ll"));
        PLAY_SE(fighter, Hash40::new("se_miiswordsman_special_h03_win02"));
    }
}

#[acmd_script( agent = "miiswordsman", script = "game_specialairn3end" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_air_n3_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let end_frame = MotionModule::end_frame(boma);
    if is_excute(fighter) {
        SET_SPEED_EX(fighter, 0.5, 1.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(lua_state, 6.0);
    for _ in 0..2 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 0.8, 90, 100, 1, 0, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 0.8, 285, 100, 10, 0, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 2.0);
    }
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 0.8, 120, 100, 20, 0, 4.0, 0.0, 8.2, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 0.8, 45, 100, 25, 0, 4.0, 0.0, -1.2, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 10.0, false);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        let addSpeed = smash::phx::Vector3f { x: 0.0, y: 2.0, z: 0.0 };
        KineticModule::add_speed(boma, &addSpeed);
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 4.0);
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 1.0);
    }
    frame(lua_state, 43.0);
    if is_excute(fighter) {
        //ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 40, 110, 0, 58, 13.0, 0.0, 10.0, 9.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 8.0, 40, 110, 0, 58, 4.0, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        let addSpeed = smash::phx::Vector3f { x: 0.0, y: 2.0, z: 0.0 };
        KineticModule::add_speed(boma, &addSpeed);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 40, 110, 0, 58, 8.5, 0.0, 8.0, 12.5, Some(0.0), Some(8.5), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "miiswordsman", script = "effect_specialairn3end" , category = ACMD_EFFECT , low_priority)]
unsafe fn miiswordsman_special_air_n3_end_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1.0, true);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_WORK_arg29(fighter, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD, 3, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 0.2, 0.0, true, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_Y, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 2.0, 0.2);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("miiswordsman_rapid_slash_wind_s"), Hash40::new("top"), -0.0, 5.5, 12, 0, 0, 0, 1, true);
	    EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_wind_s"), false, false);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 43.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("miiswordsman_rapid_slash_arc"), Hash40::new("top"), 3, 8, 3, 70, -80, 190, 1.2, true);
	    LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
	    //EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("haver"), -0.0, 12, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        EFFECT(fighter, Hash40::new("sys_bomb_b"), Hash40::new("top"), 0, 0, 13, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 55.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 2);
    }
    frame(lua_state, 74.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
}

#[acmd_script( agent = "miiswordsman", script = "sound_specialairn3end" , category = ACMD_SOUND , low_priority)]
unsafe fn miiswordsman_special_air_n3_end_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let end_frame = MotionModule::end_frame(boma);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_miiswordsman_special_s01"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_miiswordsman_rnd_special_c3_n01"));
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_miiswordsman_special_c3_n01"));
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_miiswordsman_special_c3_n02"));
    }
    frame(lua_state, 47.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_bomb_l"));
    }
}

#[acmd_script( agent = "miiswordsman", script = "game_specialairn3endturn" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_air_n3_end_turn_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let end_frame = MotionModule::end_frame(boma);
    if is_excute(fighter) {
        SET_SPEED_EX(fighter, -0.5, 1.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        REVERSE_LR(fighter);
    }
    for _ in 0..2 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 0.8, 90, 100, 1, 0, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 0.8, 285, 100, 10, 0, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 2.0);
    }
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 0.8, 120, 100, 20, 0, 4.0, 0.0, 8.2, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 0.8, 45, 100, 25, 0, 4.0, 0.0, -1.2, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 10.0, false);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        let addSpeed = smash::phx::Vector3f { x: 0.0, y: 2.0, z: 0.0 };
        KineticModule::add_speed(boma, &addSpeed);
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 4.0);
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 1.0);
    }
    frame(lua_state, 43.0);
    if is_excute(fighter) {
        //ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 40, 110, 0, 58, 13.0, 0.0, 10.0, 9.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 8.0, 40, 110, 0, 58, 4.0, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        let addSpeed = smash::phx::Vector3f { x: 0.0, y: 2.0, z: 0.0 };
        KineticModule::add_speed(boma, &addSpeed);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 40, 110, 0, 58, 8.5, 0.0, 8.0, -12.5, Some(0.0), Some(8.5), Some(-13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "miiswordsman", script = "effect_specialairn3endturn" , category = ACMD_EFFECT , low_priority)]
unsafe fn miiswordsman_special_air_n3_end_turn_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1.0, true);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_WORK_arg29(fighter, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD, 3, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 0.2, 0.0, true, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_Y, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 2.0, 0.2);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("miiswordsman_rapid_slash_wind_s"), Hash40::new("top"), -0.0, 5.5, -12, 0, 180, 0, 1, true);
	    EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_wind_s"), false, false);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 43.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("miiswordsman_rapid_slash_arc"), Hash40::new("top"), -3, 8, -3, 250, 80, 10, 1.2, true);
	    LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
	    //EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("haver"), -0.0, 12, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        EFFECT(fighter, Hash40::new("sys_bomb_b"), Hash40::new("top"), 0, 0, -13, 0, 180, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 55.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 2);
    }
    frame(lua_state, 74.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
}

#[acmd_script( agent = "miiswordsman", script = "sound_specialairn3endturn" , category = ACMD_SOUND , low_priority)]
unsafe fn miiswordsman_special_air_n3_end_turn_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let end_frame = MotionModule::end_frame(boma);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_miiswordsman_special_s01"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_miiswordsman_rnd_special_c3_n01"));
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_miiswordsman_special_c3_n01"));
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_miiswordsman_special_c3_n02"));
    }
    frame(lua_state, 47.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_bomb_l"));
    }
}

#[acmd_script( agent = "miiswordsman", script = "game_specialairn3endmax" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_air_n3_end_max_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let end_frame = MotionModule::end_frame(boma);
    if is_excute(fighter) {
        SET_SPEED_EX(fighter, 0.5, 1.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(lua_state, 6.0);
    for _ in 0..2 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 0.8, 90, 100, 1, 0, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 0.8, 285, 100, 10, 0, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 2.0);
    }
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 0.8, 120, 100, 20, 0, 4.0, 0.0, 8.2, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 0.8, 45, 100, 25, 0, 4.0, 0.0, -1.2, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 10.0, false);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        let addSpeed = smash::phx::Vector3f { x: 0.0, y: 2.0, z: 0.0 };
        KineticModule::add_speed(boma, &addSpeed);
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 4.0);
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 1.0);
    }
    frame(lua_state, 43.0);
    if is_excute(fighter) {
        //ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 40, 110, 0, 58, 13.0, 0.0, 10.0, 9.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 8.0, 40, 110, 0, 58, 4.0, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        let addSpeed = smash::phx::Vector3f { x: 0.0, y: 2.0, z: 0.0 };
        KineticModule::add_speed(boma, &addSpeed);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 40, 110, 0, 58, 13.0, 0.0, 9.0, 15.0, Some(0.0), Some(8.5), Some(15.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "miiswordsman", script = "effect_specialairn3endmax" , category = ACMD_EFFECT , low_priority)]
unsafe fn miiswordsman_special_air_n3_end_max_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1.0, true);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_WORK_arg29(fighter, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD, 3, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 0.2, 0.0, true, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_Y, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 2.0, 0.2);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("miiswordsman_rapid_slash_wind_s"), Hash40::new("top"), -0.0, 5.5, 12, 0, 0, 0, 1, true);
	    EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_wind_s"), false, false);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 43.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("miiswordsman_rapid_slash_arc"), Hash40::new("top"), 3, 8, 3, 70, -80, 190, 1.2, true);
	    LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
	    //EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("haver"), -0.0, 12, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        EFFECT(fighter, Hash40::new("sys_bomb_c"), Hash40::new("top"), 0, 0, 13, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 55.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 2);
    }
    frame(lua_state, 74.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
}

#[acmd_script( agent = "miiswordsman", script = "sound_specialairn3endmax" , category = ACMD_SOUND , low_priority)]
unsafe fn miiswordsman_special_air_n3_end_max_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let end_frame = MotionModule::end_frame(boma);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_miiswordsman_special_s01"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_miiswordsman_rnd_special_c3_n01"));
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_miiswordsman_special_c3_n01"));
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_miiswordsman_special_c3_n02"));
    }
    frame(lua_state, 47.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_bomb_ll"));
    }
}

#[acmd_script( agent = "miiswordsman", script = "game_specialairn3endmaxturn" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_air_n3_end_max_turn_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let end_frame = MotionModule::end_frame(boma);
    if is_excute(fighter) {
        SET_SPEED_EX(fighter, -0.5, 1.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        REVERSE_LR(fighter);
    }
    for _ in 0..2 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 0.8, 90, 100, 1, 0, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 0.8, 285, 100, 10, 0, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 2.0);
    }
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 0.8, 120, 100, 20, 0, 4.0, 0.0, 8.2, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 0.8, 45, 100, 25, 0, 4.0, 0.0, -1.2, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 10.0, false);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        let addSpeed = smash::phx::Vector3f { x: 0.0, y: 2.0, z: 0.0 };
        KineticModule::add_speed(boma, &addSpeed);
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 4.0);
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 1.0);
    }
    frame(lua_state, 43.0);
    if is_excute(fighter) {
        //ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 40, 110, 0, 58, 13.0, 0.0, 10.0, 9.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 8.0, 40, 110, 0, 58, 4.0, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        let addSpeed = smash::phx::Vector3f { x: 0.0, y: 2.0, z: 0.0 };
        KineticModule::add_speed(boma, &addSpeed);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 40, 110, 0, 58, 13.0, 0.0, 9.0, -15.0, Some(0.0), Some(8.5), Some(-15.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "miiswordsman", script = "effect_specialairn3endmaxturn" , category = ACMD_EFFECT , low_priority)]
unsafe fn miiswordsman_special_air_n3_end_max_turn_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1.0, true);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_WORK_arg29(fighter, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD, 3, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 0.2, 0.0, true, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_Y, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 2.0, 0.2);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("miiswordsman_rapid_slash_wind_s"), Hash40::new("top"), -0.0, 5.5, -12, 0, 180, 0, 1, true);
	    EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_wind_s"), false, false);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 43.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("miiswordsman_rapid_slash_arc"), Hash40::new("top"), -3, 8, -3, 250, 80, 10, 1.2, true);
	    LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
	    //EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("haver"), -0.0, 12, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        EFFECT(fighter, Hash40::new("sys_bomb_c"), Hash40::new("top"), 0, 0, -13, 0, 180, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 55.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 2);
    }
    frame(lua_state, 74.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
}

#[acmd_script( agent = "miiswordsman", script = "sound_specialairn3endmaxturn" , category = ACMD_SOUND , low_priority)]
unsafe fn miiswordsman_special_air_n3_end_max_turn_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let end_frame = MotionModule::end_frame(boma);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_miiswordsman_special_s01"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_miiswordsman_rnd_special_c3_n01"));
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_miiswordsman_special_c3_n01"));
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_miiswordsman_special_c3_n02"));
    }
    frame(lua_state, 47.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_bomb_ll"));
    }
}



// ==================================================================================================
// ======================================== AIRBORNE ASSAULT ========================================
// ==================================================================================================

#[acmd_script( agent = "miiswordsman", script = "game_specials1start" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_s1_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let end_frame = MotionModule::end_frame(boma);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.4);
    }
    
}

#[acmd_script( agent = "miiswordsman", script = "game_specialairs1start" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_air_s1_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let end_frame = MotionModule::end_frame(boma);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.4);
    }
    
}

#[acmd_script( agent = "miiswordsman", script = "game_specials1" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_s1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {

    }
    
}

#[acmd_script( agent = "miiswordsman", script = "game_specialairs1" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_air_s1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {

    }
    
}
/*
#[acmd_script( agent = "miiswordsman", script = "game_specials1hit" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_s1_hit_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 12.5, 44, 75, 0, 56, 4.5, 0.0, 2.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 12.5, 44, 75, 0, 56, 4.5, 0.0, 6.5, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 12.5, 44, 75, 0, 56, 4.5, 0.0, 11.5, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}
*/

// =============================================================================================
// ======================================== GALE STAB ==========================================
// =============================================================================================

#[acmd_script( agent = "miiswordsman", script = "game_specials2dash" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_s2_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 10.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 0.5);
    }
}

#[acmd_script( agent = "miiswordsman", script = "game_specials2attack" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_s2_attack_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 55, 88, 0, 55, 5.0, 0.0, 5.5, 16.0, Some(0.0), Some(5.5), Some(14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 55, 88, 0, 55, 3.5, 0.0, 5.5, 15.0, Some(0.0), Some(5.5), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_ATTACK_END);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "miiswordsman", script = "effect_specials2attack" , category = ACMD_EFFECT , low_priority)]
unsafe fn miiswordsman_special_s2_attack_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        // Vanilla Effects
        //EFFECT_FOLLOW_WORK(fighter, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        //LAST_EFFECT_SET_COLOR(fighter, 1, 0.72, 1.1);
        
        //EFFECT_FOLLOW(fighter, Hash40::new("miiswordsman_blastwind_stab"), Hash40::new("top"), -0.0, 5, 12, 0, 0, 0, 1.1, true);
        //EFFECT_DETACH_KIND(fighter, Hash40::new("miiswordsman_blastwind_stab"), -1);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 11.4, 0, 0, 0, 0, 2, true);
        EFFECT_DETACH_KIND(fighter, Hash40::new("sys_smash_flash_s"), -1);
        EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 5, 7, 0, 0, 0, 1, 0, 0, 10, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.511, 0.264, 1.3);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 5, -3, 0, 0, 0, 0.8, 0, 0, 10, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.511, 0.264, 1.3);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        

        // Sword Trail
        //AFTER_IMAGE4_ON_WORK_arg29(fighter, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD, 5, Hash40::new("haver"), 0, 0.2, 0, Hash40::new("haver"), -0.0, 10.8, 0, true, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("miiswordsman_counter_arc"), Hash40::new("top"), -1, 7, 7.5, 0.0, -180.0, 25.3, 1.1, true);
	    LAST_EFFECT_SET_RATE(fighter, 2.5);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        //EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0.0, 10, 6, -10.6, -159.5, 55.3, 1.1, true);
	    //LAST_EFFECT_SET_RATE(fighter, 1.4);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 9.25, 0, 0, 0, 180, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 5);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        //EFFECT_FOLLOW_WORK(fighter, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        //LAST_EFFECT_SET_COLOR(fighter, 2.5, 2.5, 2.5);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        //EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 6, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        //EFFECT_OFF_KIND_WORK(fighter, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, false, false);
    }
}

#[acmd_script( agent = "miiswordsman", script = "game_specialairs2dash" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_air_s2_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 10.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 0.5);
    }
}

#[acmd_script( agent = "miiswordsman", script = "game_specialairs2attack" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_air_s2_attack_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 55, 88, 0, 55, 5.0, 0.0, 5.5, 16.0, Some(0.0), Some(5.5), Some(14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 55, 88, 0, 55, 3.5, 0.0, 5.5, 15.0, Some(0.0), Some(5.5), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_ATTACK_END);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "miiswordsman", script = "effect_specialairs2attack" , category = ACMD_EFFECT , low_priority)]
unsafe fn miiswordsman_special_air_s2_attack_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        // Vanilla Effects
        //EFFECT_FOLLOW_WORK(fighter, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        //LAST_EFFECT_SET_COLOR(fighter, 1, 0.72, 1.1);
        
    
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 11.4, 0, 0, 0, 0, 2, true);
        EFFECT_DETACH_KIND(fighter, Hash40::new("sys_smash_flash_s"), -1);
        EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 5, 7, 0, 0, 0, 1, 0, 0, 10, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.511, 0.264, 1.3);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 5, -3, 0, 0, 0, 0.8, 0, 0, 10, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.511, 0.264, 1.3);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        

        // Sword Trail
        //AFTER_IMAGE4_ON_WORK_arg29(fighter, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD, 5, Hash40::new("haver"), 0, 0.2, 0, Hash40::new("haver"), -0.0, 10.8, 0, true, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);

    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("miiswordsman_counter_arc"), Hash40::new("top"), -1, 7, 7.5, 0.0, -180.0, 25.3, 1.1, true);
	    LAST_EFFECT_SET_RATE(fighter, 2.5);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        //EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0.0, 10, 6, -10.6, -159.5, 55.3, 1.1, true);
	    //LAST_EFFECT_SET_RATE(fighter, 1.4);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 9.25, 0, 0, 0, 180, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 5);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        //EFFECT_FOLLOW_WORK(fighter, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        //LAST_EFFECT_SET_COLOR(fighter, 2.5, 2.5, 2.5);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        //EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 6, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        //EFFECT_OFF_KIND_WORK(fighter, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, false, false);
    }
}


// =============================================================================================
// ======================================== CHAKRAM ============================================
// =============================================================================================

#[acmd_script( agent = "miiswordsman", script = "game_specials3_1" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_s3_1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let end_frame = MotionModule::end_frame(boma);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.27);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_REQUEST_GENERATE);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_CHECK_MOTION_HI_LW);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
            WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
        else {
            WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK){
            FT_MOTION_RATE(fighter, 1.25);
        }
        else{
            FT_MOTION_RATE(fighter, 1.1);
        }
        ArticleModule::shoot_exist(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_CHAKRAM, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }  
}

#[acmd_script( agent = "miiswordsman", script = "game_specials3_1hi" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_s3_1_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let end_frame = MotionModule::end_frame(boma);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.27);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_REQUEST_GENERATE);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_CHECK_MOTION_HI_LW);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
            WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
        else {
            WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK){
            FT_MOTION_RATE(fighter, 1.25);
        }
        else{
            FT_MOTION_RATE(fighter, 1.1);
        }
        ArticleModule::shoot_exist(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_CHAKRAM, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }     
}

#[acmd_script( agent = "miiswordsman", script = "game_specials3_1lw" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_s3_1_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let end_frame = MotionModule::end_frame(boma);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.27);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_REQUEST_GENERATE);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_CHECK_MOTION_HI_LW);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
            WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
        else {
            WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK){
            FT_MOTION_RATE(fighter, 1.25);
        }
        else{
            FT_MOTION_RATE(fighter, 1.1);
        }
        ArticleModule::shoot_exist(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_CHAKRAM, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }    
}

#[acmd_script( agent = "miiswordsman", script = "game_specialairs3_1" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_air_s3_1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let end_frame = MotionModule::end_frame(boma);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.27);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_REQUEST_GENERATE);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_CHECK_MOTION_HI_LW);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
            WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
        else {
            WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK){
            FT_MOTION_RATE(fighter, 1.25);
        }
        else{
            FT_MOTION_RATE(fighter, 1.1);
        }
        ArticleModule::shoot_exist(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_CHAKRAM, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }     
}

#[acmd_script( agent = "miiswordsman", script = "game_specialairs3_1hi" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_air_s3_1_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let end_frame = MotionModule::end_frame(boma);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.27);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_REQUEST_GENERATE);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_CHECK_MOTION_HI_LW);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
            WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
        else {
            WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK){
            FT_MOTION_RATE(fighter, 1.25);
        }
        else{
            FT_MOTION_RATE(fighter, 1.1);
        }
        ArticleModule::shoot_exist(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_CHAKRAM, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }    
}

#[acmd_script( agent = "miiswordsman", script = "game_specialairs3_1lw" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_air_s3_1_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let end_frame = MotionModule::end_frame(boma);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.27);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_REQUEST_GENERATE);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_CHECK_MOTION_HI_LW);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
            WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
        else {
            WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK){
            FT_MOTION_RATE(fighter, 1.25);
        }
        else{
            FT_MOTION_RATE(fighter, 1.1);
        }
        ArticleModule::shoot_exist(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_CHAKRAM, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }     
}


// =============================================================================================
// ======================================== STONE SCABBARD ========================================
// =============================================================================================

#[acmd_script( agent = "miiswordsman", script = "game_specialhi1start" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_hi1_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.7);
    }
}

#[acmd_script( agent = "miiswordsman", script = "game_specialairhi1" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_air_hi1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        let addSpeed = smash::phx::Vector3f { x: 0.0, y: 0.5, z: 0.0 };
        KineticModule::add_speed(boma, &addSpeed);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 46, 95, 0, 60, 4.0, 0.0, 7.0, 6.0, Some(0.0), Some(4.0), Some(6.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 46, 95, 0, 60, 3.5, 0.0, 7.0, 10.0, Some(0.0), Some(4.0), Some(10.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 46, 95, 0, 60, 3.5, 0.0, 7.0, 14.0, Some(0.0), Some(4.0), Some(14.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 92, 100, 60, 0, 4.0, 0.0, 7.0, 6.0, Some(0.0), Some(4.0), Some(6.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 92, 100, 60, 0, 3.5, 0.0, 7.0, 10.0, Some(0.0), Some(4.0), Some(10.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 92, 100, 60, 0, 3.5, 0.0, 7.0, 14.0, Some(0.0), Some(4.0), Some(14.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 7.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 7.0, false);
        AttackModule::set_add_reaction_frame(boma, 2, 7.0, false);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        // Tip
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 3.0, 268, 180, 30, 0, 5.0, 0.0, 14.5, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 3.0, 361, 100, 20, 0, 5.0, 0.0, 14.5, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // Middle
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 3.0, 268, 180, 30, 0, 5.0, 0.0, 7.25, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("haver"), 3.0, 361, 100, 20, 0, 5.0, 0.0, 7.25, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // Hilt
        ATTACK(fighter, 4, 0, Hash40::new("haver"), 3.0, 268, 180, 30, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 5, 0, Hash40::new("haver"), 3.0, 361, 100, 20, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_ROKET_UNDER_DISABLE_CONTROL_X);
    }
    frame(lua_state, 60.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "miiswordsman", script = "game_specialhi1end" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_hi1_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 55, 116, 0, 103, 8.0, 0.0, 6.0, -4.0, Some(0.0), Some(6.0), Some(4.5), 2.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.8);
    }
}


#[acmd_script( agent = "miiswordsman", script = "game_specialairhi1end" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_air_hi1_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 55, 116, 0, 103, 8.0, 0.0, 6.0, -4.0, Some(0.0), Some(6.0), Some(4.5), 2.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.8);
    } 
}


// ====================================================================================================
// ======================================== SKYWARD SLASH DASH ========================================
// ====================================================================================================

#[acmd_script( agent = "miiswordsman", script = "game_specialhi2hold" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_hi2_hold_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        //VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        FT_MOTION_RATE(fighter, 0.6);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW){
            //VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
            VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
            FT_MOTION_RATE(fighter, 2.5);
        }
        else{
            FT_MOTION_RATE(fighter, 0.8);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.2, 91, 100, 140, 0, 4.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.2, 100, 100, 80, 0, 10.0, 0.0, 8.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 1.0);
}

#[acmd_script( agent = "miiswordsman", script = "game_specialhi2holdair" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_hi2_hold_air_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        //VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        FT_MOTION_RATE(fighter, 0.6);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW){
            //VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
            VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
            FT_MOTION_RATE(fighter, 2.5);
        }
        else{
            FT_MOTION_RATE(fighter, 0.8);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.2, 91, 100, 140, 0, 4.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.2, 100, 100, 80, 0, 10.0, 0.0, 8.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 1.0);
}

#[acmd_script( agent = "miiswordsman", script = "game_specialhi2" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_hi2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    for _ in 0..5 {
        frame(lua_state, 2.0);
        if is_excute(fighter) {
            // Charged
            //if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
                ATTACK(fighter, 0, 0, Hash40::new("rot"), 3.0, 367, 40, 0, 40, 8.0, 0.0, 0.0, 12.0, Some(0.0), Some(0.0), Some(15.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 1, 0, Hash40::new("rot"), 3.0, 55, 40, 0, 40, 8.0, 0.0, 0.0, 12.0, Some(0.0), Some(0.0), Some(15.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 2, 0, Hash40::new("rot"), 3.0, 367, 100, 100, 0, 3.0, 0.0, 10.0, 6.0, Some(0.0), Some(-10.0), Some(6.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 3, 0, Hash40::new("rot"), 3.0, 363, 100, 120, 0, 3.0, 0.0, 10.0, 6.0, Some(0.0), Some(-10.0), Some(6.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
            // Uncharged
            else{
                ATTACK(fighter, 0, 0, Hash40::new("rot"), 1.0, 367, 40, 0, 40, 8.0, 0.0, 0.0, 12.0, Some(0.0), Some(0.0), Some(15.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 1, 0, Hash40::new("rot"), 1.0, 55, 40, 0, 40, 8.0, 0.0, 0.0, 12.0, Some(0.0), Some(0.0), Some(15.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 2, 0, Hash40::new("rot"), 1.0, 367, 100, 100, 0, 3.0, 0.0, 10.0, 6.0, Some(0.0), Some(-10.0), Some(6.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 3, 0, Hash40::new("rot"), 1.0, 363, 100, 120, 0, 3.0, 0.0, 10.0, 6.0, Some(0.0), Some(-10.0), Some(6.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 1.0);
    }
    if is_excute(fighter) {
        
        let h_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) as f32;
        let v_speed = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) as f32;
        //let angle_calc = v_speed / h_speed as f32;
        //println!("Angle: {}", angle_calc);
        
        let mut launch_angle = if v_speed > 0.0 {(v_speed / h_speed.abs()).atan().to_degrees() as u64} else {361 as u64};

        //println!("H speed: {}", h_speed);
        //println!("V speed: {}", v_speed);

        //println!("Launch Angle (pre-calc): {}", launch_angle);

        if h_speed == 0.0 {
            launch_angle = 90 as u64;
        }

        //println!("Launch Angle: {}", launch_angle);
        
        // Charged
        //if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            ATTACK(fighter, 0, 0, Hash40::new("rot"), 6.0, launch_angle, 110, 0, 70, 8.0, 0.0, 0.0, 12.0, Some(0.0), Some(0.0), Some(15.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("rot"), 6.0, launch_angle, 110, 0, 70, 3.0, 0.0, 10.0, 6.0, Some(0.0), Some(-10.0), Some(6.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        // Uncharged
        else{
            ATTACK(fighter, 0, 0, Hash40::new("rot"), 4.0, 79, 40, 0, 90, 8.0, 0.0, 0.0, 12.0, Some(0.0), Some(0.0), Some(15.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("rot"), 4.0, 79, 40, 0, 90, 3.0, 0.0, 10.0, 6.0, Some(0.0), Some(-10.0), Some(6.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "miiswordsman", script = "game_specialhi2landing" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_hi2_landing_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        //if !VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
        if !VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(fighter, 0.8);
        }
    }
}

#[acmd_script( agent = "miiswordsman", script = "game_specialhi2fall" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_hi2_fall_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        //if !VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
        if !VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(fighter, 0.8);
        }
        if VarModule::is_flag(fighter.battle_object, vars::miiswordsman::instance::SKYWARD_SLASH_DASH_HIT) {
            FT_MOTION_RATE(fighter, 0.5);
        }
    }
}


// =============================================================================================
// ======================================== HERO'S SPIN ========================================
// =============================================================================================

#[acmd_script( agent = "miiswordsman", script = "game_specialhi3" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 14.0, 40, 64, 0, 90, 4.5, 1.0, 11.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 13.0, 40, 64, 0, 90, 4.0, 1.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 10.0, 40, 64, 0, 90, 4.0, 1.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        //AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 12.0, 361, 64, 0, 80, 4.5, 1.0, 11.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 11.0, 361, 64, 0, 80, 4.0, 1.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 8.0, 361, 64, 0, 80, 4.0, 1.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        //AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 9.0, 361, 64, 0, 75, 4.5, 1.0, 11.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 9.0, 361, 64, 0, 75, 4.0, 1.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 7.0, 361, 64, 0, 75, 4.0, 1.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        //AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 7.0, 361, 64, 0, 70, 4.5, 1.0, 11.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 7.0, 361, 64, 0, 70, 4.0, 1.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 5.0, 361, 64, 0, 70, 4.0, 1.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "miiswordsman", script = "game_specialairhi3" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_air_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 367, 100, 115, 0, 6.0, 0.0, 9.0, 7.0, Some(0.0), Some(6.0), Some(14.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        JostleModule::set_status(boma, false);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 60, 100, 80, 0, 6.0, 0.0, 10.0, -14.0, Some(0.0), Some(10.0), Some(-9.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 367, 100, 90, 0, 6.5, 0.0, 8.0, 9.0, Some(0.0), Some(7.0), Some(14.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 60, 100, 90, 0, 6.5, 0.0, 5.0, -15.0, Some(0.0), Some(6.0), Some(-11.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 85, 100, 75, 0, 6.0, 0.0, 8.0, 9.0, Some(0.0), Some(6.0), Some(14.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 60, 100, 90, 0, 6.5, 0.0, 8.0, -13.0, Some(0.0), Some(8.0), Some(-10.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 75, 95, 60, 0, 6.3, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(15.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 60, 100, 100, 0, 6.0, 0.0, 9.0, -14.0, Some(0.0), Some(9.0), Some(-10.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 47.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 208, 0, 30, 7.5, 0.0, 14.0, 4.0, Some(0.0), Some(18.0), Some(11.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 48.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 208, 0, 30, 7.5, 0.0, 15.0, 3.0, Some(0.0), Some(19.0), Some(7.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        JostleModule::set_status(boma, true);
    }
    
}


// ===============================================================================================
// ======================================== KINESIS BLADE ========================================
// ===============================================================================================

// Kinesis Blade - Charge Storage
#[acmd_script( agent = "miiswordsman", script = "game_speciallw1hit" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_lw1_hit_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        let current_level = VarModule::get_int(fighter.battle_object, vars::miiswordsman::instance::SPECIAL_LW1_CHARGE_LEVEL);
        // Attack transition
        if VarModule::is_flag(fighter.battle_object, vars::miiswordsman::status::SPECIAL_LW1_ATTACK_TRIGGER) {
            VarModule::off_flag(fighter.battle_object, vars::miiswordsman::status::SPECIAL_LW1_ATTACK_TRIGGER);
            if current_level > 0{
                if current_level == 1 {
                    // println!("Changing to level 1");
                    MotionModule::change_motion(boma, Hash40::new("special_lw1_hit_lv1"), 0.0, 1.0, false, 0.0, false, false);
                }
                else{
                    // println!("Changing to level 2");
                    MotionModule::change_motion(boma, Hash40::new("special_lw1_hit_lv2"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
        }
        else{
            // Increment the level
            if current_level < 2 {
                VarModule::set_int(fighter.battle_object, vars::miiswordsman::instance::SPECIAL_LW1_CHARGE_LEVEL, current_level + 1); // Add a charge level
                // println!("Kinesis increment");
            }
            // println!("Kinesis Level: {}", VarModule::get_int(fighter.battle_object, vars::miiswordsman::instance::SPECIAL_LW1_CHARGE_LEVEL));
        }
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_ON);
        //FT_MOTION_RATE(fighter, 0.65);
    }
}

#[acmd_script( agent = "miiswordsman", script = "game_specialairlw1hit" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_air_lw1_hit_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        let current_level = VarModule::get_int(fighter.battle_object, vars::miiswordsman::instance::SPECIAL_LW1_CHARGE_LEVEL);
        // Attack transition
        if VarModule::is_flag(fighter.battle_object, vars::miiswordsman::status::SPECIAL_LW1_ATTACK_TRIGGER) {
            VarModule::off_flag(fighter.battle_object, vars::miiswordsman::status::SPECIAL_LW1_ATTACK_TRIGGER);
            if current_level > 0{
                if current_level == 1 {
                    // println!("Changing to level 1");
                    MotionModule::change_motion(boma, Hash40::new("special_air_lw1_hit_lv1"), 0.0, 1.0, false, 0.0, false, false);
                }
                else{
                    // println!("Changing to level 2");
                    MotionModule::change_motion(boma, Hash40::new("special_air_lw1_hit_lv2"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
        }
        else{
            // Increment the level
            if current_level < 2 {
                VarModule::set_int(fighter.battle_object, vars::miiswordsman::instance::SPECIAL_LW1_CHARGE_LEVEL, current_level + 1); // Add a charge level
                // println!("Kinesis increment");
            }
            // println!("Kinesis Level: {}", VarModule::get_int(fighter.battle_object, vars::miiswordsman::instance::SPECIAL_LW1_CHARGE_LEVEL));
        }
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_ON);
        //FT_MOTION_RATE(fighter, 0.65);
    }
}


// Kinesis Blade - 1 Charge
#[acmd_script( agent = "miiswordsman", script = "game_speciallw1hitlv1" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_lw1_hit_lv1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VarModule::set_int(fighter.battle_object, vars::miiswordsman::instance::SPECIAL_LW1_CHARGE_LEVEL, 0); // Reset charge level
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_ON);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 1, Hash40::new("haver"), 2.0, 365, 0, 0, 50, 4.0, 0.0, 11.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 1, Hash40::new("haver"), 2.0, 365, 0, 0, 50, 4.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 1, Hash40::new("haver"), 2.0, 365, 0, 0, 50, 4.0, 0.0, 11.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 1, Hash40::new("haver"), 2.0, 365, 0, 0, 50, 4.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 7.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 80, 0, 50, 6.0, 0.0, 6.0, 10.0, Some(0.0), Some(6.0), Some(-30.0), 1.2, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        //WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
}

#[acmd_script( agent = "miiswordsman", script = "game_specialairlw1hitlv1" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_air_lw1_hit_lv1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VarModule::set_int(fighter.battle_object, vars::miiswordsman::instance::SPECIAL_LW1_CHARGE_LEVEL, 0); // Reset charge level
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_ON);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 0.0, 0, 0, 0, 0, 4.0, 0.0, 11.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 0.0, 0, 0, 0, 0, 4.0, 0.0, 2.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 1, Hash40::new("haver"), 2.0, 365, 0, 0, 50, 4.0, 0.0, 11.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 1, Hash40::new("haver"), 2.0, 365, 0, 0, 50, 4.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 7.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 80, 0, 50, 6.0, 0.0, 6.0, 10.0, Some(0.0), Some(6.0), Some(-30.0), 1.2, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        //WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
}

// Kinesis Blade - 2 Charges
#[acmd_script( agent = "miiswordsman", script = "game_speciallw1hitlv2" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_lw1_hit_lv2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 0.1, 365, 0, 0, 40, 12.0, 0.0, 12.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
    }
    frame(lua_state, 60.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 365, 0, 0, 40, 12.0, 0.0, 12.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
    }
    frame(lua_state, 63.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 68.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 5.0, 361, 100, 0, 50, 4.0, 0.0, 11.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 5.0, 361, 100, 0, 50, 4.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 73.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
    frame(lua_state, 76.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "miiswordsman", script = "game_specialairlw1hitlv2" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_air_lw1_hit_lv2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 0.1, 365, 0, 0, 40, 12.0, 0.0, 12.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
    }
    frame(lua_state, 60.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 365, 0, 0, 40, 12.0, 0.0, 12.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
    }
    frame(lua_state, 63.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 68.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 5.0, 361, 100, 0, 50, 4.0, 0.0, 11.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 5.0, 361, 100, 0, 50, 4.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 73.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
    frame(lua_state, 76.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}


// ================================================================================================================
// ======================================== DEFLECTING DRAFT / SHOCK SPELL ========================================
// ================================================================================================================

#[acmd_script( agent = "miiswordsman", script = "game_speciallw2" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_lw2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.3);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 3.0);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_REVERSE_SLASH_FLAG_SPECIAL_FALL);
    FT_MOTION_RATE(fighter, 1.2);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 60, 55, 0, 40, 5.0, 0.0, 9.0, 25.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 60, 55, 0, 40, 4.0, 0.0, 9.0, 24.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_REVERSE_SLASH_FLAG_SPECIAL_FALL);
    }
    
}

#[acmd_script( agent = "miiswordsman", script = "effect_speciallw2" , category = ACMD_EFFECT , low_priority)]
unsafe fn miiswordsman_special_lw2_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), -1.5, 0, 2, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        EFFECT_FOLLOW(fighter, Hash40::new("miiswordsman_reflect_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0.0, 0, 1, true);
        //EFFECT_FOLLOW_WORK(fighter, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.65);
    }
    frame(lua_state, 10.5);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_thunder"), Hash40::new("arml"), 4.0, 0, 0.0, 0, 0.0, 0, 0.1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_thunder_flash"), Hash40::new("arml"), 4.0, 0, 0.0, 0, 0.0, 0, 0.3, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0.5, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
    EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_elec"), Hash40::new("top"), 0, 8.0, 25.0, 0, 0.0, 0, 0.4, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.84, 0.17);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_reflect_sword"), false, true);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        //EFFECT_OFF_KIND_WORK(fighter, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, false, false);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("miiswordsman_reflect1"), -1);
    }
    
}

#[acmd_script( agent = "miiswordsman", script = "game_specialairlw2" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_air_lw2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.3);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 3.0);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_REVERSE_SLASH_FLAG_SPECIAL_FALL);
    FT_MOTION_RATE(fighter, 1.2);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 60, 55, 0, 40, 5.0, 0.0, 9.0, 25.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 60, 55, 0, 40, 4.0, 0.0, 9.0, 24.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_REVERSE_SLASH_FLAG_SPECIAL_FALL);
    }
    
}

#[acmd_script( agent = "miiswordsman", script = "effect_specialairlw2" , category = ACMD_EFFECT , low_priority)]
unsafe fn miiswordsman_special_air_lw2_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), -1.5, 0, 2, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        EFFECT_FOLLOW(fighter, Hash40::new("miiswordsman_reflect_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0.0, 0, 1, true);
        //EFFECT_FOLLOW_WORK(fighter, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.65);
    }
    frame(lua_state, 10.5);
    if is_excute(fighter) {
    EFFECT_FOLLOW(fighter, Hash40::new("sys_thunder"), Hash40::new("arml"), 4.0, 0, 0.0, 0, 0.0, 0, 0.1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_thunder_flash"), Hash40::new("arml"), 4.0, 0, 0.0, 0, 0.0, 0, 0.3, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0.5, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
    EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_elec"), Hash40::new("top"), 0, 8.0, 25.0, 0, 0.0, 0, 0.4, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.84, 0.17);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miiswordsman_reflect_sword"), false, true);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        //EFFECT_OFF_KIND_WORK(fighter, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, false, false);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("miiswordsman_reflect1"), -1);
    }
    
}

// ================================================================================================================
// ======================================== HURRICANE HEAVE =======================================================
// ================================================================================================================

#[acmd_script( agent = "miiswordsman", script = "game_speciallw3" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_lw3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 12.0, /*Angle*/ 35, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 77, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 3.8, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 15.0, /*Angle*/ 35, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 2.7, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 35, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 77, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 8.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(8.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 35, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 2.7, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 13.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(19.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_WALL_CHECK);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 42, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 8.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(8.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 13.0, /*Angle*/ 42, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 2.7, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 13.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(19.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.5, /*Angle*/ 50, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 58, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 8.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(8.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 50, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 2.7, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 13.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(19.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 8.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "miiswordsman", script = "game_speciallw3end" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_lw3_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {

    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        //ATTACK(fighter, 0, 1, Hash40::new("haver"), 1.0, 100, 10, 0, 10, 4.8, 0.0, 0.0, 0.0, Some(0.0), Some(13.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 1, Hash40::new("top"), 0.1, 368, 30, 0, 0, 7.5, 0.0, 6.0, 15.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        let hit1 = Vector2f {x: 4.0, y: 5.0};
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 10, false);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        //AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 1, Hash40::new("top"), 0.1, 368, 30, 0, 0, 7.5, 0.0, 6.0, 15.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        let hit1 = Vector2f {x: 0.0, y: 17.0};
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 4, false);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        //REVERSE_LR(fighter);
        //AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 1, Hash40::new("top"), 0.1, 368, 30, 0, 0, 7.5, 0.0, 17.0, 0.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        let hit1 = Vector2f {x: -6.0, y: 15.0};
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 3, false);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        //REVERSE_LR(fighter);
        //AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 1, Hash40::new("top"), 0.1, 368, 30, 0, 0, 7.5, 0.0, 15.0, -6.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        let hit1 = Vector2f {x: -12.0, y: 0.0};
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 2, false);
    }
    /*
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 1, Hash40::new("haver"), 0.1, 365, 30, 0, 0, 6.8, 0.0, 0.0, 0.0, Some(0.0), Some(13.0), Some(0.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        //let hit1 = Vector2f {x: 12.0, y: 0.0};
        //AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 10, false);
    }
    */
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 2, Hash40::new("haver"), 5.0, 80, 100, 0, 60, 6.8, 0.0, 0.0, 0.0, Some(0.0), Some(11.5), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "miiswordsman", script = "game_specialairlw3" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_air_lw3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_WALL_CHECK);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 275, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 44, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ -2.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-2.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ -2.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-2.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 13.0, /*Angle*/ 275, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 44, /*Size*/ 3.2, /*X*/ 0.0, /*Y*/ -7.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(-11.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 13.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.2, /*X*/ 0.0, /*Y*/ -7.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(-11.0), /*Z2*/ Some(5.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ -2.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-2.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ -2.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-2.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.2, /*X*/ 0.0, /*Y*/ -7.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(-11.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.2, /*X*/ 0.0, /*Y*/ -7.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(-11.0), /*Z2*/ Some(5.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.5, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ -2.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-2.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.5, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ -2.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-2.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.2, /*X*/ 0.0, /*Y*/ -7.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(-11.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.2, /*X*/ 0.0, /*Y*/ -7.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(-11.0), /*Z2*/ Some(5.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 8.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "miiswordsman", script = "game_specialairlw3end" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_air_lw3_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.790);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_SP_BRAKE);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 45, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ -10.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.0), /*Z2*/ Some(7.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 45, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ -10.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.0), /*Z2*/ Some(7.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_SP_BRAKE);
    }
}

#[acmd_script( agent = "miiswordsman", script = "game_specialairlw3endair" , category = ACMD_GAME , low_priority)]
unsafe fn miiswordsman_special_air_lw3_end_air_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        //ATTACK(fighter, 0, 1, Hash40::new("haver"), 1.0, 100, 10, 0, 10, 4.8, 0.0, 0.0, 0.0, Some(0.0), Some(13.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 1, Hash40::new("top"), 0.1, 368, 30, 0, 0, 7.5, 0.0, 6.0, 15.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        let hit1 = Vector2f {x: 4.0, y: 5.0};
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 10, false);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        //AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 1, Hash40::new("top"), 0.1, 368, 30, 0, 0, 7.5, 0.0, 6.0, 15.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        let hit1 = Vector2f {x: 0.0, y: 17.0};
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 4, false);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        //REVERSE_LR(fighter);
        //AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 1, Hash40::new("top"), 0.1, 368, 30, 0, 0, 7.5, 0.0, 17.0, 0.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        let hit1 = Vector2f {x: -6.0, y: 15.0};
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 3, false);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        //REVERSE_LR(fighter);
        //AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 1, Hash40::new("top"), 0.1, 368, 30, 0, 0, 7.5, 0.0, 15.0, -6.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        let hit1 = Vector2f {x: -12.0, y: 0.0};
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 2, false);
    }
    /*
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 1, Hash40::new("haver"), 0.1, 365, 30, 0, 0, 6.8, 0.0, 0.0, 0.0, Some(0.0), Some(13.0), Some(0.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        //let hit1 = Vector2f {x: 12.0, y: 0.0};
        //AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 10, false);
    }
    */
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        REVERSE_LR(fighter);
        ATTACK(fighter, 0, 2, Hash40::new("haver"), 5.0, 80, 100, 0, 60, 6.8, 0.0, 0.0, 0.0, Some(0.0), Some(11.5), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

pub fn install() {
    install_acmd_scripts!(
        miiswordsman_special_n1_game,
        //miiswordsman_special_n1_effect,
        miiswordsman_special_air_n1_game,
        //miiswordsman_special_air_n1_effect,
        miiswordsman_special_n2_game,
        miiswordsman_special_n2_effect,
        miiswordsman_special_n2_sound,
        miiswordsman_special_air_n2_game,
        miiswordsman_special_air_n2_effect,
        miiswordsman_special_air_n2_sound,
        //miiswordsman_special_n3_end_game,
        //miiswordsman_special_n3_end_effect,
        //miiswordsman_special_n3_end_sound,
        //miiswordsman_special_n3_end_turn_game,
        //miiswordsman_special_n3_end_turn_effect,
        //miiswordsman_special_n3_end_turn_sound,
        //miiswordsman_special_n3_end_max_game,
        //miiswordsman_special_n3_end_max_effect,
        //miiswordsman_special_n3_end_max_sound,
        //miiswordsman_special_n3_end_max_turn_game,
        //miiswordsman_special_n3_end_max_turn_effect,
        //miiswordsman_special_n3_end_max_turn_sound,
        //miiswordsman_special_air_n3_end_game,
        //miiswordsman_special_air_n3_end_effect,
        //miiswordsman_special_air_n3_end_sound,
        //miiswordsman_special_air_n3_end_turn_game,
        //miiswordsman_special_air_n3_end_turn_effect,
        //miiswordsman_special_air_n3_end_turn_sound,
        //miiswordsman_special_air_n3_end_max_game,
        //miiswordsman_special_air_n3_end_max_effect,
        //miiswordsman_special_air_n3_end_max_sound,
        //miiswordsman_special_air_n3_end_max_turn_game,
        //miiswordsman_special_air_n3_end_max_turn_effect,
        //miiswordsman_special_air_n3_end_max_turn_sound,

        miiswordsman_special_s1_start_game,
        miiswordsman_special_air_s1_start_game,
        miiswordsman_special_s1_game,
        miiswordsman_special_air_s1_game,
        //miiswordsman_special_s1_hit_game,
        miiswordsman_special_s2_dash_game,
        miiswordsman_special_s2_attack_game,
        miiswordsman_special_s2_attack_effect,
        miiswordsman_special_air_s2_dash_game,
        miiswordsman_special_air_s2_attack_game,
        miiswordsman_special_air_s2_attack_effect,
        miiswordsman_special_s3_1_game,
        miiswordsman_special_s3_1_hi_game,
        miiswordsman_special_s3_1_lw_game,
        miiswordsman_special_air_s3_1_game,
        miiswordsman_special_air_s3_1_hi_game,
        miiswordsman_special_air_s3_1_lw_game,

        miiswordsman_special_hi1_start_game,
        miiswordsman_special_air_hi1_game,
        miiswordsman_special_hi1_end_game,
        miiswordsman_special_air_hi1_end_game,
        miiswordsman_special_hi2_hold_game,
        miiswordsman_special_hi2_hold_air_game,
        miiswordsman_special_hi2_landing_game,
        miiswordsman_special_hi2_fall_game,
        miiswordsman_special_hi2_game,
        miiswordsman_special_hi3_game,
        miiswordsman_special_air_hi3_game,

        //miiswordsman_special_lw1_hit_game,
        //miiswordsman_special_air_lw1_hit_game,
        //miiswordsman_special_lw1_hit_lv1_game,
        //miiswordsman_special_air_lw1_hit_lv1_game,
        //miiswordsman_special_lw1_hit_lv2_game,
        //miiswordsman_special_air_lw1_hit_lv2_game,
        miiswordsman_special_lw2_game,
        miiswordsman_special_lw2_effect,
        miiswordsman_special_air_lw2_game,
        miiswordsman_special_air_lw2_effect,
        miiswordsman_special_air_lw2_game,
        miiswordsman_special_lw3_game,
        //miiswordsman_special_lw3_end_game,
        miiswordsman_special_air_lw3_game,
        miiswordsman_special_air_lw3_end_game,
        //miiswordsman_special_air_lw3_end_air_game,
    );
}

